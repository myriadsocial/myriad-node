use crate::*;

use frame_support::sp_runtime::offchain::http;
use sp_std::{str, vec, vec::Vec};

impl<T: Config> Pallet<T> {
	pub fn myriad_api_request(
		payload: &Payload<AccountIdOf<T>>,
		method: http::Method,
	) -> Result<String, &'static str> {
		let api_url_str = str::from_utf8(payload.get_api_url());
		let access_token_str = str::from_utf8(payload.get_access_token());

		let access_token = access_token_str.map_err(|_| "Failed to format acces token")?;
		let api_url = api_url_str.map_err(|_| "Failed to format api url")?;
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

		let pending = request.send().map_err(|_| "Request timeout")?;
		let response = pending.wait().map_err(|_| "Failed to response")?;

		if response.code != 200 {
			log::info!("Status code error: {}", response.code);
			return Err("Status code error")
		}

		if method == http::Method::Delete {
			return Ok(String::new())
		}

		let body = response.body().collect::<Vec<u8>>();
		let body_str = str::from_utf8(&body).map_err(|_| "Failed to format response")?;

		Ok(body_str.to_string())
	}
}
