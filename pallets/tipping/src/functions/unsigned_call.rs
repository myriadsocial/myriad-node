use crate::*;

use frame_support::sp_runtime::offchain::http;
use frame_system::offchain::SubmitTransaction;

impl<T: Config> Pallet<T> {
	pub fn handle_myriad_api(
		block_number: T::BlockNumber,
	) -> Result<Option<APIResult<T>>, http::Error> {
		let data = Self::get_indexing_data(block_number);

		if data.is_none() {
			return Ok(None)
		}

		let data = data.unwrap();
		let payload = data.get_payload();
		let data_id = data.get_data_id().clone();
		let data_type = data.get_data_type().clone();
		let payload_type = payload.get_payload_type();

		let mut event: Option<Event<T>> = None;
		let result = match payload_type {
			PayloadType::Create => {
				let created = Self::create_user_social_media(payload);

				if created.is_err() {
					event = Some(Event::<T>::VerifyingSocialMedia(Status::Failed, None));
				}

				created
			},
			PayloadType::Connect => {
				let created = Self::create_wallet(payload);

				if created.is_err() {
					event = Some(Event::<T>::ConnectingAccount(Status::Failed, None));
				}

				created
			},
			PayloadType::Delete => {
				let deleted = Self::delete_data(payload);
				let data_type = data_type.unwrap();
				let data_id = data_id.unwrap();

				event = if deleted.is_ok() {
					Some(Event::<T>::Deleting(Status::Success, data_type, data_id))
				} else {
					Some(Event::<T>::Deleting(Status::Failed, data_type, data_id))
				};

				deleted
			},
		};

		if let Some(event) = event {
			let _ = Self::submit_unsigned_transaction(Call::call_event_unsigned {
				block_number,
				event,
			});
		}

		result
	}

	pub fn verify_social_media_and_send_unsigned(
		block_number: T::BlockNumber,
	) -> Result<(), &'static str> {
		let result = Self::handle_myriad_api(block_number);
		if result.is_err() {
			return Err("Failed call api")
		}

		let api_result = result.unwrap();
		if api_result.is_none() {
			return Err("Failed to delete")
		}

		let api_result = api_result.unwrap();
		let tips_balance_info = api_result.1.clone();
		let reference_type = "user".as_bytes().to_vec();
		let reference_id = api_result.2;
		let account_id = Some(api_result.3);

		let call = Call::claim_reference_unsigned {
			block_number,
			tips_balance_info,
			reference_type,
			reference_id,
			account_id,
		};

		let result = Self::submit_unsigned_transaction(call);
		if result.is_ok() {
			let event: Event<T> = if api_result.5.is_some() {
				let user_social_media = api_result.5.unwrap();
				let user_social_media_info = UserSocialMediaInfo::new(&user_social_media);

				Event::<T>::VerifyingSocialMedia(Status::Success, Some(user_social_media_info))
			} else {
				let wallet = api_result.6.unwrap();
				let wallet_info = WalletInfo::new(&wallet);

				Event::<T>::ConnectingAccount(Status::Success, Some(wallet_info))
			};
			return Self::submit_unsigned_transaction(Call::call_event_unsigned {
				block_number,
				event,
			})
		}

		let data_id = api_result.0;
		let server_id = api_result.1.get_server_id().to_vec();
		let access_token = api_result.4;
		let data_type = if api_result.5.is_some() { DataType::default() } else { DataType::Wallet };
		let event: Event<T> = match data_type {
			DataType::UserSocialMedia => Event::<T>::VerifyingSocialMedia(Status::Failed, None),
			DataType::Wallet => Event::<T>::ConnectingAccount(Status::Failed, None),
		};

		let _ =
			Self::submit_unsigned_transaction(Call::call_event_unsigned { block_number, event });

		let call = Call::remove_data_unsigned {
			block_number,
			server_id,
			access_token,
			data_id,
			data_type,
		};

		Self::submit_unsigned_transaction(call)
	}

	pub fn submit_unsigned_transaction(call: Call<T>) -> Result<(), &'static str> {
		match SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()) {
			Ok(_) => Ok(()),
			Err(_) => Err("Failed in offchain_unsigned_tx"),
		}
	}
}
