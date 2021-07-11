use sp_core::{Pair, Public, sr25519, H160, U256};
use myriad_appchain_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, EVMConfig, EthereumConfig, WASM_BINARY, Signature,
	BABE_GENESIS_EPOCH_CONFIG,
};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::{ChainType, Properties};
use std::{str::FromStr, collections::BTreeMap};

use myriad_appchain_runtime::{
	ImOnlineConfig, SessionConfig, opaque::SessionKeys,
	StakingConfig, Balance, currency::MYRIA,
};
use sp_consensus_babe::{AuthorityId as BabeId};
use sp_runtime::Perbill;
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use pallet_staking::StakerStatus;
use myriad_appchain_runtime::BeefyConfig;
use beefy_primitives::ecdsa::AuthorityId as BeefyId;
use myriad_appchain_runtime::OctopusAppchainConfig;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use hex_literal::hex;
use sp_core::crypto::UncheckedInto;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	beefy: BeefyId,
	octopus: OctopusId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<BeefyId>(seed),
		get_from_seed::<OctopusId>(seed),
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

pub fn staging_tesnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Staging Testnet",
		// ID
		"myriad_staging_testnet",
		ChainType::Live,
		move || testnet_genesis(
			// WASM Binary
			wasm_binary,
			// Sudo account
			// 5HVgMkXJGoDGQdnTyah4shbhuaiNCmAUdqCyTdYAnr9T9Y1Q
			hex!["f03941f93b990c271015d3b485f137e117aab80af0a03b557966927caaa7d44f"].into(),
			// Initial PoA authorities
			vec![
				(
					//5HBdevUsByJZMGqdZucSr8qXvtRKHRzzqjq2NHVwXUiP8LnC
					hex!["e275bcf8adb68ed14eee7287461236cbe4a1326810e9e22b4baf05964882f828"].into(),
					//5GhTbhujpv3nZQx6idibYSwYeNCN7ddpqqjPjwZn43xdvYMT
					hex!["ccf90463ce9ae4cf881c549b09ddeac1960316930e390ca47eeba95741386e5b"].into(),
					//5GhTbhujpv3nZQx6idibYSwYeNCN7ddpqqjPjwZn43xdvYMT
					hex!["ccf90463ce9ae4cf881c549b09ddeac1960316930e390ca47eeba95741386e5b"].unchecked_into(),
					//5G5ghjBD9fkx9gR59LQLmQvFnayjaRhdBKqpujvNjYjmx4ks
					hex!["b1b04b436a8772b6429a549ae68d72fd88b8533462d03d83d9acaf9500b3ca00"].unchecked_into(),
					//5GhTbhujpv3nZQx6idibYSwYeNCN7ddpqqjPjwZn43xdvYMT
					hex!["ccf90463ce9ae4cf881c549b09ddeac1960316930e390ca47eeba95741386e5b"].unchecked_into(),
					//KW8mwncjSVKxsCbACjDDk2bLHLsq2gkeVw5xjKW4vSLgWimn1
					hex!["0302c5928b0861672271346c29e30faa2cb5328e024d1c45f2689e886cb12b6de1"].unchecked_into(),
					//5GhTbhujpv3nZQx6idibYSwYeNCN7ddpqqjPjwZn43xdvYMT
					hex!["ccf90463ce9ae4cf881c549b09ddeac1960316930e390ca47eeba95741386e5b"].unchecked_into(),
				),
				(
					//5GVjfe6b3s1sdMr2Q5oxVcSGHL33KWmwJZxDThR9b6tteyMG
					hex!["c40829541f7121c67ccb2bee956887735687e57be0ada26289501636ac60946f"].into(),
					//5H9RP9sy2g9Jaj1GG2zGaytLdxoBHQnqMaKmqvtFPJpYiRV3
					hex!["e0c5efc09df70c2e236e32ebba4c89a5ae538dacf25412e2a23e6a175291453a"].into(),
					//5H9RP9sy2g9Jaj1GG2zGaytLdxoBHQnqMaKmqvtFPJpYiRV3
					hex!["e0c5efc09df70c2e236e32ebba4c89a5ae538dacf25412e2a23e6a175291453a"].unchecked_into(),
					//5Dvf9Qq8rmfFdSLACJwvcDEYJMYYq6wYiKkazZrUmWLqUDEE
					hex!["52556063e8c72431f643c8eb66ba172d5b0d2a095429a8a6e29b522208e26ccd"].unchecked_into(),
					//5H9RP9sy2g9Jaj1GG2zGaytLdxoBHQnqMaKmqvtFPJpYiRV3
					hex!["e0c5efc09df70c2e236e32ebba4c89a5ae538dacf25412e2a23e6a175291453a"].unchecked_into(),
					//KW9uY45eZ65PpHxk21KiXvc8XiTse6amUPKpAWgvxmfhorryw
					hex!["0334cbe01d6db7bf3d0f4148c468a3a01a5a560f21244d9891c35de23d7c752c24"].unchecked_into(),
					//5H9RP9sy2g9Jaj1GG2zGaytLdxoBHQnqMaKmqvtFPJpYiRV3
					hex!["e0c5efc09df70c2e236e32ebba4c89a5ae538dacf25412e2a23e6a175291453a"].unchecked_into(),
				),
			],
			// Pre-funded accounts
			vec![
				// 5HVgMkXJGoDGQdnTyah4shbhuaiNCmAUdqCyTdYAnr9T9Y1Q
				hex!["f03941f93b990c271015d3b485f137e117aab80af0a03b557966927caaa7d44f"].into(),
			],
			// Appchain Id
			"myriad_staging_testnet",
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-staging-tesnet".into()),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn development_tesnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Development Testnet",
		// ID
		"myriad_development_testnet",
		ChainType::Live,
		move || testnet_genesis(
			// WASM Binary
			wasm_binary,
			// Sudo account
			// 5EZYLWe1j3MjuH1vJf6Mc5CxaeGfVeoAQn3DwYuLvABDYU1U
			hex!["6e768960d4a61b5583eb76ac22ba91dce97ef55fa8ca4b764c774cdb9af93b36"].into(),
			// Initial PoA authorities
			vec![
				(
					//5CXF4c7rRuX3NfEbToR32ScG3mDNvJ1aFGVg7Wd6YZS5cU37
					hex!["143d6dbd1fa1906ef35a1308afd7940cf8e987271d47a8c196062eca1ef87a5b"].into(),
					//5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
					hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"].into(),
					//5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
					hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"].unchecked_into(),
					//5H83G9CMm7wPYq6FeYqrn7ueVBWBvK37xdZYamtm8re2LYn3
					hex!["dfb839beaf6fe750ca87b9059161d43f2682a6c3a0ac765f1e5054063ed9903b"].unchecked_into(),
					//5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
					hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"].unchecked_into(),
					//KW7hbC4ZzNJEjkofpAhxC51PRHxPxSr6RZ8NG4UyerChmQH3E
					hex!["02d337069cb73bcefafc4e35e5189ad62932e4f2ee3f985b6bbff654cb68017ff1"].unchecked_into(),
					//5Gx1QL5a18H63ofyYdZhjpiTKA9XCgpfoTCztT2dpKsHQE9j
					hex!["d811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073"].unchecked_into(),
				),
			],
			// Pre-funded accounts
			vec![
				// 5EZYLWe1j3MjuH1vJf6Mc5CxaeGfVeoAQn3DwYuLvABDYU1U
				hex!["6e768960d4a61b5583eb76ac22ba91dce97ef55fa8ca4b764c774cdb9af93b36"].into(),
			],
			// Appchain Id
			"myriad_development_testnet",
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-development-tesnet".into()),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Local Testnet",
		// ID
		"myriad_local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			// WASM Binary
			wasm_binary,
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			],
			// Appchain Id
			"myriad_local_testnet",
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-local-tesnet".into()),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_development_tesnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("MYRIA", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Myriad Local Development Testnet",
		// ID
		"myriad_local_development_testnet",
		ChainType::Development,
		move || testnet_genesis(
			// WASM Binary
			wasm_binary,
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			],
			// Appchain Id
			"myriad_local_development_testnet",
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("myriad-local-development-tesnet".into()),
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(
		AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId
	)>,
	endowed_accounts: Vec<AccountId>,
	appchain_id: &str,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 1_000_000 * MYRIA;
	const STASH: Balance = 100 * MYRIA;
	const OCTOPUS_STASH: Balance = 100;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		im_online: ImOnlineConfig {
			keys: vec![],
		},
		beefy: BeefyConfig {
			authorities: vec![],
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						)
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						StakerStatus::Validator,
					)
				})
				.collect(),
			.. Default::default()
		},
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0xD3C21BCECCEDA1000000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					}
				);
				map
			},
		},
		ethereum: EthereumConfig {},
		octopus_appchain: OctopusAppchainConfig {
			appchain_id: appchain_id.to_string(),
			validators: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), OCTOPUS_STASH))
				.collect(),
			asset_id_by_name: vec![("test-stable.testnet".to_string(), 0)],
		},
	}
}
