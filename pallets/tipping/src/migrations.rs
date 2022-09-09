use crate::{Config, Pallet, TipsBalance, TipsBalanceByReference, TipsBalanceOf};
use frame_support::{traits::Get, weights::Weight};

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let mut weight: Weight = 0;
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(version::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	version.put::<Pallet<T>>();
	weight
}

mod version {
	use super::*;

	pub mod v1 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			TipsBalanceByReference::<T>::translate_values(|tips_balance: TipsBalanceOf<T>| {
				if tips_balance.get_server_id() == b"myriad" {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					let amount = *tips_balance.get_amount();
					let new_tips_balance_info =
						tips_balance.get_tips_balance_info().clone().set_server_id(b"0");

					let mut new_tips_balance = TipsBalance::new(&new_tips_balance_info, &amount);
					let new_key = new_tips_balance.key();

					// Handle duplicate
					if let Some(tips_balance) = TipsBalanceByReference::<T>::get(&new_key) {
						new_tips_balance.add_amount(*tips_balance.get_amount());
					}

					if let Some(account_id) = new_tips_balance.get_account_id().clone() {
						new_tips_balance.set_account_id(&account_id);
					}

					TipsBalanceByReference::<T>::insert(&new_key, new_tips_balance);

					return None
				}

				Some(tips_balance)
			});

			weight
		}
	}
}
