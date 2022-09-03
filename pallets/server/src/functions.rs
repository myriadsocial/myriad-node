use crate::*;

impl<T: Config> Pallet<T> {
	pub fn can_update_server(
		server_id: u64,
		account_id: &T::AccountId,
	) -> Result<ServerOf<T>, Error<T>> {
		let server =
			<Self as ServerInterface<T>>::get_by_id(server_id).ok_or(Error::<T>::NotExists)?;

		let current_owner = server.get_owner();

		if current_owner != account_id {
			return Err(Error::<T>::Unauthorized)
		}

		Ok(server)
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

				*server = match data {
					ServerDataKind::Owner(new_owner) => server.clone().set_owner(new_owner),
					ServerDataKind::ApiUrl(new_url) => server.clone().set_api_url(new_url),
				};

				Ok(())
			},
			None => Err(Error::<T>::NotExists),
		})?;

		Ok(())
	}

	pub fn do_set_server(
		register: bool,
		operator: &OperatorKind,
		server: &ServerOf<T>,
	) -> Result<(), Error<T>> {
		ServerCount::<T>::try_mutate(|value| {
			let result = match operator {
				OperatorKind::Add => value.checked_add(1),
				OperatorKind::Sub => value.checked_sub(1),
			};

			let total_value = result.ok_or(Error::<T>::Overflow)?;

			*value = total_value;

			Ok(())
		})?;

		let server_id = server.get_id();
		let owner = server.get_owner();

		if register {
			ServerById::<T>::insert(server_id, server);
			ServerByOwner::<T>::insert(owner, server_id, server);
		} else {
			ServerById::<T>::remove(server_id);
			ServerByOwner::<T>::remove(owner, server_id);
		}

		Ok(())
	}
}
