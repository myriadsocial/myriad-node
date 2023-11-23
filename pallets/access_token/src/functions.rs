use crate::*;
use frame_support::{
	sp_runtime::{
		traits::{AccountIdConversion, Zero},
		DispatchError,
	},
	traits::{Currency, ExistenceRequirement},
	weights::Weight,
	PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec::Vec;

const PALLET_ID: PalletId = PalletId(*b"AccTkn!!");

impl<T: Config> Pallet<T> {
	pub fn do_hash_exist(hash: &T::Hash) -> Result<(), Error<T>> {
		if Self::access_token_by_hash(hash).is_some() {
			return Err(Error::<T>::AlreadyExists);
		}

		Ok(())
	}
}
