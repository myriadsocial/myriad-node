use crate::*;
use scale_info::prelude::string::ToString;

impl<T: Config> Pallet<T> {
	pub fn can_update_server(
		server_id: u64,
		account_id: &T::AccountId,
	) -> Result<ServerOf<T>, Error<T>> {
		let formatted_id = server_id.to_string().as_bytes().to_vec();
		let server =
			<Self as ServerInterface<T>>::get_by_id(&formatted_id).ok_or(Error::<T>::NotExists)?;

		let current_owner = server.get_owner();

		if current_owner != account_id {
			return Err(Error::<T>::Unauthorized)
		}

		Ok(server)
	}

	pub fn do_update_server(server: &Server<AccountIdOf<T>>, unreg: bool, count: Option<u64>) {
		let server_id = server.get_id();
		let owner = server.get_owner();

		if unreg {
			ServerById::<T>::remove(server_id);
			ServerByOwner::<T>::remove(owner, server_id);
		} else {
			ServerById::<T>::insert(server_id, server);
			ServerByOwner::<T>::insert(owner, server_id, server);
		}

		if let Some(count) = count {
			ServerCount::<T>::set(count);
		}
	}
}
