use crate::{Config, Pallet, TipsBalanceByReference, TipsBalanceOf};
use frame_support::{traits::Get, weights::Weight};
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let mut weight: Weight = 0;
	let version = StorageVersion::get::<Pallet<T>>();

	if version == 1 {
		weight = weight.saturating_add(v2::migrate::<T>());
		StorageVersion::new(2).put::<Pallet<T>>();
	}

	weight
}

mod v2 {
	use super::*;

	pub fn migrate<T: Config>() -> Weight {
		let mut weight = T::DbWeight::get().writes(1);
		let mut tips_balances: Vec<TipsBalanceOf<T>> = Vec::new();

		for key in TipsBalanceByReference::<T>::iter_keys() {
			if let Some(mut tips_balance) = TipsBalanceByReference::<T>::get(key.clone()) {
				let tips_balance_info =
					tips_balance.get_tips_balance_info().clone().set_server_id(b"0");

				tips_balance.set_tips_balance_info(&tips_balance_info);
				tips_balances.push(tips_balance);

				TipsBalanceByReference::<T>::remove(key);
			}
		}

		for tips_balance in tips_balances.iter() {
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

			let key = tips_balance.key();

			TipsBalanceByReference::<T>::insert(key, tips_balance);
		}

		weight
	}
}
