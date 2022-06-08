use crate::*;

use frame_support::sp_runtime::offchain::http::{Method, Request, Response};
use sp_std::{str, vec, vec::Vec};

impl<T: Config> Pallet<T> {
	pub fn myriad_api_request(
		payload: &Payload<AccountIdOf<T>>,
		method: Method,
	) -> Result<String, &'static str> {
		let api_url_str = str::from_utf8(payload.get_api_url());
		let access_token_str = str::from_utf8(payload.get_access_token());

		let access_token = access_token_str.map_err(|_| "Failed to format acces token")?;
		let api_url = api_url_str.map_err(|_| "Failed to format api url")?;
		let body = payload.get_body();
		let request_body = vec![body.clone()];
		let init_request = Request::<Vec<Vec<u8>>>::new(api_url)
			.method(method.clone())
			.add_header("Authorization", access_token)
			.add_header("content-type", "application/json");

		let request = match method {
			Method::Delete => init_request,
			_ => init_request.body(request_body),
		};

		let pending = request.send().map_err(|_| "Request timeout")?;
		let response = pending.wait().map_err(|_| "Failed to response")?;

		if response.code != 200 {
			let body_str = Self::parse_body_reponse(response)?;

			log::info!("{}", body_str);

			return Err("Status code error")
		}

		if method == Method::Delete {
			return Ok(String::new())
		}

		Self::parse_body_reponse(response)
	}

	pub fn parse_body_reponse(response: Response) -> Result<String, &'static str> {
		let body = response.body().collect::<Vec<u8>>();

		match str::from_utf8(&body) {
			Ok(result) => Ok(result.to_string()),
			Err(_) => Err("Failed to parse body"),
		}
	}
}
