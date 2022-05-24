use crate::*;

use frame_support::sp_runtime::offchain::http;
use frame_system::offchain::SubmitTransaction;
use sp_std::str;

impl<T: Config> Pallet<T> {
	pub fn handle_myriad_api(
		block_number: T::BlockNumber,
	) -> Result<Option<APIResult<T>>, http::Error> {
		let data = Self::get_indexing_data(block_number);

		if data.is_none() {
			return Ok(None)
		}

		let payload = data.unwrap().1;
		let payload_type = payload.get_payload_type();

		match payload_type {
			PayloadType::Create => {
				let created = Self::create_user_social_media(payload);
				let event: Event<T> = if created.is_ok() {
					let user_social_media = created.clone().unwrap().unwrap().2;
					let user_social_media_info = UserSocialMediaInfo::new(&user_social_media);

					Event::<T>::VerifyingSocialMedia(Status::Success, Some(user_social_media_info))
				} else {
					Event::<T>::VerifyingSocialMedia(Status::Failed, None)
				};

				let _ = Self::submit_unsigned_transaction(Call::call_event_unsigned {
					block_number,
					event,
				});

				created
			},
			PayloadType::Delete => {
				let deleted = Self::delete_user_social_media(payload);
				let event: Event<T> = if deleted.is_ok() {
					Event::<T>::DeletingSocialMedia(Status::Success)
				} else {
					Event::<T>::DeletingSocialMedia(Status::Failed)
				};

				let _ = Self::submit_unsigned_transaction(Call::call_event_unsigned {
					block_number,
					event,
				});

				deleted
			},
		}
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
		let reference_type = "user".as_bytes().to_vec();
		let reference_id = api_result.2.get_user_id().as_bytes().to_vec();
		let account_id = Some(api_result.0);
		let tips_balance_info = api_result.1.clone();

		let call = Call::claim_reference_unsigned {
			block_number,
			tips_balance_info,
			reference_type,
			reference_id,
			account_id,
		};

		let result = Self::submit_unsigned_transaction(call);
		if result.is_ok() {
			return Ok(())
		}

		let server_id = api_result.1.get_server_id().to_vec();
		let access_token = api_result.3.as_bytes().to_vec();
		let user_social_media_id = api_result.2.get_id().as_bytes().to_vec();

		let call = Call::remove_user_social_media_unsigned {
			block_number,
			server_id,
			access_token,
			user_social_media_id,
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
