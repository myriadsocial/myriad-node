#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use beefy_primitives::{crypto::AuthorityId as BeefyId, mmr::MmrLeafVersion};
use sp_api::impl_runtime_apis;
use sp_consensus_babe::{
	AllowedSlots::PrimaryAndSecondaryVRFSlots, BabeConfiguration, BabeEpochConfiguration, Epoch,
	EquivocationProof, OpaqueKeyOwnershipProof, Slot,
};
use sp_core::{crypto::KeyTypeId, sr25519, Encode, OpaqueMetadata, H256};
use sp_inherents::{CheckInherentsResult, InherentData};
use sp_mmr_primitives as mmr;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, Extrinsic, Hash as HashT,
		IdentifyAccount, Keccak256, NumberFor, OpaqueKeys, SaturatedConversion, StaticLookup,
		Verify,
	},
	transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, FixedPointNumber, MultiAddress, MultiSignature, Perbill, Perquintill,
};
use sp_staking::SessionIndex;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
	construct_runtime,
	dispatch::DispatchClass,
	parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU32, Everything, KeyOwnerProofSystem,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		ConstantMultiplier, IdentityFee, Weight,
	},
	PalletId,
};
pub use frame_system::Call as SystemCall;
use frame_system::{
	limits::{BlockLength, BlockWeights},
	offchain, ChainContext, CheckEra, CheckGenesis, CheckNonZeroSender, CheckNonce,
	CheckSpecVersion, CheckTxVersion, CheckWeight, EnsureRoot, EnsureSigned,
};

use pallet_babe::{
	AuthorityId as BabeId, EquivocationHandler as BabeEquivocationHandler, ExternalTrigger,
};
pub use pallet_balances::{AccountData, Call as BalancesCall};
use pallet_beefy_mmr::{BeefyEcdsaToEthereum, DepositBeefyDigest};
use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
	EquivocationHandler as GrandpaEquivocationHandler,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::sr25519::AuthorityId as OctopusId;
use pallet_octopus_lpos::{EraIndex, ExposureOf, FilterHistoricalOffences};
use pallet_session::{historical as pallet_session_historical, FindAccountFromAuthorIndex};
use pallet_session_historical::NoteHistoricalRoot;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{
	ChargeTransactionPayment, CurrencyAdapter, Multiplier, TargetedFeeAdjustment,
};

// Local pallet
pub use pallet_server;
pub use pallet_tipping;

/// An index to a block.
pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// Balance of an account.
pub type Balance = u128;
/// Type used for expressing timestamp.
pub type Moment = u64;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = H256;
/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	CheckNonZeroSender<Runtime>,
	CheckSpecVersion<Runtime>,
	CheckTxVersion<Runtime>,
	CheckGenesis<Runtime>,
	CheckEra<Runtime>,
	CheckNonce<Runtime>,
	CheckWeight<Runtime>,
	ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;
pub type CollectionId = u128;
pub type ItemId = u128;
pub type AssetId = u32;

pub struct OctopusAppCrypto;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_server, Server]
		[pallet_tipping, Tipping]
	);
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub babe: Babe,
			pub grandpa: Grandpa,
			pub im_online: ImOnline,
			pub beefy: Beefy,
			pub octopus: OctopusAppchain,
		}
	}
}

/// The native token, uses 18 decimals of precision.
pub mod currency {
	use super::Balance;

	pub const OCTS: Balance = 1_000_000_000_000_000_000;

	pub const UNITS: Balance = 1_000_000_000_000_000_000;
	pub const DOLLARS: Balance = UNITS;
	pub const CENTS: Balance = DOLLARS / 100;
	pub const MILLICENTS: Balance = CENTS / 1_000;

	pub const EXISTENSIAL_DEPOSIT: Balance = CENTS;
	pub const BYTE_FEE: Balance = 10 * MILLICENTS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		(items as Balance) * CENTS + (bytes as Balance) * BYTE_FEE
	}
}

// To learn more about runtime versioning and what each of the following value means:
//   https://docs.substrate.io/v3/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("appchain"),
	impl_name: create_runtime_str!("myriad"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 2030,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 2,
	state_version: 1,
};

