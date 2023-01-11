use crate::*;
use frame_support::{
	sp_runtime::traits::AccountIdConversion,
	traits::{Currency, ExistenceRequirement, Get},
	weights::Weight,
	PalletId,
};
use sp_std::vec::Vec;

const PALLET_ID: PalletId = PalletId(*b"Server!!");

impl<T: Config> Pallet<T> {
	pub fn server_account_id(server_id: ServerId) -> T::AccountId {
		PALLET_ID.into_sub_account_truncating(server_id)
	}

	pub fn do_api_url_exist(api_url: &[u8]) -> Result<(), Error<T>> {
		if Self::server_by_api_url(api_url).is_some() {
			return Err(Error::<T>::AlreadyExists)
		}

		Ok(())
	}

	pub fn do_balance_sufficient(
		account_id: &T::AccountId,
		amount: Option<BalanceOf<T>>,
	) -> Result<BalanceOf<T>, Error<T>> {
		let amount = if let Some(amount) = amount { amount } else { T::MinimumStakeAmount::get() };

		let minimum_balance = CurrencyOf::<T>::minimum_balance();
		let current_balance = CurrencyOf::<T>::free_balance(account_id);
		let transferable_balance = current_balance - minimum_balance;

		if amount > transferable_balance {
			return Err(Error::<T>::InsufficientBalance)
		}

		Ok(amount)
	}

	pub fn do_transfer(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		existence: ExistenceRequirement,
	) -> Result<(), Error<T>> {
		let result = CurrencyOf::<T>::transfer(sender, receiver, amount, existence);

		if result.is_err() {
			return Err(Error::<T>::BadSignature)
		}

		Ok(())
	}

	pub fn do_mutate_server(
		server_id: u64,
		owner: &T::AccountId,
		data: &ServerDataKind<T::AccountId, BalanceOf<T>>,
	) -> Result<ServerOf<T>, Error<T>> {
		ServerById::<T>::try_mutate(server_id, |result| match result {
			Some(server) => {
				if server.get_owner() != owner {
					return Err(Error::<T>::Unauthorized)
				}

				let updated_server = match data {
					ServerDataKind::Owner(new_owner) => {
						let server = server.clone().set_owner(new_owner);
						Ok(server)
					},
					ServerDataKind::ApiUrl(new_url) => {
						ServerByApiUrl::<T>::swap(server.get_api_url(), new_url);
						let server = server.clone().set_api_url(new_url);
						Ok(server)
					},
					ServerDataKind::StakeAmount(amount) => {
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
					ServerDataKind::UnstakeAmount(amount) => {
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

				ServerByOwner::<T>::insert(owner, server_id, &updated_server);

				*server = updated_server;

				Ok(server.clone())
			},
			None => Err(Error::<T>::NotExists),
		})
	}

	pub fn do_remove_servers(when: T::BlockNumber, tasks: Vec<ServerId>) -> Weight {
		let mut total_weight = Weight::zero();

		for server_id in tasks.iter() {
			let server_id = *server_id;
			let server = ServerById::<T>::get(server_id);

			if let Some(server) = server {
				let count = Self::server_count().saturating_sub(1);
				let sender = Self::server_account_id(server_id);
				let receiver = server.get_owner();
				let amount = CurrencyOf::<T>::free_balance(&sender);

				let existence = ExistenceRequirement::AllowDeath;
				let transfer = Self::do_transfer(&sender, receiver, amount, existence);

				if transfer.is_err() {
					Self::deposit_event(Event::Scheduled {
						server_id,
						when,
						task: b"Unstaked".to_vec(),
						status: Status::Failed,
					});
				} else {
					total_weight += T::WeightInfo::on_initialize_server();

					ServerCount::<T>::set(count);
					ServerById::<T>::remove(server_id);
					ServerByOwner::<T>::remove(receiver, server_id);
					ServerByApiUrl::<T>::remove(server.get_api_url());

					Self::deposit_event(Event::Unregistered(server_id));
					Self::deposit_event(Event::Unstaked(receiver.clone(), server_id, amount));
					Self::deposit_event(Event::Scheduled {
						server_id,
						when,
						task: b"Unstaked".to_vec(),
						status: Status::Success,
					});
				}
			} else {
				Self::deposit_event(Event::Scheduled {
					server_id,
					when,
					task: b"Unstaked".to_vec(),
					status: Status::Failed,
				});
			}
		}

		total_weight
	}
}
