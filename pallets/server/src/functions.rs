use crate::*;
use frame_support::{
	sp_runtime::{
		traits::{AccountIdConversion, Zero},
		DispatchError,
	},
	traits::{Currency, ExistenceRequirement},
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

	pub fn do_transfer(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		existence: ExistenceRequirement,
	) -> Result<(), Error<T>> {
		let result = CurrencyOf::<T>::transfer(sender, receiver, amount, existence);

		if let Err(err) = result {
			return match err {
				DispatchError::Module(_) => Err(Error::<T>::InsufficientBalance),
				DispatchError::Arithmetic(_) => Err(Error::<T>::InsufficientBalance),
				_ => Err(Error::<T>::BadSignature),
			}
		}

		Ok(())
	}

	pub fn do_remove_servers(when: T::BlockNumber, tasks: Vec<ServerId>) -> Weight {
		let mut total_weight = Weight::zero();

		for server_id in tasks.iter() {
			let server_id = *server_id;
			let server = ServerById::<T>::get(server_id);

			if let Some(server) = server {
				let server = server.set_stake_amount(Zero::zero());
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
					ServerByApiUrl::<T>::remove(server.get_api_url());
					ServerByOwner::<T>::insert(receiver, server_id, &server);

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
