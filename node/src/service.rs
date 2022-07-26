use crate::rpc::{BabeDeps, BeefyDeps, FullDeps, GrandpaDeps, IoHandlerRpcExtension};

use std::{sync::Arc, time::Duration};

use sp_authorship::InherentDataProvider as AuthorshipInherentDataProvider;
use sp_consensus::CanAuthorWithNativeVersion;
use sp_consensus_babe::inherents::InherentDataProvider as BabeInherentDataProvider;
use sp_runtime::traits::Block as BlockT;
use sp_timestamp::InherentDataProvider;
use sp_transaction_storage_proof::registration;

use sc_basic_authorship::ProposerFactory;
use sc_client_api::{BlockBackend, ExecutorProvider};
use sc_consensus::{DefaultImportQueue, LongestChain};
use sc_consensus_babe::{
	self, BabeBlockImport, BabeLink, BabeParams, Config as BabeConfig, SlotProportion,
};
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch, NativeVersion};
use sc_finality_grandpa::{
	warp_proof::NetworkProvider, Config as GrandpaConfig, FinalityProofProvider,
	GrandpaBlockImport, GrandpaParams, LinkHalf as GrandpaLinkHalf, SharedVoterState,
	VotingRulesBuilder,
};
use sc_network::NetworkService;
use sc_rpc::SubscriptionTaskExecutor;
use sc_rpc_api::DenyUnsafe;
use sc_service::{
	config::Configuration, error::Error as ServiceError, BuildNetworkParams, PartialComponents,
	RpcHandlers, SpawnTasksParams, TFullBackend, TFullClient, TaskManager,
};
use sc_telemetry::{Error as TelemetryError, Telemetry, TelemetryWorker};
use sc_transaction_pool::{BasicPool, FullPool};

use beefy_gadget::{
	notification::{
		BeefyBestBlockSender, BeefyBestBlockStream, BeefySignedCommitmentSender,
		BeefySignedCommitmentStream,
	},
	BeefyParams,
};

use myriad_runtime::{opaque::Block, RuntimeApi};

/// The full client type definition.
type FullClient = TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = TFullBackend<Block>;
type FullSelectChain = LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport = GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;
/// The transaction pool type defintion.
type TransactionPool = FullPool<Block, FullClient>;

// Our native executor instance.
pub struct ExecutorDispatch;
/// Result of [`new_full_base`].
pub struct NewFullBase {
	/// The task manager of the node.
	pub task_manager: TaskManager,
	/// The client instance of the node.
	pub client: Arc<FullClient>,
	/// The networking service of the node.
	pub network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	/// The transaction pool of the node.
	pub transaction_pool: Arc<TransactionPool>,
	/// The rpc handlers of the node.
	pub rpc_handlers: RpcHandlers,
}

impl NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		myriad_runtime::api::dispatch(method, data)
	}

	fn native_version() -> NativeVersion {
		myriad_runtime::native_version()
	}
}

/// Creates a new partial node.
#[allow(clippy::type_complexity)]
pub fn new_partial(
	config: &Configuration,
) -> Result<
	PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		DefaultImportQueue<Block, FullClient>,
		FullPool<Block, FullClient>,
		(
			(
				BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
				BabeLink<Block>,
				GrandpaLinkHalf<Block, FullClient, FullSelectChain>,
				(BeefySignedCommitmentSender<Block>, BeefyBestBlockSender<Block>),
			),
			impl Fn(DenyUnsafe, SubscriptionTaskExecutor) -> Result<IoHandlerRpcExtension, ServiceError>,
			SharedVoterState,
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, TelemetryError> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;

	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = LongestChain::new(backend.clone());

	let transaction_pool = BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let justification_import = grandpa_block_import.clone();

	let (block_import, babe_link) = sc_consensus_babe::block_import(
		BabeConfig::get(&*client)?,
		grandpa_block_import,
		client.clone(),
	)?;

	let slot_duration = babe_link.config().slot_duration();

	let import_queue = sc_consensus_babe::import_queue(
		babe_link.clone(),
		block_import.clone(),
		Some(Box::new(justification_import)),
		client.clone(),
		select_chain.clone(),
		move |_, ()| async move {
			let timestamp = InherentDataProvider::from_system_time();

			let slot = BabeInherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);

			let uncles =
				AuthorshipInherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

			Ok((timestamp, slot, uncles))
		},
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
		CanAuthorWithNativeVersion::new(client.executor().clone()),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let (beefy_commitment_link, beefy_commitment_stream) =
		BeefySignedCommitmentStream::<Block>::channel();
	let (beefy_best_block_link, beefy_best_block_stream) = BeefyBestBlockStream::<Block>::channel();
	let beefy_links = (beefy_commitment_link, beefy_best_block_link);

	let import_setup = (block_import, babe_link, grandpa_link, beefy_links);

	let (rpc_extensions_builder, shared_voter_state) = {
		let (_, babe_link, grandpa_link, _) = &import_setup;

		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let keystore = keystore_container.sync_keystore();
		let chain_spec = config.chain_spec.cloned_box();

		let babe_config = babe_link.config().clone();
		let shared_epoch_changes = babe_link.epoch_changes().clone();

		let shared_voter_state_new = SharedVoterState::empty();
		let shared_voter_state = shared_voter_state_new.clone();
		let justification_stream = grandpa_link.justification_stream();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let finality_proof_provider = FinalityProofProvider::new_for_service(
			backend.clone(),
			Some(shared_authority_set.clone()),
		);

		let rpc_extensions_builder =
			move |deny_unsafe, subscription_executor: SubscriptionTaskExecutor| {
				let deps = FullDeps {
					client: client.clone(),
					pool: pool.clone(),
					select_chain: select_chain.clone(),
					chain_spec: chain_spec.cloned_box(),
					deny_unsafe,
					babe: BabeDeps {
						babe_config: babe_config.clone(),
						shared_epoch_changes: shared_epoch_changes.clone(),
						keystore: keystore.clone(),
					},
					grandpa: GrandpaDeps {
						shared_voter_state: shared_voter_state_new.clone(),
						shared_authority_set: shared_authority_set.clone(),
						justification_stream: justification_stream.clone(),
						subscription_executor: subscription_executor.clone(),
						finality_provider: finality_proof_provider.clone(),
					},
					beefy: BeefyDeps {
						beefy_commitment_stream: beefy_commitment_stream.clone(),
						beefy_best_block_stream: beefy_best_block_stream.clone(),
						beefy_subscription_executor: subscription_executor,
					},
				};

				crate::rpc::create_full(deps).map_err(Into::into)
			};

		(rpc_extensions_builder, shared_voter_state)
	};

	Ok(PartialComponents {
		client,
		backend,
		task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (import_setup, rpc_extensions_builder, shared_voter_state, telemetry),
	})
}

