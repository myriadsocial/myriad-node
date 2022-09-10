use super::*;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Server<AccountId> {
	pub id: u64,
	pub owner: AccountId,
	pub api_url: Vec<u8>,
}
impl<AccountId: Clone> Server<AccountId> {
	pub fn new(id: u64, owner: &AccountId, api_url: &[u8]) -> Self {
		Self { id, owner: owner.clone(), api_url: api_url.to_vec() }
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
}

impl<T, AccountId: Clone> ServerInfo<T> for Server<AccountId>
where
	T: frame_system::Config<AccountId = AccountId>,
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
pub enum ServerDataKind<AccountId> {
	Owner(AccountId),
	ApiUrl(Vec<u8>),
}

#[derive(Encode, Decode, Clone)]
pub enum OperatorKind {
	Add,
	Sub,
}

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type ServerOf<T> = Server<AccountIdOf<T>>;
pub type ServerId = u64;
pub type ApiUrl = Vec<u8>;
