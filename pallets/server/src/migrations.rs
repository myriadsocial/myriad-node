use crate::{
	AccountIdOf, Config, Pallet, Server as NewServer, ServerByApiUrl, ServerById as NewServerById,
	ServerByOwner as NewServerByOwner, ServerCount as NewServerCount, ServerId,
	ServerIndex as NewServerIndex, ServerOf,
};
use frame_support::{
	generate_storage_alias, pallet_prelude::*, traits::Get, weights::Weight, Blake2_128Concat,
};
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let mut weight: Weight = 0;
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(versions::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	if version == 1 {
		weight = weight.saturating_add(versions::v2::migrate::<T>());
		version = StorageVersion::new(2);
	}

	if version == 2 {
		weight = weight.saturating_add(versions::v3::migrate::<T>());
		version = StorageVersion::new(3);
	}

	if version == 3 {
		weight = weight.saturating_add(versions::v4::migrate::<T>());
		version = StorageVersion::new(4);
	}

	version.put::<Pallet<T>>();
	weight
}

mod versions {
	use super::*;

	pub mod v1 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[derive(Encode, Decode, Clone)]
			pub struct OldServer<AccountId> {
				pub id: Vec<u8>,
				pub owner: AccountId,
				pub name: Vec<u8>,
			}

			#[derive(Encode, Decode, Clone)]
			pub struct Server<AccountId> {
				pub id: Vec<u8>,
				pub owner: AccountId,
				pub name: Vec<u8>,
				pub api_url: Vec<u8>,
				pub web_url: Vec<u8>,
			}

			generate_storage_alias!(
				Server,
				ServerById<T: Config> => Map<(Blake2_128Concat, Vec<u8>), Server<AccountIdOf<T>>>
			);

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

	pub mod v2 {
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
				NewServerByOwner::<T>::insert(old.owner, 0, new_server);
				NewServerCount::<T>::set(1);

				None
			});

			weight
		}
	}

	pub mod v3 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			NewServerById::<T>::translate(|server_id: ServerId, old: ServerOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				if server_id == 0_u64 {
					return Some(old)
				}

				None
			});

			NewServerByOwner::<T>::translate(|_owner, server_id: ServerId, old: ServerOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				if server_id == 0_u64 {
					return Some(old)
				}

				None
			});

			weight = weight.saturating_add(T::DbWeight::get().writes(2));

			NewServerCount::<T>::set(1);
			NewServerIndex::<T>::set(1);

			weight
		}
	}

	pub mod v4 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			NewServerById::<T>::translate(|server_id: ServerId, server: ServerOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				ServerByApiUrl::<T>::insert(server.get_api_url(), server_id);

				Some(server)
			});

			weight
		}
	}
}
