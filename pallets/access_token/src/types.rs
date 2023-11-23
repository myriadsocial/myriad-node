use super::*;
use frame_support::{pallet_prelude::*, sp_runtime::traits::Saturating, traits::Currency};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub type TimelineId = Vec<u8>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum Scopes<TimelineId> {
	Login,
	Timeline(Vec<TimelineId>),
}
impl Default for Scopes<TimelineId> {
	fn default() -> Self {
		Self::Login
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct AccessToken<AccountId, Hash, TimelineId, Moment>
where
	Scopes<TimelineId>: Default,
{
	owner: AccountId,
	pub hash: Hash,
	pub scope: Scopes<TimelineId>,
	created_at: Moment,
	updated_at: Moment,
}
impl<AccountId, Hash, TimelineId, Moment: Copy> AccessToken<AccountId, Hash, TimelineId, Moment>
where
	Scopes<TimelineId>: Default,
	AccountId: PartialEq,
{
	pub fn new(
		owner: AccountId,
		hash: Hash,
		scope: Scopes<TimelineId>,
		created_at: Moment,
	) -> Self {
		Self { owner, hash, scope, created_at, updated_at: created_at }
	}

	pub fn is_authorized(self, owner: &AccountId) -> Option<Self> {
		if &self.owner == owner {
			Some(self)
		} else {
			None
		}
	}
}

pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
pub type AccessTokenOf<T> = AccessToken<AccountIdOf<T>, HashOf<T>, TimelineId, MomentOf<T>>;