/// Since BABE is probabilistic this is the average expected block time that
/// we are targeting. Blocks will be produced at a minimum duration defined
/// by `SLOT_DURATION`, but some slots will not be allocated to any
/// authority and hence no block will be produced. We expect to have this
/// block time on average following the defined slot duration and the value
/// of `c` configured for BABE (where `1 - c` represents the probability of
/// a slot being empty).
/// This value is only used indirectly to define the unit constants below
/// that are expressed in blocks. The rest of the code should use
/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
/// minimum period).
///
/// If using BABE with secondary slots (default) then all of the slots will
/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
/// `SLOT_DURATION` should have the same value.
///
/// <https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html#-6.-practical-results>
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const WEEKS: BlockNumber = DAYS * 7;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
//       Attempting to do so will brick block production.
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 4 * HOURS;
pub const EPOCH_DURATION_IN_SLOTS: Moment = {
	const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

	(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as Moment
};

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: BabeEpochConfiguration =
	BabeEpochConfiguration { c: PRIMARY_PROBABILITY, allowed_slots: PrimaryAndSecondaryVRFSlots };

/// We assume that an on-initialize consumes 1% of the weight on average, hence a single extrinsic
/// will not be allowed to consume more than `AvailableBlockRatio - 1%`.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(1);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.saturating_mul(2);

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

// Configure FRAME pallets to include in runtime.
parameter_types! {
	pub RuntimeBlockLength: BlockLength = BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
	type AccountData = AccountData<Balance>;
	type AccountId = AccountId;
	type BaseCallFilter = Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = RuntimeBlockLength;
	type BlockNumber = BlockNumber;
	type BlockWeights = RuntimeBlockWeights;
	type DbWeight = RocksDbWeight;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Index = Index;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type MaxConsumers = ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = ConstU16<42>;
	type SystemWeightInfo = ();
	type Version = Version;
}

impl offchain::SigningTypes for Runtime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<C> offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
}

impl offchain::AppCrypto<<Signature as Verify>::Signer, Signature> for OctopusAppCrypto {
	type RuntimeAppPublic = OctopusId;
	type GenericSignature = sr25519::Signature;
	type GenericPublic = sr25519::Public;
}

impl<LocalCall> offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: <Signature as Verify>::Signer,
		account: AccountId,
		nonce: Index,
	) -> Option<(RuntimeCall, <UncheckedExtrinsic as Extrinsic>::SignaturePayload)> {
		let tip = 0;
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number().saturated_into::<u64>().saturating_sub(1);
		let era = generic::Era::mortal(period, current_block);
		let extra = (
			CheckNonZeroSender::<Runtime>::new(),
			CheckSpecVersion::<Runtime>::new(),
			CheckTxVersion::<Runtime>::new(),
			CheckGenesis::<Runtime>::new(),
			CheckEra::<Runtime>::from(era),
			CheckNonce::<Runtime>::from(nonce),
			CheckWeight::<Runtime>::new(),
			ChargeTransactionPayment::<Runtime>::from(tip),
		);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = <Self as frame_system::Config>::Lookup::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}

parameter_types! {
	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const EpochDuration: Moment = EPOCH_DURATION_IN_SLOTS;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const MaxAuthorities: u32 = 100;
	pub const ReportLongevity: Moment = BondingDuration::get() as Moment * SessionsPerEra::get() as Moment * EpochDuration::get();
}

impl pallet_babe::Config for Runtime {
	type DisabledValidators = Session;
	type EpochChangeTrigger = ExternalTrigger;
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type HandleEquivocation = BabeEquivocationHandler<
		Self::KeyOwnerIdentification,
		FilterHistoricalOffences<OctopusLpos, Offences>,
		ReportLongevity,
	>;
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		BabeId,
	)>>::IdentificationTuple;
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, BabeId)>>::Proof;
	type KeyOwnerProofSystem = Historical;
	type MaxAuthorities = MaxAuthorities;
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = MinimumPeriod;
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = currency::EXISTENSIAL_DEPOSIT;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type AccountStore = frame_system::Pallet<Runtime>;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const OperationalFeeMultiplier: u8 = 5;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub const TransactionByteFee: Balance = currency::BYTE_FEE;
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}

impl pallet_transaction_payment::Config for Runtime {
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type RuntimeEvent = RuntimeEvent;
	type WeightToFee = IdentityFee<Balance>;
}

parameter_types! {
	pub const CollectionDeposit: Balance = 100 * currency::DOLLARS;
	pub const ItemDeposit: Balance = currency::DOLLARS;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
}

