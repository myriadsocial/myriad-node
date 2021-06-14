use cumulus_primitives_core::ParaId;
use myriad_runtime::{AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use hex_literal::hex;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<myriad_runtime::GenesisConfig, Extensions>;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate authority key
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AuraId
) {
	(
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<AuraId>(seed)
	)
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	properties
}

pub fn staging_testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/staging.json")[..])
}

pub fn development_testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/development.json")[..])
}

pub fn make_staging_testnet_config(id: ParaId) -> ChainSpec {
    let properties = get_properties("MYRIA", 12, 214);

	ChainSpec::from_genesis(
		// Name
		"Myriad Staging Tesnet",
		// ID
		"myriad_staging_tesnet",
		ChainType::Live,
		move || {
			make_staging_testnet_config_genesis(id)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-staging".into()),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-staging".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn make_staging_testnet_config_genesis(id: ParaId) -> myriad_runtime::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV
		hex!["46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"].into(),
	];

	// ./.maintain/prepare-test-net.sh 4
	let initial_authorities: Vec<(
		AccountId,
		AuraId,
	)> = vec![
		(
			//5F4H97f7nQovyrbiq4ZetaaviNwThSVcFobcA5aGab6167dK
			hex!["84617f575372edb5a36d85c04cdf2e4699f96fe33eb5f94a28c041b88e398d0c"].into(),
			//5F4H97f7nQovyrbiq4ZetaaviNwThSVcFobcA5aGab6167dK
			hex!["84617f575372edb5a36d85c04cdf2e4699f96fe33eb5f94a28c041b88e398d0c"].unchecked_into(),
		),
		(
			//5DiDShBWa1fQx6gLzpf3SFBhMinCoyvHM1BWjPNsmXS8hkrW
			hex!["48d7e931307afb4b68d8d565d4c66e00d856c6d65f5fed6bb82dcfb60e936c67"].into(),
			//5DiDShBWa1fQx6gLzpf3SFBhMinCoyvHM1BWjPNsmXS8hkrW
			hex!["48d7e931307afb4b68d8d565d4c66e00d856c6d65f5fed6bb82dcfb60e936c67"].unchecked_into(),
		),
		(
			//5EFb84yH9tpcFuiKUcsmdoF7xeeY3ajG1ZLQimxQoFt9HMKR
			hex!["60c57f0008067cc01c5ff9eb2e2f9b3a94299a915a91198bd1021a6c55596f57"].into(),
			//5EFb84yH9tpcFuiKUcsmdoF7xeeY3ajG1ZLQimxQoFt9HMKR
			hex!["60c57f0008067cc01c5ff9eb2e2f9b3a94299a915a91198bd1021a6c55596f57"].unchecked_into(),
		),
		(
			//5DZLHESsfGrJ5YzT3HuRPXsSNb589xQ4Unubh1mYLodzKdVY
			hex!["4211b79e34ee8072eab506edd4b93a7b85a14c9a05e5cdd056d98e7dbca87730"].into(),
			//5DZLHESsfGrJ5YzT3HuRPXsSNb589xQ4Unubh1mYLodzKdVY
			hex!["4211b79e34ee8072eab506edd4b93a7b85a14c9a05e5cdd056d98e7dbca87730"].unchecked_into(),
		),
	];

	const MYRIA: u128 = 1_000_000_000_000;
	const ENDOWMENT: u128 = 1_000_000 * MYRIA;
	const AUTHOR: u128 = 100 * MYRIA;

    myriad_runtime::GenesisConfig {
		frame_system: myriad_runtime::SystemConfig {
			code: myriad_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: myriad_runtime::BalancesConfig {
			balances: endowed_accounts.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), AUTHOR)))
				.collect(),
		},
		pallet_sudo: myriad_runtime::SudoConfig { key: endowed_accounts[0].clone() },
		parachain_info: myriad_runtime::ParachainInfoConfig { parachain_id: id },
		pallet_aura: myriad_runtime::AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
		},
		cumulus_pallet_aura_ext: Default::default(),
	}
}

pub fn make_development_testnet_config(id: ParaId) -> ChainSpec {
    let properties = get_properties("MYRIA", 12, 214);

	ChainSpec::from_genesis(
		// Name
		"Myriad Development Tesnet",
		// ID
		"myriad_development_tesnet",
		ChainType::Live,
		move || {
			make_development_testnet_config_genesis(id)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-development".into()),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-dev".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn make_development_testnet_config_genesis(id: ParaId) -> myriad_runtime::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV
		hex!["46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"].into(),
	];

	// ./.maintain/prepare-test-net.sh 2
	let initial_authorities: Vec<(
		AccountId,
		AuraId,
	)> = vec![
		(
			//5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o
			hex!["b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48"].into(),
			//5GBNeWRhZc2jXu7D55rBimKYDk8PGk8itRYFTPfC8RJLKG5o
			hex!["b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48"].unchecked_into(),
		),
		(
			//5Dfis6XL8J2P6JHUnUtArnFWndn62SydeP8ee8sG2ky9nfm9
			hex!["46f136b564e1fad55031404dd84e5cd3fa76bfe7cc7599b39d38fd06663bbc0a"].into(),
			//5Dfis6XL8J2P6JHUnUtArnFWndn62SydeP8ee8sG2ky9nfm9
			hex!["46f136b564e1fad55031404dd84e5cd3fa76bfe7cc7599b39d38fd06663bbc0a"].unchecked_into(),
		),
	];

	const MYRIA: u128 = 1_000_000_000_000;
	const ENDOWMENT: u128 = 1_000_000 * MYRIA;
	const AUTHOR: u128 = 100 * MYRIA;

    myriad_runtime::GenesisConfig {
		frame_system: myriad_runtime::SystemConfig {
			code: myriad_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: myriad_runtime::BalancesConfig {
			balances: endowed_accounts.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), AUTHOR)))
				.collect(),
		},
		pallet_sudo: myriad_runtime::SudoConfig { key: endowed_accounts[0].clone() },
		parachain_info: myriad_runtime::ParachainInfoConfig { parachain_id: id },
		pallet_aura: myriad_runtime::AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
		},
		cumulus_pallet_aura_ext: Default::default(),
	}
}

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
    let properties = get_properties("MYRIA", 12, 214);

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_authority_keys_from_seed("Alice"),
					get_authority_keys_from_seed("Bob"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-local".into()),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> myriad_runtime::GenesisConfig {
	const MYRIA: u128 = 1_000_000_000_000;
	const ENDOWMENT: u128 = 1_000_000 * MYRIA;

	myriad_runtime::GenesisConfig {
		frame_system: myriad_runtime::SystemConfig {
			code: myriad_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: myriad_runtime::BalancesConfig {
			balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
		},
		pallet_sudo: myriad_runtime::SudoConfig { key: root_key },
		parachain_info: myriad_runtime::ParachainInfoConfig { parachain_id: id },
		pallet_aura: myriad_runtime::AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
		},
		cumulus_pallet_aura_ext: Default::default(),
	}
}
