use crate::*;

use frame_support::sp_runtime::offchain::http;
use sp_std::{str, vec, vec::Vec};

impl<T: Config> Pallet<T> {
	pub fn create_user_social_media(
		payload: &Payload<AccountIdOf<T>>,
	) -> Result<Option<APIResult<AccountIdOf<T>>>, http::Error> {
		let access_token = str::from_utf8(payload.get_access_token()).unwrap_or("error");
		let body_str = Self::myriad_api_request(payload, access_token, http::Method::Post)?;
		let api_result = Self::get_api_result(payload, &body_str);

		Ok(api_result)
	}

	pub fn create_wallet(
		payload: &Payload<AccountIdOf<T>>,
	) -> Result<Option<APIResult<AccountIdOf<T>>>, http::Error> {
		let access_token = str::from_utf8(payload.get_access_token()).unwrap_or("error");
		let body_str = Self::myriad_api_request(payload, access_token, http::Method::Post)?;
		let api_result = Self::get_api_result(payload, &body_str);

		Ok(api_result)
	}

	pub fn delete_data(
		payload: &Payload<AccountIdOf<T>>,
	) -> Result<Option<APIResult<AccountIdOf<T>>>, http::Error> {
		let access_token = str::from_utf8(payload.get_access_token()).unwrap_or("error");
		let _ = Self::myriad_api_request(payload, access_token, http::Method::Delete);

		Ok(None)
	}

	pub fn myriad_api_request(
		payload: &Payload<AccountIdOf<T>>,
		access_token: &str,
		method: http::Method,
	) -> Result<String, http::Error> {
		let api_url = str::from_utf8(payload.get_api_url()).unwrap_or("error");
		let body = payload.get_body();
		let request_body = vec![body.clone()];
		let init_request = http::Request::<Vec<Vec<u8>>>::new(api_url)
			.method(method.clone())
			.add_header("Authorization", access_token)
			.add_header("content-type", "application/json");

		let request = match method {
			http::Method::Delete => init_request,
			_ => init_request.body(request_body),
		};

		let pending = request.send().map_err(|_| http::Error::IoError)?;
		let response = pending.wait().map_err(|_| http::Error::IoError)?;

		if response.code != 200 {
			return Err(http::Error::Unknown)
		}

		if method == http::Method::Delete {
			return Ok(String::new())
		}

		let body = response.body().collect::<Vec<u8>>();
		let body_str = str::from_utf8(&body).map_err(|_| http::Error::Unknown)?;

		Ok(body_str.to_string())
	}

	pub fn get_api_result(
		payload: &Payload<AccountIdOf<T>>,
		body_str: &str,
	) -> Option<APIResult<AccountIdOf<T>>> {
		let server_id = payload.get_server_id();
		let ft_identifier = payload.get_ft_identifier();
		let account_id = payload.get_account_id().clone().unwrap_or_default();
		let access_token = payload.get_access_token();
		let data_type = match payload.get_payload_type() {
			PayloadType::Create => {
				let user_social_media = Self::parse_user_social_media(body_str);
				let info = UserSocialMediaInfo::new(&user_social_media);

				Some(DataType::UserSocialMedia(info))
			},
			PayloadType::Connect => {
				let wallet = Self::parse_wallet(body_str);
				let info = WalletInfo::new(&wallet);

				Some(DataType::Wallet(info))
			},
			PayloadType::Delete => None,
		};

		let api_result = APIResult::init(server_id, ft_identifier, access_token)
			.set_account_id(account_id)
			.set_data_type(data_type.unwrap_or_default());

		Some(api_result)
	}
}
