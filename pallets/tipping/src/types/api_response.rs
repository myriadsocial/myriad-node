use crate::*;

use frame_support::pallet_prelude::*;
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

pub type APIResult<T> = (AccountIdOf<T>, TipsBalanceInfo, UserSocialMedia, String);

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
