use crate::{AccountIdOf, Config, Pallet, Server, ServerById};
use frame_support::{pallet_prelude::Decode, traits::Get, weights::Weight};
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let mut weight: Weight = 0;
	let version = StorageVersion::get::<Pallet<T>>();

	if version == 0 {
		weight = weight.saturating_add(v1::migrate::<T>());
		StorageVersion::new(1).put::<Pallet<T>>();
	}

	weight
}

mod v1 {
	use super::*;

	pub fn migrate<T: Config>() -> Weight {
		let mut weight = T::DbWeight::get().writes(1);

		#[derive(Decode)]
		pub struct OldServer<AccountId> {
			pub id: Vec<u8>,
			pub owner: AccountId,
			pub name: Vec<u8>,
		}

		ServerById::<T>::translate(|_key, old: OldServer<AccountIdOf<T>>| {
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
			Some(Server {
				id: old.id,
				owner: old.owner,
				name: old.name,
				api_url: "https://api.example.com".as_bytes().to_vec(),
				web_url: "https://web.example.com".as_bytes().to_vec(),
			})
		});

		weight
	}
}
