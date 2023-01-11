use crate as pallet_tipping;

use sp_core::{
	sr25519::{self as sr25519, Signature},
	Pair, H256,
};
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
};

use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, Everything, GenesisBuild},
	weights::Weight,
};
use frame_system as system;

use pallet_balances::AccountData;

type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Test>;
type Block = system::mocking::MockBlock<Test>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Moment = u64;
type Balance = u128;
type AssetId = u32;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Tipping: pallet_tipping,
	}
);

parameter_types! {
	pub BlockWeights: system::limits::BlockWeights = system::limits::BlockWeights::simple_max(Weight::from_ref_time(1024));
}

impl system::Config for Test {
	type AccountData = AccountData<Balance>;
	type AccountId = sr25519::Public;
	type BaseCallFilter = Everything;
	type BlockHashCount = ConstU64<250>;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = ConstU32<2>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = ConstU16<42>;
	type SystemWeightInfo = ();
	type Version = ();
}

parameter_types! {
	pub const MinimumPeriod: Moment = 10 / 2;
}

impl pallet_timestamp::Config for Test {
	type MinimumPeriod = MinimumPeriod;
	type Moment = Moment;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

parameter_types! {
	pub static ExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const ApprovalDeposit: Balance = 1;
	pub const AssetAccountDeposit: Balance = 10;
	pub const AssetDeposit: Balance = 1;
	pub const MetadataDepositBase: Balance = 1;
	pub const MetadataDepositPerByte: Balance = 1;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config for Test {
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type AssetDeposit = AssetDeposit;
	type AssetId = AssetId;
	type Balance = Balance;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = system::EnsureRoot<AccountId>;
	type Freezer = ();
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = StringLimit;
	type WeightInfo = ();
}

impl pallet_tipping::Config for Test {
	type Assets = Assets;
	type Currency = Balances;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type TimeProvider = Timestamp;
	type WeightInfo = ();
}

pub fn account_key(s: &str) -> sr25519::Public {
	sr25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valud; qed")
		.public()
}

pub struct ExternalityBuilder {
	existential_deposit: u64,
}

impl ExternalityBuilder {
	pub fn build(&self) -> TestExternalities {
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

		let alice_public = account_key("alice");
		let bob_public = account_key("bob");
		let john_public = account_key("john");
		let satoshi_public = account_key("satoshi");
		let sender_1_public = account_key("sender_1");
		let sender_2_public = account_key("sender_2");
		let admin_public = account_key("admin");

		pallet_assets::GenesisConfig::<Test> {
			assets: vec![(1, alice_public, true, 1), (2, alice_public, true, 1)],
			metadata: vec![
				(1, b"DeBio".to_vec(), b"DBIO".to_vec(), 18),
				(2, b"Doge".to_vec(), b"DOGE".to_vec(), 18),
			],
			accounts: vec![
				(1, alice_public, 10),
				(1, bob_public, 20),
				(1, john_public, 30),
				(1, satoshi_public, 40),
				(1, admin_public, 50),
				(1, sender_1_public, 200),
				(1, sender_2_public, 200),
				(2, alice_public, 10),
				(2, bob_public, 20),
				(2, john_public, 30),
				(2, satoshi_public, 40),
				(2, admin_public, 50),
				(2, sender_1_public, 200),
				(2, sender_2_public, 200),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(alice_public, 10),
				(bob_public, 20),
				(john_public, 30),
				(satoshi_public, 40),
				(admin_public, 50),
				(sender_1_public, 200),
				(sender_2_public, 200),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}

	pub fn default() -> Self {
		Self { existential_deposit: 0 }
	}
}
