use super::*;
use frame_support::{pallet_prelude::*, sp_runtime::traits::Saturating, traits::Currency};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Server<AccountId, Balance> {
	id: u64,
	owner: AccountId,
	api_url: Vec<u8>,
	staked_amount: Balance,
}
impl<AccountId, Balance> Server<AccountId, Balance>
where
	AccountId: Clone + PartialEq + Eq,
	Balance: Copy + Saturating,
{
	pub fn new(id: u64, owner: &AccountId, api_url: &[u8], staked_amount: Balance) -> Self {
		Self { id, owner: owner.clone(), api_url: api_url.to_vec(), staked_amount }
	}

	pub fn is_authorized(self, owner: &AccountId) -> Option<Self> {
		if &self.owner == owner {
			Some(self)
		} else {
			None
		}
	}

	pub fn get_id(&self) -> u64 {
		self.id
	}

	pub fn get_owner(&self) -> &AccountId {
		&self.owner
	}

	pub fn get_api_url(&self) -> &Vec<u8> {
		&self.api_url
	}

	pub fn get_stake_amount(&self) -> &Balance {
		&self.staked_amount
	}

	pub fn set_id(&mut self, server_id: u64) {
		self.id = server_id;
	}

	pub fn set_owner(mut self, account_id: &AccountId) -> Self {
		self.owner = account_id.clone();
		self
	}

	pub fn set_api_url(mut self, api_url: &[u8]) -> Self {
		self.api_url = api_url.to_vec();
		self
	}

	pub fn set_stake_amount(mut self, amount: Balance) -> Self {
		self.staked_amount = amount;
		self
	}

	pub fn increase_stake_amount(mut self, amount: Balance) -> Self {
		self.staked_amount = self.staked_amount.saturating_add(amount);
		self
	}

	pub fn decrease_stake_amount(mut self, amount: Balance) -> Self {
		self.staked_amount = self.staked_amount.saturating_sub(amount);
		self
	}
}

impl<T, AccountId, Balance> ServerInfo<T> for Server<AccountId, Balance>
where
	T: frame_system::Config<AccountId = AccountId>,
	AccountId: Clone + PartialEq + Eq,
	Balance: Copy + Saturating,
{
	fn get_id(&self) -> u64 {
		self.get_id()
	}

	fn get_owner(&self) -> &AccountId {
		self.get_owner()
	}

	fn get_api_url(&self) -> &Vec<u8> {
		self.get_api_url()
	}
}

#[derive(Encode, Decode, Clone)]
pub enum ServerDataKind<AccountId, Balance> {
	Owner(AccountId),
	ApiUrl(Vec<u8>),
	StakeAmount(Balance),
	UnstakeAmount(Balance),
}

#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub enum Status {
	InProgress,
	Failed,
	Success,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub enum Action<Balance> {
	Stake(Balance),
	Unstake(Balance),
}

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type ServerOf<T> = Server<AccountIdOf<T>, BalanceOf<T>>;
pub type ServerId = u64;
pub type CurrencyOf<T> = <T as self::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type ActionOf<T> = Action<BalanceOf<T>>;
pub type ApiUrl = Vec<u8>;
