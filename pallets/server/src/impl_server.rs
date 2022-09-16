use super::*;

impl<T: Config> ServerInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Server = ServerOf<T>;

	fn get_by_id(server_id: u64) -> Option<Self::Server> {
		Self::server_by_id(server_id)
	}

	fn register(owner: &T::AccountId, api_url: &[u8]) -> Result<Self::Server, Self::Error> {
		Self::do_api_url_exist(api_url)?;

		let count = Self::server_count();
		let index = Self::server_index();

		let server = Server::new(index, owner, api_url);

		let updated_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;
		let updated_index = index.checked_add(1).ok_or(Error::<T>::Overflow)?;

		ServerCount::<T>::set(updated_count);
		ServerIndex::<T>::set(updated_index);
		ServerById::<T>::insert(index, &server);
		ServerByApiUrl::<T>::insert(api_url, index);
		ServerByOwner::<T>::insert(owner, index, &server);

		Ok(server)
	}

	fn transfer_owner(
		server_id: u64,
		owner: &T::AccountId,
		new_owner: &T::AccountId,
	) -> Result<(), Self::Error> {
		Self::do_mutate_server(server_id, owner, &ServerDataKind::Owner(new_owner.clone()))?;

		ServerByOwner::<T>::swap(owner, server_id, new_owner, server_id);

		Ok(())
	}

	fn update_api_url(
		server_id: u64,
		owner: &T::AccountId,
		new_api_url: &[u8],
	) -> Result<(), Self::Error> {
		Self::do_api_url_exist(new_api_url)?;
		Self::do_mutate_server(server_id, owner, &ServerDataKind::ApiUrl(new_api_url.to_vec()))?;

		Ok(())
	}

	fn unregister(server_id: u64, owner: &T::AccountId) -> Result<(), Self::Error> {
		let server =
			<Self as ServerInterface<T>>::get_by_id(server_id).ok_or(Error::<T>::NotExists)?;

		let current_owner = server.get_owner();

		if current_owner != owner {
			return Err(Error::<T>::Unauthorized)
		}

		let count = Self::server_count().checked_sub(1).ok_or(Error::<T>::Overflow)?;

		ServerCount::<T>::set(count);
		ServerById::<T>::remove(server_id);
		ServerByOwner::<T>::remove(owner, server_id);
		ServerByApiUrl::<T>::remove(server.get_api_url());

		Ok(())
	}
}

impl<T: Config> ServerProvider<T> for Pallet<T>
where
	ServerOf<T>: ServerInfo<T>,
{
	type Error = Error<T>;
	type Server = ServerOf<T>;

	fn get_by_id(id: u64) -> Option<ServerOf<T>> {
		<Self as ServerInterface<T>>::get_by_id(id)
	}
}
