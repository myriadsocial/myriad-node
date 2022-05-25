use super::*;

use frame_support::{pallet_prelude::*, traits::Currency};
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct TipsBalance<Balance, AccountId> {
	tips_balance_info: TipsBalanceInfo,
	account_id: Option<AccountId>,
	amount: Balance,
}
impl<Balance: Clone, AccountId: Clone> TipsBalance<Balance, AccountId> {
	pub fn new(
		tips_balance_info: &TipsBalanceInfo,
		account_id: &Option<AccountId>,
		amount: &Balance,
	) -> Self {
		Self {
			tips_balance_info: tips_balance_info.clone(),
			account_id: account_id.clone(),
			amount: amount.clone(),
		}
	}

	pub fn get_tips_balance_info(&self) -> &TipsBalanceInfo {
		&self.tips_balance_info
	}

	pub fn get_amount(&self) -> &Balance {
		&self.amount
	}

	pub fn get_reference_id(&self) -> &Vec<u8> {
		self.tips_balance_info.get_reference_id()
	}

	pub fn get_reference_type(&self) -> &Vec<u8> {
		self.tips_balance_info.get_reference_type()
	}

	pub fn get_server_id(&self) -> &Vec<u8> {
		self.tips_balance_info.get_server_id()
	}

	pub fn get_ft_identifier(&self) -> &Vec<u8> {
		self.tips_balance_info.get_ft_identifier()
	}

	pub fn get_account_id(&self) -> &Option<AccountId> {
		&self.account_id
	}

	pub fn set_tips_balance_info(&mut self, tips_balance_info: &TipsBalanceInfo) {
		self.tips_balance_info = tips_balance_info.clone();
	}

	pub fn set_amount(&mut self, amount: Balance) {
		self.amount = amount;
	}

	pub fn set_account_id(&mut self, account_id: &Option<AccountId>) {
		self.account_id = account_id.clone();
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct TipsBalanceInfo {
	server_id: Vec<u8>,
	reference_type: Vec<u8>,
	reference_id: Vec<u8>,
	ft_identifier: Vec<u8>,
}
impl TipsBalanceInfo {
	pub fn new(
		server_id: &[u8],
		reference_type: &[u8],
		reference_id: &[u8],
		ft_identifier: &[u8],
	) -> Self {
		Self {
			server_id: server_id.to_vec(),
			reference_type: reference_type.to_vec(),
			reference_id: reference_id.to_vec(),
			ft_identifier: ft_identifier.to_vec(),
		}
	}

	pub fn get_reference_id(&self) -> &Vec<u8> {
		&self.reference_id
	}

	pub fn get_reference_type(&self) -> &Vec<u8> {
		&self.reference_type
	}

	pub fn get_server_id(&self) -> &Vec<u8> {
		&self.server_id
	}

	pub fn get_ft_identifier(&self) -> &Vec<u8> {
		&self.ft_identifier
	}

	pub fn set_reference_id(&mut self, reference_id: &[u8]) {
		self.reference_id = reference_id.to_vec();
	}

	pub fn set_reference_type(&mut self, reference_type: &[u8]) {
		self.reference_type = reference_type.to_vec();
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

#[derive(Serialize, Deserialize, RuntimeDebug, Clone)]
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
impl Default for UserSocialMedia {
	fn default() -> Self {
		Self {
			id: String::from("id"),
			verified: false,
			platform: String::from("platform"),
			primary: false,
			created_at: String::from("created_at"),
			updated_at: String::from("updated_at"),
			user_id: String::from("user_id"),
			people_id: String::from("people_id"),
		}
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
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum Status {
	OnProgress,
	Success,
	Failed,
}
impl Default for Status {
	fn default() -> Self {
		Status::OnProgress
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct IndexingData<AccountId>(pub(super) Vec<u8>, pub(super) Payload<AccountId>);

pub(super) type APIResult<T> = (AccountIdOf<T>, TipsBalanceInfo, UserSocialMedia, String);
pub(super) type FtIdentifier = Vec<u8>;
pub(super) type ReferenceId = Vec<u8>;
pub(super) type ReferenceType = Vec<u8>;
pub(super) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub(super) type CurrencyOf<T> = <T as self::Config>::Currency;
pub(super) type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub(super) type TipsBalanceOf<T> = TipsBalance<BalanceOf<T>, AccountIdOf<T>>;
pub(super) type ServerId = Vec<u8>;
