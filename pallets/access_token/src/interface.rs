use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec::Vec;

pub trait AccessTokenInterface<T: frame_system::Config> {
	type Error;
	type AccessToken;
	type Scopes;

	fn create(
		owner: &T::AccountId,
		hash: &T::Hash,
		scopes: &Self::Scopes,
	) -> Result<Self::AccessToken, Self::Error>;

	fn revoke(owner: &T::AccountId, hash: &T::Hash) -> Result<Self::AccessToken, Self::Error>;

	fn revoke_all(owner: &T::AccountId) -> Result<Vec<Self::AccessToken>, Self::Error>;

	fn revoke_all_by_scopes(
		owner: &T::AccountId,
		scopes: &Self::Scopes,
	) -> Result<Vec<Self::AccessToken>, Self::Error>;
}
