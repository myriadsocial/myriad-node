use super::*;
use frame_support::traits::{ExistenceRequirement, Get};
use sp_std::vec::Vec;

impl<T: Config> ServerInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Server = ServerOf<T>;
	type Balance = BalanceOf<T>;
	type Action = ActionOf<T>;

	fn register(owner: &T::AccountId, api_url: &[u8]) -> Result<Self::Server, Self::Error> {
		Self::do_api_url_exist(api_url)?;

		let count = Self::server_count();
		let index = Self::server_index();
		let stake_amount = Self::do_balance_sufficient(owner, None)?;

		let server = Server::new(index, owner, api_url, stake_amount);

		let updated_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;
		let updated_index = index.checked_add(1).ok_or(Error::<T>::Overflow)?;

		let receiver = Self::server_account_id(index);

		Self::do_transfer(owner, &receiver, stake_amount, ExistenceRequirement::KeepAlive)?;

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

	fn unregister(server_id: u64, owner: &T::AccountId) -> Result<T::BlockNumber, Self::Error> {
		let server = ServerById::<T>::get(server_id)
			.ok_or(Error::<T>::NotExists)?
			.is_authorized(owner)
			.ok_or(Error::<T>::Unauthorized)?;

		let current_block_number = <frame_system::Pallet<T>>::block_number();
		let scheduled_block_number = current_block_number + T::ScheduledBlockTime::get();

		Tasks::<T>::mutate(scheduled_block_number, |tasks: &mut Vec<ServerId>| {
			if tasks.len() as u32 >= T::MaxScheduledPerBlock::get() {
				return Err(Error::<T>::FailedToSchedule)
			}

			tasks.push(server_id);

			Ok(())
		})?;

		let server = server.set_unstaked_at(scheduled_block_number);

		ServerById::<T>::insert(server_id, &server);
		ServerByOwner::<T>::insert(owner, server_id, &server);

		Ok(scheduled_block_number)
	}

	fn update_stake_amount(
		server_id: u64,
		owner: &T::AccountId,
		action: &Self::Action,
	) -> Result<(), Self::Error> {
		let server_data_kind = match action {
			Action::Stake(amount) => ServerDataKind::StakeAmount(*amount),
			Action::Unstake(amount) => ServerDataKind::UnstakeAmount(*amount),
		};

		Self::do_mutate_server(server_id, owner, &server_data_kind)?;

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
		ServerById::<T>::get(id)
	}
}
