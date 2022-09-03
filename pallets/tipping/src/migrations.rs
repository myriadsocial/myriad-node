use crate::{Config, Pallet, TipsBalance, TipsBalanceByReference, TipsBalanceOf};
use frame_support::{traits::Get, weights::Weight};

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

		TipsBalanceByReference::<T>::translate_values(|tips_balance: TipsBalanceOf<T>| {
			if tips_balance.get_server_id() == b"myriad" {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let tips_balance_info =
					tips_balance.get_tips_balance_info().clone().set_server_id(b"0");
				let mut new_tips_balance =
					TipsBalance::new(&tips_balance_info, tips_balance.get_amount());

				if let Some(account_id) = tips_balance.get_account_id().as_ref() {
					new_tips_balance.set_account_id(account_id);
				}

				TipsBalanceByReference::<T>::insert(new_tips_balance.key(), new_tips_balance);

				return None
			}

			Some(tips_balance)
		});

		weight
	}
}
