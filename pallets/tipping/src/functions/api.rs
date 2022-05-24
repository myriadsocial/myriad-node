use crate::*;

use frame_support::sp_runtime::offchain::http;
use sp_std::{str, vec, vec::Vec};

impl<T: Config> Pallet<T> {
	pub fn create_user_social_media(
		payload: Payload<AccountIdOf<T>>,
	) -> Result<Option<APIResult<T>>, http::Error> {
		let api_url = str::from_utf8(payload.get_api_url()).unwrap_or("error");
		let access_token = str::from_utf8(payload.get_access_token()).unwrap_or("error");
		let user_verification = payload.get_user_verification();
		let account_id = payload.get_account_id();
		let server_id = payload.get_server_id();
		let ft_identifier = payload.get_ft_identifier();
		let request_body = vec![user_verification];
		let request = http::Request::post(api_url, request_body.clone())
			.add_header("Authorization", access_token)
			.add_header("content-type", "application/json");

		let pending =
			request.body(request_body.clone()).send().map_err(|_| http::Error::IoError)?;

		let response = pending.wait().map_err(|_| http::Error::IoError)?;

		if response.code != 200 {
			return Err(http::Error::Unknown)
		}

		let body = response.body().collect::<Vec<u8>>();
		let body_str = str::from_utf8(&body).map_err(|_| http::Error::Unknown)?;
		let user_social_media = Self::parse_user_social_media(body_str);
		let tips_balance_info = TipsBalanceInfo::new(
			server_id,
			"people".as_bytes(),
			user_social_media.get_people_id().as_bytes(),
			ft_identifier,
		);

		Ok(Some((
			account_id.clone(),
			tips_balance_info,
			user_social_media,
			access_token.to_string(),
		)))
	}

	pub fn delete_user_social_media(
		payload: Payload<AccountIdOf<T>>,
	) -> Result<Option<APIResult<T>>, http::Error> {
		let api_url = str::from_utf8(payload.get_api_url()).unwrap_or("error");
		let access_token = str::from_utf8(payload.get_access_token()).unwrap_or("error");
		let request = http::Request::<Vec<Vec<u8>>>::new(api_url)
			.method(http::Method::Delete)
			.add_header("Authorization", access_token)
			.add_header("content-type", "application/json");

		let pending = request.send().map_err(|_| http::Error::IoError)?;

		let response = pending.wait().map_err(|_| http::Error::Unknown)?;

		if response.code != 200 {
			return Err(http::Error::Unknown)
		}

		Ok(None)
	}
}
