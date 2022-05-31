use crate::*;

use frame_support::sp_runtime::offchain::http;
use frame_system::offchain::SubmitTransaction;

impl<T: Config> Pallet<T> {
	pub fn call_myriad_api(
		block_number: T::BlockNumber,
	) -> Result<Option<APIResult<AccountIdOf<T>>>, http::Error> {
		let data = Self::get_indexing_data(block_number);

		if data.is_none() {
			return Ok(None)
		}

		let data = data.unwrap();
		let payload = data.get_payload();
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
			PayloadType::Delete => Self::delete_data(payload),
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
		let result = Self::call_myriad_api(block_number);
		if result.is_err() {
			return Err("Failed call api")
		}

		let api_result = result.unwrap();
		if api_result.is_none() {
			return Ok(())
		}

		let api_response = api_result.unwrap();
		let call = Call::claim_reference_unsigned { block_number, api_response };

		Self::submit_unsigned_transaction(call)
	}

	pub fn submit_unsigned_transaction(call: Call<T>) -> Result<(), &'static str> {
		match SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()) {
			Ok(_) => Ok(()),
			Err(_) => Err("Failed in offchain_unsigned_tx"),
		}
	}
}
