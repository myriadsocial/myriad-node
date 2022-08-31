use super::*;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Server<AccountId> {
	pub id: Vec<u8>,
	pub owner: AccountId,
	pub name: Vec<u8>,
	pub api_url: Vec<u8>,
	pub web_url: Vec<u8>,
}
impl<AccountId: Clone> Server<AccountId> {
	pub fn new(id: &[u8], owner: &AccountId, name: &[u8], api_url: &[u8], web_url: &[u8]) -> Self {
		Self {
			id: id.to_vec(),
			owner: owner.clone(),
			name: name.to_vec(),
			api_url: api_url.to_vec(),
			web_url: web_url.to_vec(),
		}
	}

	pub fn get_id(&self) -> &Vec<u8> {
		&self.id
	}

	pub fn get_owner(&self) -> &AccountId {
		&self.owner
	}

	pub fn get_name(&self) -> &Vec<u8> {
		&self.name
	}

	pub fn get_api_url(&self) -> &Vec<u8> {
		&self.api_url
	}

	pub fn get_web_url(&self) -> &Vec<u8> {
		&self.web_url
	}

	pub fn set_id(&mut self, server_id: &[u8]) {
		self.id = server_id.to_vec();
	}

	pub fn set_owner(mut self, account_id: &AccountId) -> Self {
		self.owner = account_id.clone();
		self
	}

	pub fn set_name(mut self, name: &[u8]) -> Self {
		self.name = name.to_vec();
		self
	}

	pub fn set_api_url(mut self, api_url: &[u8]) -> Self {
		self.api_url = api_url.to_vec();
		self
	}

	pub fn set_web_url(mut self, web_url: &[u8]) -> Self {
		self.web_url = web_url.to_vec();
		self
	}
}

impl<T, AccountId: Clone> ServerInfo<T> for Server<AccountId>
where
	T: frame_system::Config<AccountId = AccountId>,
{
	fn get_id(&self) -> &Vec<u8> {
		self.get_id()
	}

	fn get_owner(&self) -> &AccountId {
		self.get_owner()
	}

	fn get_name(&self) -> &Vec<u8> {
		self.get_name()
	}

	fn get_api_url(&self) -> &Vec<u8> {
		self.get_api_url()
	}

	fn get_web_url(&self) -> &Vec<u8> {
		self.get_web_url()
	}
}

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type ServerOf<T> = Server<AccountIdOf<T>>;
pub type ServerId = Vec<u8>;
