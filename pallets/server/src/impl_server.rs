use super::*;
use frame_support::traits::{ExistenceRequirement, Get};
use sp_std::vec::Vec;

impl<T: Config> ServerInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Server = ServerOf<T>;
	type Balance = BalanceOf<T>;
	type ActionType = ActionTypeOf<T>;

	fn register(
		owner: &T::AccountId,
		api_url: &[u8],
		stake_amount: Option<Self::Balance>,
	) -> Result<Self::Server, Self::Error> {
		Self::do_api_url_exist(api_url)?;

		let count = Self::server_count();
		let index = Self::server_index();

		let minimum_amount = T::MinimumStakeAmount::get();
		let stake_amount = if let Some(amount) = stake_amount { amount } else { minimum_amount };

		if stake_amount < minimum_amount {
			return Err(Error::<T>::MinimumStakeLimitBalance)
		}

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

	fn update_server(
		server_id: u64,
		owner: &T::AccountId,
		action: &Self::ActionType,
	) -> Result<(), Self::Error> {
		ServerById::<T>::try_mutate(server_id, |result| match result {
			Some(server) => {
				if server.get_owner() != owner {
					return Err(Error::<T>::Unauthorized)
				}

				if server.get_unstaked_at().is_some() {
					return Err(Error::<T>::WaitingToUnstaked)
				}

				let updated_server = match action {
					ActionType::TransferOwner(new_owner) => {
						let server = server.clone().set_owner(new_owner);

						ServerByOwner::<T>::swap(owner, server_id, new_owner, server_id);
						Ok(server)
					},
					ActionType::UpdateApiUrl(new_url) => {
						Self::do_api_url_exist(new_url)?;

						ServerByApiUrl::<T>::swap(server.get_api_url(), new_url);
						let server = server.clone().set_api_url(new_url);
						Ok(server)
					},
					ActionType::StakeAmount(amount) => {
						let receiver = Self::server_account_id(server_id);

						Self::do_transfer(
							owner,
							&receiver,
							*amount,
							ExistenceRequirement::KeepAlive,
						)?;

						let server = server.clone().increase_stake_amount(*amount);
						Ok(server)
					},
					ActionType::UnstakeAmount(amount) => {
						let sender = Self::server_account_id(server_id);
						let current_stake_amount = server.get_stake_amount();

						if amount.gt(current_stake_amount) {
							return Err(Error::<T>::InsufficientBalance)
						}

						if *current_stake_amount - *amount < T::MinimumStakeAmount::get() {
							return Err(Error::<T>::UnstakingLimitBalance)
						}

						Self::do_transfer(
							&sender,
							owner,
							*amount,
							ExistenceRequirement::KeepAlive,
						)?;

						let server = server.clone().decrease_stake_amount(*amount);
						Ok(server)
					},
				}?;

				ServerByOwner::<T>::insert(updated_server.get_owner(), server_id, &updated_server);

				*server = updated_server;

				Ok(server.clone())
			},
			None => Err(Error::<T>::NotExists),
		})?;

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

		let server = server.set_unstaked_at(Some(scheduled_block_number));

		ServerById::<T>::insert(server_id, &server);
		ServerByOwner::<T>::insert(owner, server_id, &server);

		Ok(scheduled_block_number)
	}

	fn cancel_unregister(
		server_id: u64,
		owner: &T::AccountId,
	) -> Result<T::BlockNumber, Self::Error> {
		let server = ServerById::<T>::get(server_id)
			.ok_or(Error::<T>::NotExists)?
			.is_authorized(owner)
			.ok_or(Error::<T>::Unauthorized)?;

		let unstaked_at = server.get_unstaked_at();

		if unstaked_at.is_none() {
			return Err(Error::<T>::NotExists)
		}

		let unstaked_at = unstaked_at.unwrap();

		let mut tasks = Vec::new();

		for e in Tasks::<T>::take(unstaked_at).iter() {
			if *e != server_id {
				tasks.push(*e);
			}
		}

		let server = server.set_unstaked_at(None);

		ServerById::<T>::insert(server_id, &server);
		ServerByOwner::<T>::insert(owner, server_id, &server);

		if tasks.len() > 0 {
			Tasks::<T>::insert(unstaked_at, tasks);
		}

		Ok(unstaked_at)
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
