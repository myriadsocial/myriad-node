use crate::*;

use frame_support::pallet_prelude::*;
use serde::{Deserialize, Serialize};
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

	pub fn set_account_id(mut self, account_id: AccountId) -> Self {
		self.3 = Some(account_id);
		self
	}

	pub fn set_data_type(mut self, data_type: DataType) -> Self {
		self.4 = Some(data_type);
		self
	}
}

#[derive(Serialize, Deserialize, RuntimeDebug, Clone, Default)]
pub struct UserSocialMedia {
	id: String,
	verified: bool,
	platform: String,
	primary: bool,
	created_at: String,
	updated_at: String,
	user_id: String,
	people_id: String,
}
impl UserSocialMedia {
	pub fn get_id(&self) -> &str {
		&self.id
	}

	pub fn get_user_id(&self) -> &str {
		&self.user_id
	}

	pub fn get_people_id(&self) -> &str {
		&self.people_id
	}
}

#[derive(Serialize, Deserialize, RuntimeDebug, Clone, Default)]
pub struct Wallet {
	id: String,
	primary: bool,
	created_at: String,
	updated_at: String,
	user_id: String,
	network_id: String,
}
impl Wallet {
	pub fn get_id(&self) -> &str {
		&self.id
	}

	pub fn get_user_id(&self) -> &str {
		&self.user_id
	}

	pub fn get_network_id(&self) -> &str {
		&self.network_id
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, Default)]
pub struct UserSocialMediaInfo {
	id: Vec<u8>,
	verified: bool,
	platform: Vec<u8>,
	primary: bool,
	created_at: Vec<u8>,
	updated_at: Vec<u8>,
	user_id: Vec<u8>,
	people_id: Vec<u8>,
}
impl UserSocialMediaInfo {
	pub fn new(user_social_media: &UserSocialMedia) -> Self {
		let user_social_media = user_social_media.clone();
		let result = Self::default();

		result
			.id(user_social_media.get_id())
			.verified(user_social_media.verified)
			.platform(&user_social_media.platform)
			.primary(user_social_media.primary)
			.created_at(&user_social_media.created_at)
			.updated_at(&user_social_media.updated_at)
			.user_id(user_social_media.get_user_id())
			.people_id(user_social_media.get_people_id())
	}

	pub fn id(mut self, id: &str) -> Self {
		self.id = id.as_bytes().to_vec();
		self
	}

	pub fn verified(mut self, verified: bool) -> Self {
		self.verified = verified;
		self
	}

	pub fn platform(mut self, platform: &str) -> Self {
		self.platform = platform.as_bytes().to_vec();
		self
	}

	pub fn primary(mut self, primary: bool) -> Self {
		self.primary = primary;
		self
	}

	pub fn created_at(mut self, created_at: &str) -> Self {
		self.created_at = created_at.as_bytes().to_vec();
		self
	}

	pub fn updated_at(mut self, updated_at: &str) -> Self {
		self.updated_at = updated_at.as_bytes().to_vec();
		self
	}

	pub fn user_id(mut self, user_id: &str) -> Self {
		self.user_id = user_id.as_bytes().to_vec();
		self
	}

	pub fn people_id(mut self, people_id: &str) -> Self {
		self.people_id = people_id.as_bytes().to_vec();
		self
	}

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

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, Default)]
pub struct WalletInfo {
	id: Vec<u8>,
	primary: bool,
	created_at: Vec<u8>,
	updated_at: Vec<u8>,
	user_id: Vec<u8>,
	network_id: Vec<u8>,
}
impl WalletInfo {
	pub fn new(wallet: &Wallet) -> Self {
		let wallet = wallet.clone();
		let result = Self::default();

		result
			.id(wallet.get_id())
			.primary(wallet.primary)
			.created_at(&wallet.created_at)
			.updated_at(&wallet.updated_at)
			.user_id(wallet.get_user_id())
			.network_id(wallet.get_network_id())
	}

	pub fn id(mut self, id: &str) -> Self {
		self.id = id.as_bytes().to_vec();
		self
	}

	pub fn primary(mut self, primary: bool) -> Self {
		self.primary = primary;
		self
	}

	pub fn created_at(mut self, created_at: &str) -> Self {
		self.created_at = created_at.as_bytes().to_vec();
		self
	}

	pub fn updated_at(mut self, updated_at: &str) -> Self {
		self.updated_at = updated_at.as_bytes().to_vec();
		self
	}

	pub fn user_id(mut self, user_id: &str) -> Self {
		self.user_id = user_id.as_bytes().to_vec();
		self
	}

	pub fn network_id(mut self, network_id: &str) -> Self {
		self.network_id = network_id.as_bytes().to_vec();
		self
	}

	pub fn get_id(&self) -> &Vec<u8> {
		&self.id
	}

	pub fn get_user_id(&self) -> &Vec<u8> {
		&self.user_id
	}
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
