use super::*;
use scale_info::prelude::string::ToString;

impl<T: Config> ServerInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Server = ServerOf<T>;

	fn get_by_id(server_id: &[u8]) -> Option<Self::Server> {
		Self::server_by_id(server_id)
	}

	fn register(
		owner: &T::AccountId,
		name: &[u8],
		api_url: &[u8],
		web_url: &[u8],
	) -> Result<Self::Server, Self::Error> {
		let count = Self::server_count();
		let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

		let server_id = count.to_string().as_bytes().to_vec();
		let server = Server::new(&server_id, owner, name, api_url, web_url);

		Self::do_update_server(&server, false, Some(new_count));

		Ok(server)
	}

	fn transfer_owner(
		server_id: u64,
		owner: &T::AccountId,
		new_owner: &T::AccountId,
	) -> Result<(), Self::Error> {
		let server = Self::can_update_server(server_id, owner)?.set_owner(new_owner);

		Self::do_update_server(&server, false, None);

		Ok(())
	}

	fn update_name(
		server_id: u64,
		owner: &T::AccountId,
		new_name: &[u8],
	) -> Result<(), Self::Error> {
		let server = Self::can_update_server(server_id, owner)?.set_name(new_name);

		Self::do_update_server(&server, false, None);

		Ok(())
	}

	fn update_api_url(
		server_id: u64,
		owner: &T::AccountId,
		new_api_url: &[u8],
	) -> Result<(), Self::Error> {
		let server = Self::can_update_server(server_id, owner)?.set_api_url(new_api_url);

		Self::do_update_server(&server, false, None);

		Ok(())
	}

	fn update_web_url(
		server_id: u64,
		owner: &T::AccountId,
		new_web_url: &[u8],
	) -> Result<(), Self::Error> {
		let server = Self::can_update_server(server_id, owner)?.set_web_url(new_web_url);

		Self::do_update_server(&server, false, None);

		Ok(())
	}

	fn unregister(server_id: u64, owner: &T::AccountId) -> Result<(), Self::Error> {
		let server = Self::can_update_server(server_id, owner)?;
		let count = Self::server_count().checked_sub(1).ok_or(Error::<T>::Overflow)?;

		Self::do_update_server(&server, true, Some(count));

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
