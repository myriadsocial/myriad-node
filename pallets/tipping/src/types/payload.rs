use crate::*;

use frame_support::pallet_prelude::*;
use sp_std::vec::Vec;

pub type AccessToken = Vec<u8>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum DataType {
	UserSocialMedia(UserSocialMedia),
	Wallet(Wallet),
}
impl Default for DataType {
	fn default() -> Self {
		DataType::UserSocialMedia(UserSocialMedia::default())
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum PayloadType {
	Create,
	Connect,
	Delete,
}
impl Default for PayloadType {
	fn default() -> Self {
		PayloadType::Delete
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Payload<AccountId> {
	server_id: Vec<u8>,
	api_url: Vec<u8>,
	body: Vec<u8>,
	access_token: Vec<u8>,
	account_id: Option<AccountId>,
	ft_identifier: Vec<u8>,
	payload_type: PayloadType,
}
impl<AccountId: Clone> Payload<AccountId> {
	pub fn init(server_id: &[u8], api_url: &[u8], access_token: &[u8]) -> Self {
		Self {
			server_id: server_id.to_vec(),
			api_url: api_url.to_vec(),
			access_token: access_token.to_vec(),
			body: Vec::new(),
			account_id: None,
			ft_identifier: b"native".to_vec(),
			payload_type: PayloadType::default(),
		}
	}

	pub fn get_server_id(&self) -> &Vec<u8> {
		&self.server_id
	}

	pub fn get_api_url(&self) -> &Vec<u8> {
		&self.api_url
	}

	pub fn get_access_token(&self) -> &Vec<u8> {
		&self.access_token
	}

	pub fn get_body(&self) -> &Vec<u8> {
		&self.body
	}

	pub fn get_account_id(&self) -> &Option<AccountId> {
		&self.account_id
	}

	pub fn get_ft_identifier(&self) -> &Vec<u8> {
		&self.ft_identifier
	}

	pub fn get_payload_type(&self) -> &PayloadType {
		&self.payload_type
	}

	pub fn set_body(mut self, body: &[u8]) -> Self {
		self.body = body.to_vec();
		self
	}

	pub fn set_account_id(mut self, account_id: &AccountId) -> Self {
		self.account_id = Some(account_id.clone());
		self
	}

	pub fn set_ft_identifier(mut self, ft_identifier: &[u8]) -> Self {
		self.ft_identifier = ft_identifier.to_vec();
		self
	}

	pub fn set_payload_type(mut self, payload_type: PayloadType) -> Self {
		self.payload_type = payload_type;
		self
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct IndexingData<AccountId>(Vec<u8>, Payload<AccountId>);
impl<AccountId> IndexingData<AccountId> {
	pub fn get_payload(&self) -> &Payload<AccountId> {
		&self.1
	}

	pub fn init(key_id: &[u8], payload: Payload<AccountId>) -> Self {
		Self(key_id.to_vec(), payload)
	}
}