impl pallet_uniques::Config<pallet_uniques::Instance1> for Runtime {
	type AttributeDepositBase = MetadataDepositBase;
	type CollectionDeposit = CollectionDeposit;
	type CollectionId = CollectionId;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Currency = Balances;
	type DepositPerByte = MetadataDepositPerByte;
	type ForceOrigin = EnsureRoot<AccountId>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type ItemDeposit = ItemDeposit;
	type ItemId = ItemId;
	type KeyLimit = KeyLimit;
	type Locker = ();
	type MetadataDepositBase = MetadataDepositBase;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = StringLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
}

parameter_types! {
	pub const ApprovalDeposit: Balance = currency::DOLLARS;
	pub const AssetDeposit: Balance = 100 * currency::DOLLARS;
	pub const MetadataDepositBase: Balance = 10 * currency::DOLLARS;
	pub const MetadataDepositPerByte: Balance = currency::DOLLARS;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config<pallet_assets::Instance1> for Runtime {
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = ConstU128<{ currency::DOLLARS }>;
	type AssetDeposit = AssetDeposit;
	type AssetId = AssetId;
	type Balance = Balance;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type Freezer = ();
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = StringLimit;
	type WeightInfo = ();
}

parameter_types! {
	pub const GracePeriod: u32 = 10;
	pub const OctopusAppchainPalletId: PalletId = PalletId(*b"py/octps");
	pub const RequestEventLimit: u32 = 10;
	pub const UnsignedPriority: u64 = 1 << 21;
}

impl pallet_octopus_appchain::Config for Runtime {
	type AppCrypto = OctopusAppCrypto;
	type AuthorityId = OctopusId;
	type BridgeInterface = OctopusBridge;
	type GracePeriod = GracePeriod;
	type LposInterface = OctopusLpos;
	type MaxValidators = MaxAuthorities;
	type RequestEventLimit = RequestEventLimit;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type UnsignedPriority = UnsignedPriority;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type WeightInfo = ();
}

impl pallet_octopus_bridge::Config for Runtime {
	type AppchainInterface = OctopusAppchain;
	type AssetBalance = Balance;
	type AssetId = AssetId;
	type AssetIdByTokenId = OctopusBridge;
	type CollectionId = CollectionId;
	type Convertor = ();
	type Currency = Balances;
	type Fungibles = OctopusAssets;
	type ItemId = ItemId;
	type Nonfungibles = OctopusUniques;
	type PalletId = OctopusAppchainPalletId;
	type RuntimeEvent = RuntimeEvent;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type WeightInfo = ();
}

parameter_types! {
	pub const BondingDuration: EraIndex = 24 * 21;
	pub const SessionsPerEra: SessionIndex = 6;
}

impl pallet_octopus_lpos::Config for Runtime {
	type AppchainInterface = OctopusAppchain;
	type BondingDuration = BondingDuration;
	type Currency = Balances;
	type PalletId = OctopusAppchainPalletId;
	type RuntimeEvent = RuntimeEvent;
	type SessionInterface = Self;
	type SessionsPerEra = SessionsPerEra;
	type UnixTime = Timestamp;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxMessagePayloadSize: u32 = 256;
	pub const MaxMessagesPerCommit: u32 = 20;
}

impl pallet_octopus_upward_messages::Config for Runtime {
	type Hashing = Keccak256;
	type MaxMessagePayloadSize = MaxMessagePayloadSize;
	type MaxMessagesPerCommit = MaxMessagesPerCommit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 0;
}

impl pallet_authorship::Config for Runtime {
	type EventHandler = (OctopusLpos, ImOnline);
	type FilterUncle = ();
	type FindAuthor = FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
}

impl pallet_session_historical::Config for Runtime {
	type FullIdentification = u128;
	type FullIdentificationOf = ExposureOf<Runtime>;
}

impl pallet_session::Config for Runtime {
	type Keys = opaque::SessionKeys;
	type NextSessionRotation = Babe;
	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type SessionManager = NoteHistoricalRoot<Self, OctopusLpos>;
	type ShouldEndSession = Babe;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = ConvertInto;
	type WeightInfo = ();
}

impl pallet_offences::Config for Runtime {
	type IdentificationTuple = pallet_session_historical::IdentificationTuple<Self>;
	type OnOffenceHandler = ();
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_grandpa::Config for Runtime {
	type HandleEquivocation = GrandpaEquivocationHandler<
		Self::KeyOwnerIdentification,
		FilterHistoricalOffences<OctopusLpos, Offences>,
		ReportLongevity,
	>;
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	type KeyOwnerProofSystem = Historical;
	type MaxAuthorities = MaxAuthorities;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl pallet_beefy::Config for Runtime {
	type BeefyId = BeefyId;
	type MaxAuthorities = MaxAuthorities;
	type OnNewValidatorSet = MmrLeaf;
}

parameter_types! {
	/// Version of the produced MMR leaf.
	///
	/// The version consists of two parts;
	/// - `major` (3 bits)
	/// - `minor` (5 bits)
	///
	/// `major` should be updated only if decoding the previous MMR Leaf format from the payload
	/// is not possible (i.e. backward incompatible change).
	/// `minor` should be updated if fields are added to the previous MMR Leaf, which given SCALE
	/// encoding does not prevent old leafs from being decoded.
	///
	/// Hence we expect `major` to be changed really rarely (think never).
	/// See [`MmrLeafVersion`] type documentation for more details.
	pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(0, 0);
}

impl pallet_beefy_mmr::Config for Runtime {
	type BeefyAuthorityToMerkleLeaf = BeefyEcdsaToEthereum;
	type BeefyDataProvider = ();
	type LeafExtra = Vec<u8>;
	type LeafVersion = LeafVersion;
}

impl pallet_mmr::Config for Runtime {
	const INDEXING_PREFIX: &'static [u8] = b"mmr";
	type Hash = <Keccak256 as HashT>::Output;
	type Hashing = Keccak256;
	type LeafData = pallet_beefy_mmr::Pallet<Runtime>;
	type OnNewRoot = DepositBeefyDigest<Runtime>;
	type WeightInfo = ();
}

parameter_types! {
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type MaxKeys = MaxKeys;
	type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
	type NextSessionRotation = Babe;
	type ReportUnresponsiveness = FilterHistoricalOffences<OctopusLpos, Offences>;
	type RuntimeEvent = RuntimeEvent;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type ValidatorSet = Historical;
	type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

// Local pallets
parameter_types! {
	pub const MaxScheduledPerBlock: u32 = DAYS;
	pub const MinimumStakeAmount: Balance = 50_000 * currency::DOLLARS;
	pub const ScheduledBlockTime: BlockNumber = 5;
}

impl pallet_server::Config for Runtime {
	type Currency = Balances;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type MinimumStakeAmount = MinimumStakeAmount;
	type RuntimeEvent = RuntimeEvent;
	type ScheduledBlockTime = ScheduledBlockTime;
	type WeightInfo = ();
}

impl pallet_tipping::Config for Runtime {
	type Assets = OctopusAssets;
	type Currency = Balances;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type TimeProvider = Timestamp;
	type WeightInfo = ();
}

// type Migrations = (
// 	// This migration is used to set the interval value in the upward-messages. It should be
// 	// deleted after the upgrade.
// 	SetIntervalValueRuntimeUpgrade,
// );

/// Please set the value of interval according to your own needs.
const INTERVAL: u32 = 1;
pub struct SetIntervalValueRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for SetIntervalValueRuntimeUpgrade {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		pallet_octopus_upward_messages::migrations::migration_to_v1::<Runtime>(INTERVAL)
	}
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		Babe: pallet_babe,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		OctopusUniques: pallet_uniques::<Instance1>,
		OctopusAssets: pallet_assets::<Instance1>,
		OctopusAppchain: pallet_octopus_appchain,
		OctopusBridge: pallet_octopus_bridge,
		OctopusLpos: pallet_octopus_lpos,
		OctopusUpwardMessages: pallet_octopus_upward_messages,
		Authorship: pallet_authorship,
		Historical: pallet_session_historical::{Pallet},
		Session: pallet_session,
		Offences: pallet_offences,
		Grandpa: pallet_grandpa,
		Beefy: pallet_beefy,
		MmrLeaf: pallet_beefy_mmr,
		Mmr: pallet_mmr,
		ImOnline: pallet_im_online,
		Sudo: pallet_sudo,

		// Local pallets
		Server: pallet_server,
		Tipping: pallet_tipping,
	}
);

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_babe::BabeApi<Block> for Runtime {
		fn configuration() -> BabeConfiguration {
			// The choice of `c` parameter (where `1 - c` represents the
			// probability of a slot being empty), is done in accordance to the
			// slot duration and expected target block time, for safely
			// resisting network delays of maximum two seconds.
			// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
			let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
			BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: epoch_config.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: epoch_config.allowed_slots,
			}
		}

		fn current_epoch_start() -> Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(_slot: Slot, authority_id: BabeId) -> Option<OpaqueKeyOwnershipProof> {
			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id)).map(|p| p.encode()).map(OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(equivocation_proof, key_owner_proof)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: fg_primitives::EquivocationProof<<Block as BlockT>::Hash, NumberFor<Block>>,
			key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(equivocation_proof, key_owner_proof)
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			Historical::prove((fg_primitives::KEY_TYPE, authority_id)).map(|p| p.encode()).map(fg_primitives::OpaqueKeyOwnershipProof::new)
		}
	}

	impl beefy_primitives::BeefyApi<Block> for Runtime {
		fn validator_set() -> Option<beefy_primitives::ValidatorSet<BeefyId>> {
			Beefy::validator_set()
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}

		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl sp_mmr_primitives::MmrApi<Block, Hash> for Runtime {
		fn generate_proof(leaf_index: u64) -> Result<(mmr::EncodableOpaqueLeaf, mmr::Proof<Hash>), mmr::Error> {
			Mmr::generate_batch_proof(vec![leaf_index])
				.and_then(|(leaves, proof)| Ok((
					mmr::EncodableOpaqueLeaf::from_leaf(&leaves[0]),
					mmr::BatchProof::into_single_leaf_proof(proof)?
				)))
		}

		fn verify_proof(leaf: mmr::EncodableOpaqueLeaf, proof: mmr::Proof<Hash>) -> Result<(), mmr::Error> {
			pub type Leaf = <<Runtime as pallet_mmr::Config>::LeafData as mmr::LeafDataProvider>::LeafData;

			let leaf: Leaf = leaf.into_opaque_leaf().try_decode().ok_or(mmr::Error::Verify)?;

			Mmr::verify_leaves(vec![leaf], mmr::Proof::into_batch_proof(proof))
		}

		fn verify_proof_stateless(
			root: Hash,
			leaf: mmr::EncodableOpaqueLeaf,
			proof: mmr::Proof<Hash>,
		) -> Result<(), mmr::Error> {
			type MmrHashing = <Runtime as pallet_mmr::Config>::Hashing;

			let node = mmr::DataOrHash::Data(leaf.into_opaque_leaf());

			pallet_mmr::verify_leaves_proof::<MmrHashing, _>(root, vec![node], mmr::Proof::into_batch_proof(proof))
		}

		fn mmr_root() -> Result<Hash, mmr::Error> {
			Ok(Mmr::mmr_root())
		}

		fn generate_batch_proof(leaf_indices: Vec<mmr::LeafIndex>)
			-> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::BatchProof<Hash>), mmr::Error>
		{
			Mmr::generate_batch_proof(leaf_indices)
				.map(|(leaves, proof)| (leaves.into_iter().map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf)).collect(), proof))
		}

		fn verify_batch_proof(leaves: Vec<mmr::EncodableOpaqueLeaf>, proof: mmr::BatchProof<Hash>)
			-> Result<(), mmr::Error>
		{
			pub type MmrLeaf = <<Runtime as pallet_mmr::Config>::LeafData as mmr::LeafDataProvider>::LeafData;

			let leaves = leaves.into_iter().map(|leaf|
				leaf.into_opaque_leaf()
				.try_decode()
				.ok_or(mmr::Error::Verify)).collect::<Result<Vec<MmrLeaf>, mmr::Error>>()?;

			Mmr::verify_leaves(leaves, proof)
		}

		fn verify_batch_proof_stateless(
			root: Hash,
			leaves: Vec<mmr::EncodableOpaqueLeaf>,
			proof: mmr::BatchProof<Hash>
		) -> Result<(), mmr::Error> {
			type MmrHashing = <Runtime as pallet_mmr::Config>::Hashing;

			let nodes = leaves.into_iter().map(|leaf|mmr::DataOrHash::Data(leaf.into_opaque_leaf())).collect();

			pallet_mmr::verify_leaves_proof::<MmrHashing, _>(root, nodes, proof)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, TrackedStorageKey};
			use frame_support::traits::WhitelistedStorageKeys;

			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();
			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmarks!(params, batches);

			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade().unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			log::info!(
				target: "node-runtime",
				"try-runtime: executing block {:?} / root checks: {:?} / try-state-select: {:?}",
				block.header.hash(),
				state_root_check,
				select,
			);
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, select).unwrap()
		}
	}
}
