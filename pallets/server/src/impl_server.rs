use super::*;

impl<T: Config> ServerInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Server = ServerOf<T>;

	fn get_by_id(server_id: &[u8]) -> Option<Self::Server> {
		Self::server_by_id(server_id)
	}

	fn register(
		server_id: &[u8],
		account_id: &T::AccountId,
		name: &[u8],
		api_url: &[u8],
		web_url: &[u8],
	) -> Result<Self::Server, Self::Error> {
		if ServerById::<T>::contains_key(server_id) {
			return Err(Error::<T>::AlreadyExists)
		}
		let server = Server::new(server_id, account_id, name, api_url, web_url);

		ServerById::<T>::insert(server_id, server.clone());

		Ok(server)
	}

	fn transfer_owner(
		server_id: &[u8],
		account_id: &T::AccountId,
		new_owner: &T::AccountId,
	) -> Result<(), Self::Error> {
		if !ServerById::<T>::contains_key(server_id) {
			return Err(Error::<T>::NotExists)
		}

		let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();
		let current_owner = server.get_owner();

		if current_owner != account_id {
			return Err(Error::<T>::Unauthorized)
		}

		if current_owner == new_owner {
			return Ok(())
		}

		server.set_owner(new_owner);

		ServerById::<T>::insert(server_id, server);

		Ok(())
	}

	fn update_name(
		server_id: &[u8],
		account_id: &T::AccountId,
		new_name: &[u8],
	) -> Result<(), Self::Error> {
		if !ServerById::<T>::contains_key(server_id) {
			return Err(Error::<T>::NotExists)
		}

		let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();

		if server.get_owner() != account_id {
			return Err(Error::<T>::Unauthorized)
		}

		server.set_name(new_name);

		ServerById::<T>::insert(server_id, server);

		Ok(())
	}

	fn update_api_url(
		server_id: &[u8],
		account_id: &T::AccountId,
		new_api_url: &[u8],
	) -> Result<(), Self::Error> {
		if !ServerById::<T>::contains_key(server_id) {
			return Err(Error::<T>::NotExists)
		}

		let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();

		if server.get_owner() != account_id {
			return Err(Error::<T>::Unauthorized)
		}

		server.set_api_url(new_api_url);

		ServerById::<T>::insert(server_id, server);

		Ok(())
	}

	fn update_web_url(
		server_id: &[u8],
		account_id: &T::AccountId,
		new_web_url: &[u8],
	) -> Result<(), Self::Error> {
		if !ServerById::<T>::contains_key(server_id) {
			return Err(Error::<T>::NotExists)
		}

		let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();

		if server.get_owner() != account_id {
			return Err(Error::<T>::Unauthorized)
		}

		server.set_web_url(new_web_url);

		ServerById::<T>::insert(server_id, server);

		Ok(())
	}

	fn unregister(server_id: &[u8], account_id: &T::AccountId) -> Result<(), Self::Error> {
		if !ServerById::<T>::contains_key(server_id) {
			return Err(Error::<T>::NotExists)
		}

		let server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();

		if server.get_owner() != account_id {
			return Err(Error::<T>::Unauthorized)
		}

		ServerById::<T>::remove(server_id);

		Ok(())
	}
}

impl<T: Config> ServerProvider<T> for Pallet<T>
where
	ServerOf<T>: ServerInfo<T>,
{
	type Error = Error<T>;
	type Server = ServerOf<T>;

	fn get_by_id(id: &[u8]) -> Option<ServerOf<T>> {
		<Self as ServerInterface<T>>::get_by_id(id)
	}
}
