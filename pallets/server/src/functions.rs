use crate::*;

impl<T: Config> Pallet<T> {
	pub fn do_api_url_exist(api_url: &[u8]) -> Result<(), Error<T>> {
		if Self::server_by_api_url(api_url).is_some() {
			return Err(Error::<T>::AlreadyExists)
		}

		Ok(())
	}

	pub fn do_mutate_server(
		server_id: u64,
		owner: &T::AccountId,
		data: &ServerDataKind<T::AccountId>,
	) -> Result<(), Error<T>> {
		ServerById::<T>::try_mutate(server_id, |result| match result {
			Some(server) => {
				if server.get_owner() != owner {
					return Err(Error::<T>::Unauthorized)
				}

				let updated_server = match data {
					ServerDataKind::Owner(new_owner) => server.clone().set_owner(new_owner),
					ServerDataKind::ApiUrl(new_url) => {
						ServerByApiUrl::<T>::swap(server.get_api_url(), &new_url);
						server.clone().set_api_url(new_url)
					},
				};

				ServerByOwner::<T>::insert(owner, server_id, &updated_server);

				*server = updated_server;

				Ok(())
			},
			None => Err(Error::<T>::NotExists),
		})?;

		Ok(())
	}
}
