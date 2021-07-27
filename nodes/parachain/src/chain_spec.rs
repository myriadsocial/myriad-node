use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use myriad_parachain_runtime::{
	currency::MYRIA, AccountId, AuraConfig, AuraId, Balance, BalancesConfig, GenesisConfig,
	ParachainInfoConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	properties
}

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

pub fn staging_tesnet_config(id: ParaId) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Staging Testnet",
		// ID
		"myriad_staging_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5HVgMkXJGoDGQdnTyah4shbhuaiNCmAUdqCyTdYAnr9T9Y1Q
				hex!["f03941f93b990c271015d3b485f137e117aab80af0a03b557966927caaa7d44f"].into(),
				// Initial PoA authorities
				vec![
					// 5GhTbhujpv3nZQx6idibYSwYeNCN7ddpqqjPjwZn43xdvYMT
					hex!["ccf90463ce9ae4cf881c549b09ddeac1960316930e390ca47eeba95741386e5b"]
						.unchecked_into(),
					// 5H9RP9sy2g9Jaj1GG2zGaytLdxoBHQnqMaKmqvtFPJpYiRV3
					hex!["e0c5efc09df70c2e236e32ebba4c89a5ae538dacf25412e2a23e6a175291453a"]
						.unchecked_into(),
				],
				// Pre-funded accounts
				vec![
					// 5HVgMkXJGoDGQdnTyah4shbhuaiNCmAUdqCyTdYAnr9T9Y1Q
					hex!["f03941f93b990c271015d3b485f137e117aab80af0a03b557966927caaa7d44f"].into(),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-staging-tesnet"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-staging".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	))
}

pub fn development_tesnet_config(id: ParaId) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Development Testnet",
		// ID
		"myriad_development_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5EZYLWe1j3MjuH1vJf6Mc5CxaeGfVeoAQn3DwYuLvABDYU1U
				hex!["6e768960d4a61b5583eb76ac22ba91dce97ef55fa8ca4b764c774cdb9af93b36"].into(),
				// Initial PoA authorities
				vec![
					// 5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
					hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"]
						.unchecked_into(),
				],
				// Pre-funded accounts
				vec![
					// 5EZYLWe1j3MjuH1vJf6Mc5CxaeGfVeoAQn3DwYuLvABDYU1U
					hex!["6e768960d4a61b5583eb76ac22ba91dce97ef55fa8ca4b764c774cdb9af93b36"].into(),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-development-tesnet"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-dev".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	))
}

pub fn local_testnet_config(id: ParaId) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Local Testnet",
		// ID
		"myriad_local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-local-tesnet"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	))
}

pub fn local_development_tesnet_config(id: ParaId) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Local Development Testnet",
		// ID
		"myriad_local_development_testnet",
		ChainType::Development,
		move || {
			testnet_genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![get_from_seed::<AuraId>("Alice")],
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-local-development-tesnet"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-dev".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	))
}

fn testnet_genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 1_000_000 * MYRIA;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		aura: AuraConfig { authorities: initial_authorities },
		aura_ext: Default::default(),
		parachain_info: ParachainInfoConfig { parachain_id: id },
		parachain_system: Default::default(),
	}
}