/// Creates a full service from the configuration.
pub fn new_full_base(
	mut config: Configuration,
	with_startup_data: impl FnOnce(
		&BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
		&BabeLink<Block>,
	),
) -> Result<NewFullBase, ServiceError> {
	let PartialComponents {
		client,
		backend,
		mut task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (import_setup, rpc_extensions_builder, shared_voter_state, mut telemetry),
	} = new_partial(&config)?;

	let (block_import, babe_link, grandpa_link, beefy_links) = import_setup;

	let grandpa_protocol_name = sc_finality_grandpa::protocol_standard_name(
		&client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
		&config.chain_spec,
	);
	config
		.network
		.extra_sets
		.push(sc_finality_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));

	let beefy_protocol_name = beefy_gadget::protocol_standard_name(
		&client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
		&config.chain_spec,
	);
	config
		.network
		.extra_sets
		.push(beefy_gadget::beefy_peers_set_config(beefy_protocol_name.clone()));

	let warp_sync = Arc::new(NetworkProvider::new(
		backend.clone(),
		grandpa_link.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, network_starter) =
		sc_service::build_network(BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync: Some(warp_sync),
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let name = config.network.node_name.clone();
	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks = Some(BackoffAuthoringOnFinalizedHeadLagging::default());
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();

	let rpc_handlers = sc_service::spawn_tasks(SpawnTasksParams {
		config,
		client: client.clone(),
		backend: backend.clone(),
		task_manager: &mut task_manager,
		keystore: keystore_container.sync_keystore(),
		transaction_pool: transaction_pool.clone(),
		rpc_extensions_builder: Box::new(rpc_extensions_builder),
		network: network.clone(),
		system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;

	(with_startup_data)(&block_import, &babe_link);

	if role.is_authority() {
		let proposer = ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let can_author_with = CanAuthorWithNativeVersion::new(client.executor().clone());

		let client_clone = client.clone();
		let slot_duration = babe_link.config().slot_duration();

		let babe_params = BabeParams {
			keystore: keystore_container.sync_keystore(),
			client: client.clone(),
			select_chain,
			env: proposer,
			block_import,
			sync_oracle: network.clone(),
			justification_sync_link: network.clone(),
			create_inherent_data_providers: move |parent, ()| {
				let client_clone = client_clone.clone();
				async move {
					let uncles = sc_consensus_uncles::create_uncles_inherent_data_provider(
						&*client_clone,
						parent,
					)?;

					let timestamp = InherentDataProvider::from_system_time();

					let slot = BabeInherentDataProvider::from_timestamp_and_slot_duration(
						*timestamp,
						slot_duration,
					);

					let storage_proof = registration::new_data_provider(&*client_clone, &parent)?;

					Ok((timestamp, slot, uncles, storage_proof))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			can_author_with,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		let babe = sc_consensus_babe::start_babe(babe_params)?;

		task_manager
			.spawn_essential_handle()
			.spawn_blocking("babe-proposer", None, babe);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore =
		if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };

	if enable_grandpa {
		let grandpa_config = GrandpaConfig {
			// FIXME #1578 make this available through chainspec
			gossip_duration: Duration::from_millis(333),
			justification_period: 512,
			observer_enabled: false,
			local_role: role,
			name: Some(name),
			keystore: keystore.clone(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			protocol_name: grandpa_protocol_name,
		};

		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_params = GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network: network.clone(),
			voting_rule: VotingRulesBuilder::default().build(),
			prometheus_registry: prometheus_registry.clone(),
			shared_voter_state,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		let grandpa = sc_finality_grandpa::run_grandpa_voter(grandpa_params)?;

		task_manager
			.spawn_essential_handle()
			.spawn_blocking("grandpa-voter", None, grandpa);
	}

	let beefy_params = BeefyParams {
		client: client.clone(),
		backend,
		key_store: keystore.clone(),
		network: network.clone(),
		signed_commitment_sender: beefy_links.0,
		beefy_best_block_sender: beefy_links.1,
		min_block_delta: 8,
		prometheus_registry,
		protocol_name: beefy_protocol_name,
	};

	let beefy = beefy_gadget::start_beefy_gadget::<_, _, _, _>(beefy_params);

	task_manager.spawn_handle().spawn_blocking("beefy-gadget", None, beefy);

	network_starter.start_network();

	Ok(NewFullBase { task_manager, client, network, transaction_pool, rpc_handlers })
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
	new_full_base(config, |_, _| ()).map(|NewFullBase { task_manager, .. }| task_manager)
}
