use crate::*;

use frame_support::pallet_prelude::*;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum PayloadType {
	Create,
	Delete,
}
impl Default for PayloadType {
	fn default() -> Self {
		PayloadType::Create
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Payload<AccountId> {
	api_url: Vec<u8>,
	access_token: Vec<u8>,
	user_verification: Vec<u8>,
	account_id: AccountId,
	server_id: Vec<u8>,
	ft_identifier: Vec<u8>,
	payload_type: PayloadType,
}
impl<AccountId: Clone> Payload<AccountId> {
	pub fn new(
		api_url: &[u8],
		access_token: &[u8],
		user_verification: &[u8],
		account_id: &AccountId,
		server_id: &[u8],
		ft_identifier: &[u8],
		payload_type: &PayloadType,
	) -> Self {
		Self {
			api_url: api_url.to_vec(),
			access_token: access_token.to_vec(),
			user_verification: user_verification.to_vec(),
			account_id: account_id.clone(),
			server_id: server_id.to_vec(),
			ft_identifier: ft_identifier.to_vec(),
			payload_type: payload_type.clone(),
		}
	}

	pub fn get_api_url(&self) -> &Vec<u8> {
		&self.api_url
	}

	pub fn get_access_token(&self) -> &Vec<u8> {
		&self.access_token
	}

	pub fn get_user_verification(&self) -> &Vec<u8> {
		&self.user_verification
	}

	pub fn get_account_id(&self) -> &AccountId {
		&self.account_id
	}

	pub fn get_server_id(&self) -> &Vec<u8> {
		&self.server_id
	}

	pub fn get_ft_identifier(&self) -> &Vec<u8> {
		&self.ft_identifier
	}

	pub fn get_payload_type(&self) -> &PayloadType {
		&self.payload_type
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct IndexingData<AccountId>(pub Vec<u8>, pub Payload<AccountId>);
