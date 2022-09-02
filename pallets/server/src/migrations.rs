use crate::{AccountIdOf, Config, Pallet, Server, ServerById, ServerByOwner, ServerCount};
use frame_support::{pallet_prelude::Decode, traits::Get, weights::Weight};
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

		#[allow(dead_code)]
		#[derive(Decode)]
		pub struct OldServer<AccountId> {
			id: Vec<u8>,
			owner: AccountId,
			name: Vec<u8>,
			api_url: Vec<u8>,
			web_url: Vec<u8>,
		}

		ServerById::<T>::swap(b"myriad".to_vec(), b"0".to_vec());
		ServerById::<T>::translate(|_key, server: OldServer<AccountIdOf<T>>| {
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

			let owner = server.owner;
			let api_url = server.api_url;
			let new_server = Server::new(b"0", &owner, &api_url);

			ServerCount::<T>::set(1);
			ServerByOwner::<T>::insert(owner, b"0".to_vec(), new_server.clone());

			Some(new_server)
		});

		weight
	}
}
