use crate::{Config, Pallet, Server, ServerById, ServerByOwner, ServerCount};
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

		let server = ServerById::<T>::get(b"myriad".to_vec());

		if let Some(server) = server {
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 3));

			let server_id = b"0";
			let new_server = Server::new(
				server_id,
				server.get_owner(),
				server.get_name(),
				server.get_api_url(),
				server.get_web_url(),
			);

			ServerByOwner::<T>::insert(
				server.get_owner().clone(),
				server_id.to_vec(),
				new_server.clone(),
			);
			ServerCount::<T>::set(1);
			ServerById::<T>::insert(server_id.to_vec(), new_server);
			ServerById::<T>::remove(b"myriad".to_vec());
		}

		weight
	}
}
