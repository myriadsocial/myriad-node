use super::*;

impl<T: Config> ServerInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Server = ServerOf<T>;

	fn get_by_id(server_id: u64) -> Option<Self::Server> {
		Self::server_by_id(server_id)
	}

	fn register(owner: &T::AccountId, api_url: &[u8]) -> Result<Self::Server, Self::Error> {
		let count = Self::server_count();
		let server = Server::new(count, owner, api_url);

		Self::do_set_server(true, &OperatorKind::Add, &server)?;

		Ok(server)
	}

	fn transfer_owner(
		server_id: u64,
		owner: &T::AccountId,
		new_owner: &T::AccountId,
	) -> Result<(), Self::Error> {
		Self::do_mutate_server(server_id, owner, &ServerDataKind::Owner(new_owner.clone()))?;

		Ok(())
	}

	fn update_api_url(
		server_id: u64,
		owner: &T::AccountId,
		new_api_url: &[u8],
	) -> Result<(), Self::Error> {
		Self::do_mutate_server(server_id, owner, &ServerDataKind::ApiUrl(new_api_url.to_vec()))?;

		Ok(())
	}

	fn unregister(server_id: u64, owner: &T::AccountId) -> Result<(), Self::Error> {
		let server = Self::can_update_server(server_id, owner)?;

		Self::do_set_server(false, &OperatorKind::Sub, &server)?;

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
