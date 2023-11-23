use super::*;
use frame_support::traits::{ExistenceRequirement, Get};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec::Vec;

impl<T: Config + pallet_timestamp::Config> AccessTokenInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type AccessToken = AccessTokenOf<T>;
	type Scopes = Scopes<TimelineId>;

	fn create(
		owner: &T::AccountId,
		hash: &T::Hash,
		scopes: &Self::Scopes,
	) -> Result<Self::AccessToken, Self::Error> {
		Self::do_hash_exist(hash)?;

		let count = Self::access_token_count();
		let index = Self::access_token_index();

		let access_token =
			AccessToken::new(owner.clone(), *hash, scopes.clone(), MomentOf::<T>::default());

		let updated_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;
		let updated_index = index.checked_add(1).ok_or(Error::<T>::Overflow)?;

		AccessTokenCount::<T>::set(updated_count);
		AccessTokenIndex::<T>::set(updated_index);
		AccessTokenByHash::<T>::insert(hash, access_token.clone());

		let mut access_token_list = AccessTokenByOwner::<T>::get(owner).unwrap_or_default();
		access_token_list.push(access_token.clone());
		AccessTokenByOwner::<T>::insert(owner, &access_token_list);

		Ok(access_token)
	}

	fn revoke(owner: &T::AccountId, hash: &T::Hash) -> Result<Self::AccessToken, Self::Error> {
		let access_token = AccessTokenByHash::<T>::get(hash)
			.ok_or(Error::<T>::NotExists)?
			.is_authorized(owner)
			.ok_or(Error::<T>::Unauthorized)?;

		// Get access_token from storage
		let access_token = AccessTokenByHash::<T>::take(hash.clone());

		if let Some(ref mut access_token_list) = AccessTokenByOwner::<T>::get(owner) {
			access_token_list.retain(|x| x.hash != *hash);
			AccessTokenByOwner::<T>::insert(owner, access_token_list);
		}

		let count = Self::access_token_count();
		let updated_count = count.checked_sub(1).ok_or(Error::<T>::Underflow)?;
		AccessTokenCount::<T>::set(updated_count);

		Ok(access_token.unwrap())
	}

	fn revoke_all(owner: &T::AccountId) -> Result<Vec<Self::AccessToken>, Self::Error> {
		let mut access_token_list =
			AccessTokenByOwner::<T>::get(owner).ok_or(Error::<T>::NotExists)?;

		// Get access_token from storage
		for access_token in &access_token_list {
			AccessTokenByHash::<T>::take(access_token.hash);

			let count = Self::access_token_count();
			let updated_count = count.checked_sub(1).ok_or(Error::<T>::Underflow)?;
			AccessTokenCount::<T>::set(updated_count);
		}

		access_token_list = AccessTokenByOwner::<T>::take(owner).unwrap();
		Ok(access_token_list)
	}

	fn revoke_all_by_scopes(
		owner: &T::AccountId,
		scope: &Self::Scopes,
	) -> Result<Vec<Self::AccessToken>, Self::Error> {
		let mut access_token_list =
			AccessTokenByOwner::<T>::get(owner).ok_or(Error::<T>::NotExists)?;
		access_token_list.retain(|x| x.scope == scope.clone());

		for access_token in &access_token_list {
			AccessTokenByHash::<T>::take(access_token.hash);

			let count = Self::access_token_count();
			let updated_count = count.checked_sub(1).ok_or(Error::<T>::Underflow)?;
			AccessTokenCount::<T>::set(updated_count);
		}

		let mut new_access_token_list = AccessTokenByOwner::<T>::get(owner).unwrap();
		new_access_token_list.retain(|x| x.scope != scope.clone());
		AccessTokenByOwner::<T>::insert(owner, new_access_token_list);

		Ok(access_token_list)
	}
}
