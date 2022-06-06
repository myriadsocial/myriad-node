use crate::*;

use frame_support::pallet_prelude::*;
use serde::{Deserialize, Deserializer};
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct APIResult<AccountId>(
	ServerId,
	FtIdentifier,
	AccessToken,
	Option<AccountId>,
	Option<DataType>,
);
impl<AccountId> APIResult<AccountId> {
	pub fn init(server_id: &[u8], ft_identifier: &[u8], access_token: &[u8]) -> Self {
		Self(server_id.to_vec(), ft_identifier.to_vec(), access_token.to_vec(), None, None)
	}

	pub fn get_server_id(&self) -> &Vec<u8> {
		&self.0
	}

	pub fn get_ft_identifier(&self) -> &Vec<u8> {
		&self.1
	}

	pub fn get_access_token(&self) -> &Vec<u8> {
		&self.2
	}

	pub fn get_account_id(&self) -> &Option<AccountId> {
		&self.3
	}

	pub fn get_data_type(&self) -> &Option<DataType> {
		&self.4
	}

	pub fn set_account_id(mut self, account_id: Option<AccountId>) -> Self {
		self.3 = account_id;
		self
	}

	pub fn set_data_type(mut self, data_type: Option<DataType>) -> Self {
		self.4 = data_type;
		self
	}
}

#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug, TypeInfo, Clone, Eq, PartialEq)]
pub struct UserSocialMedia {
	#[serde(deserialize_with = "de_string_to_bytes")]
	id: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	platform: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	created_at: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	updated_at: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	user_id: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	people_id: Vec<u8>,
	primary: bool,
	verified: bool,
}
impl UserSocialMedia {
	pub fn get_id(&self) -> &Vec<u8> {
		&self.id
	}

	pub fn get_people_id(&self) -> &Vec<u8> {
		&self.people_id
	}

	pub fn get_user_id(&self) -> &Vec<u8> {
		&self.user_id
	}
}

#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug, TypeInfo, Clone, Eq, PartialEq)]
pub struct Wallet {
	#[serde(deserialize_with = "de_string_to_bytes")]
	id: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	created_at: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	updated_at: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	user_id: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	network_id: Vec<u8>,
	primary: bool,
}
impl Wallet {
	pub fn get_id(&self) -> &Vec<u8> {
		&self.id
	}

	pub fn get_user_id(&self) -> &Vec<u8> {
		&self.user_id
	}
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, Default)]
pub struct UserCredential {
	nonce: u64,
	signature: Vec<u8>,
	user_id: Vec<u8>,
}
impl UserCredential {
	pub fn get_nonce(&self) -> &u64 {
		&self.nonce
	}

	pub fn get_signature(&self) -> &Vec<u8> {
		&self.signature
	}

	pub fn get_user_id(&self) -> &Vec<u8> {
		&self.user_id
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, Default)]
pub struct SocialMediaCredential {
	username: Vec<u8>,
	platform: Vec<u8>,
}
impl SocialMediaCredential {
	pub fn get_username(&self) -> &Vec<u8> {
		&self.username
	}

	pub fn get_platform(&self) -> &Vec<u8> {
		&self.platform
	}
}
