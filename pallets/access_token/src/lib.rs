#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod benchmarking;

pub use pallet::*;
pub use scale_info::TypeInfo;

pub mod functions;
pub mod impl_access_token;
pub mod interface;
pub mod types;
pub mod weights;

pub use crate::interface::AccessTokenInterface;
pub use types::*;
pub use weights::WeightInfo;

pub use frame_support::{
	debug, dispatch::DispatchResultWithPostInfo, pallet_prelude::*, sp_runtime::traits::Hash,
	traits::Randomness,
};
pub use sp_std::{fmt::Debug, prelude::*};

use frame_support::traits::StorageVersion;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{dispatch::DispatchResultWithPostInfo, traits::Get, Blake2_128Concat};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn access_token_count)]
	pub type AccessTokenCount<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn access_token_index)]
	pub type AccessTokenIndex<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn access_token_by_hash)]
	pub(super) type AccessTokenByHash<T: Config> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, AccessTokenOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn all_access_tokens_by_owner)]
	pub(super) type AccessTokenByOwner<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<AccessTokenOf<T>>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Create an access token success. [access_token]
		Created(AccessTokenOf<T>),
		/// Revoke access token success. [access_token]
		Revoked(AccessTokenOf<T>),
		/// Revoke all access token success. [access_token_list]
		RevokedAll(Vec<AccessTokenOf<T>>),
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyExists,
		NotExists,
		Unauthorized,
		Overflow,
		Underflow,
		BadSignature,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unregister())]
		pub fn create(
			origin: OriginFor<T>,
			hash: HashOf<T>,
			scopes: Scopes<TimelineId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as AccessTokenInterface<T>>::create(&who, &hash, &scopes) {
				Ok(access_token) => {
					Self::deposit_event(Event::Created(access_token));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unregister())]
		pub fn revoke(origin: OriginFor<T>, hash: HashOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as AccessTokenInterface<T>>::revoke(&who, &hash) {
				Ok(access_token) => {
					Self::deposit_event(Event::Revoked(access_token));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unregister())]
		pub fn revoke_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as AccessTokenInterface<T>>::revoke_all(&who) {
				Ok(access_tokens) => {
					Self::deposit_event(Event::RevokedAll(access_tokens));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unregister())]
		pub fn revoke_all_by_scopes(
			origin: OriginFor<T>,
			scopes: Scopes<TimelineId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as AccessTokenInterface<T>>::revoke_all_by_scopes(&who, &scopes) {
				Ok(access_tokens) => {
					Self::deposit_event(Event::RevokedAll(access_tokens));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
