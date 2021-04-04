use cumulus_primitives_core::ParaId;
use myriad_runtime::{AccountId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

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

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
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

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
    let properties = get_properties("MYRIAL", 15, 214);

    ChainSpec::from_genesis(
        // Name
        "MYRIAD Local Testnet",
        // ID
        "myriad_local_testnet",  
        ChainType::Local,
        move || {
            testnet_genesis(
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
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
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash")
				],
				// Parachain Id
				id
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
            relay_chain: "rococo-local".into(),
            para_id: id.into(),
        },
    )
}

pub fn staging_testnet_config(id: ParaId) -> ChainSpec {
    let properties = get_properties("MYRIAS", 15, 214);

    ChainSpec::from_genesis(
        // Name
        "MYRIAD Staging Testnet",
        // ID
        "myriad_staging_testnet",  
        ChainType::Live,
        move || {
            testnet_genesis(
                // Sudo account
                //5DSShm3qptXjE5aK7aUoVCQ7ScgCwt8wbH7MzgNwtRg4FPJZ
                hex!["3cd09eecf6faa579ff49a5bb8175c02244da1151cfa75b8b3fc9dcb15b4b281d"].into(),
                // Pre-funded accounts
				vec![
                    //5GE6M2FBBChfGfatFvRmWSgJrvSuxVYB2HNA13Fb5EFMpjst
                    hex!["b819d8c01cbc46e23d9b79f7654f704a828fa1946bc8a97f56889daade1ced4e"].into()
				],
				// Parachain Id
				id
            )
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
            relay_chain: "rococo-staging".into(),
            para_id: id.into(),
        },
    )
}

fn testnet_genesis(
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
) -> myriad_runtime::GenesisConfig {
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
                .map(|k| (k, 10000000000000000000000000000_u128))
                .collect(),
        },
        pallet_sudo: myriad_runtime::SudoConfig { key: root_key },
        parachain_info: myriad_runtime::ParachainInfoConfig { parachain_id: id },
    }
}
