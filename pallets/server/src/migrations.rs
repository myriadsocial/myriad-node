use crate::{
	AccountIdOf, Config, Pallet, Server as NewServer, ServerById as NewServerById, ServerByOwner,
	ServerCount,
};
use frame_support::{
	generate_storage_alias, pallet_prelude::*, traits::Get, weights::Weight, Blake2_128Concat,
};
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
		#[derive(Encode, Decode, Clone)]
		pub struct OldServer<AccountId> {
			id: Vec<u8>,
			owner: AccountId,
			name: Vec<u8>,
			api_url: Vec<u8>,
			web_url: Vec<u8>,
		}

		generate_storage_alias!(
			Server,
			ServerById<T: Config> => Map<(Blake2_128Concat, Vec<u8>), OldServer<AccountIdOf<T>>>
		);

		ServerById::<T>::translate(|_key, old: OldServer<AccountIdOf<T>>| {
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

			let new_server = NewServer::new(0, &old.owner, &old.api_url);

			NewServerById::<T>::insert(0, new_server.clone());
			ServerByOwner::<T>::insert(old.owner, 0, new_server);
			ServerCount::<T>::set(1);

			None
		});

		weight
	}
}
