use std::{error::Error, sync::Arc};

use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::{Block as BlockT, HashFor};

use sc_chain_spec::ChainSpec;
use sc_client_api::{backend::StateBackend, AuxStore, Backend};
use sc_consensus_babe::{Config, Epoch};
use sc_consensus_babe_rpc::{BabeApi, BabeRpcHandler};
use sc_consensus_epochs::SharedEpochChanges;
use sc_finality_grandpa::{
	FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
use sc_finality_grandpa_rpc::{GrandpaApi, GrandpaRpcHandler};
use sc_rpc::{Metadata, SubscriptionTaskExecutor};
use sc_rpc_api::DenyUnsafe;
use sc_sync_state_rpc::{SyncStateRpcApi, SyncStateRpcHandler};
use sc_transaction_pool_api::TransactionPool;

use substrate_frame_rpc_system::{AccountNonceApi, FullSystem, SystemApi};

use beefy_gadget::notification::{BeefyBestBlockStream, BeefySignedCommitmentStream};
use beefy_gadget_rpc::{BeefyApi, BeefyRpcHandler};
use pallet_mmr_rpc::{Mmr, MmrApi, MmrRuntimeApi};
use pallet_transaction_payment_rpc::{
	TransactionPayment, TransactionPaymentApi, TransactionPaymentRuntimeApi,
};

use myriad_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Hash, Index};

use jsonrpc_core::IoHandler;

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: Config,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA.
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Dependencies for BEEFY.
pub struct BeefyDeps {
	/// Receives notifications about signed commitment events from BEEFY.
	pub beefy_commitment_stream: BeefySignedCommitmentStream<Block>,
	/// Receives notifications about best block events from BEEFY.
	pub beefy_best_block_stream: BeefyBestBlockStream<Block>,
	/// Executor to drive the subscription manager in the BEEFY RPC handler.
	pub beefy_subscription_executor: SubscriptionTaskExecutor,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
	/// BEEFY specific dependencies.
	pub beefy: BeefyDeps,
}

/// A IO handler that uses all RPC extensions.
pub type IoHandlerRpcExtension = IoHandler<Metadata>;

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B>(
	deps: FullDeps<C, P, SC, B>,
) -> Result<IoHandlerRpcExtension, Box<dyn Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Sync
		+ Send
		+ 'static,
	C::Api: BlockBuilder<Block>,
	C::Api: AccountNonceApi<Block, AccountId, Index>,
	C::Api: TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: sp_consensus_babe::BabeApi<Block>,
	C::Api: MmrRuntimeApi<Block, <Block as BlockT>::Hash>,
	P: TransactionPool + 'static,
	SC: SelectChain<Block> + 'static,
	B: Backend<Block> + Send + Sync + 'static,
	B::State: StateBackend<HashFor<Block>>,
{
	let FullDeps { client, pool, select_chain, chain_spec, deny_unsafe, babe, grandpa, beefy } =
		deps;

	let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;

	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	let BeefyDeps { beefy_commitment_stream, beefy_best_block_stream, beefy_subscription_executor } =
		beefy;

	let mut io = IoHandler::default();

	io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe)));
	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));
	io.extend_with(BabeApi::to_delegate(BabeRpcHandler::new(
		client.clone(),
		shared_epoch_changes.clone(),
		keystore,
		babe_config,
		select_chain,
		deny_unsafe,
	)));
	io.extend_with(GrandpaApi::to_delegate(GrandpaRpcHandler::new(
		shared_authority_set.clone(),
		shared_voter_state,
		justification_stream,
		subscription_executor,
		finality_provider,
	)));
	let beefy_handler: BeefyRpcHandler<Block> = BeefyRpcHandler::new(
		beefy_commitment_stream,
		beefy_best_block_stream,
		beefy_subscription_executor,
	)?;
	io.extend_with(BeefyApi::to_delegate(beefy_handler));
	io.extend_with(MmrApi::to_delegate(Mmr::new(client.clone())));
	io.extend_with(SyncStateRpcApi::to_delegate(SyncStateRpcHandler::new(
		chain_spec,
		client,
		shared_authority_set,
		shared_epoch_changes,
	)?));

	Ok(io)
}
