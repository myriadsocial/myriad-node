use crate::*;

use frame_support::sp_runtime::offchain::http::Method;
use frame_system::offchain::SubmitTransaction;

impl<T: Config> Pallet<T> {
	pub fn verify_social_media_and_send_unsigned(
		block_number: T::BlockNumber,
	) -> Result<Option<&'static str>, &'static str> {
		let mut log_info: Option<&str> = None;

		let data = Self::get_indexing_data(block_number).ok_or("Empty storage")?;
		let payload = data.get_payload();
		let payload_type = payload.get_payload_type().clone();

		let body_str = match &payload_type {
			PayloadType::Create => Self::myriad_api_request(payload, Method::Post),
			PayloadType::Connect => Self::myriad_api_request(payload, Method::Post),
			PayloadType::Delete => Self::myriad_api_request(payload, Method::Delete),
		};

		let init = APIResult::init(
			payload.get_server_id(),
			payload.get_ft_identifier(),
			payload.get_access_token(),
		);

		let account_id = payload.get_account_id().clone();
		let api_response = if let Err(err) = body_str {
			log_info = Some(err);
			init
		} else {
			let body_str = body_str.unwrap();
			let data_type = match &payload_type {
				PayloadType::Create => Self::parse_user_social_media(&body_str),
				PayloadType::Connect => Self::parse_wallet(&body_str),
				PayloadType::Delete => None,
			};

			init.set_data_type(data_type).set_account_id(account_id)
		};

		let call = Call::claim_reference_unsigned { block_number, payload_type, api_response };

		match SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()) {
			Ok(_) => Ok(log_info),
			Err(_) => Err("Failed in offchain_unsigned_tx"),
		}
	}
}
