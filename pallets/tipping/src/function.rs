use super::*;

use codec::Encode;
use frame_support::{
	sp_runtime::{
		offchain::{http, storage::StorageValueRef},
		traits::{AccountIdConversion, Zero},
		transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	},
	PalletId,
};
use frame_system::{self as system, offchain::SubmitTransaction};
use sp_std::{str, vec, vec::Vec};

const PALLET_ID: PalletId = PalletId(*b"Tipping!");
const ONCHAIN_TX_KEY: &[u8] = b"pallet_tipping::indexing";

impl<T: Config> Pallet<T> {
	/// The account ID that holds tipping's funds
	pub fn tipping_account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	pub fn verify_server(
		sender: &Option<T::AccountId>,
		server_id: &[u8],
		verify_owner: bool,
	) -> Result<(), Error<T>> {
		let server = T::Server::get_by_id(server_id);

		if server.is_none() {
			return Err(Error::<T>::ServerNotRegister)
		}

		if verify_owner {
			if sender.is_none() {
				return Err(Error::<T>::Unauthorized)
			}

			let sender = sender.clone().unwrap();
			let server = server.unwrap();
			let server_owner = server.get_owner();

			if &sender != server_owner {
				return Err(Error::<T>::Unauthorized)
			}
		}

		Ok(())
	}

	pub fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
		block_number.using_encoded(|encoded_bn| {
			ONCHAIN_TX_KEY
				.to_vec()
				.iter()
				.chain(b"/".iter())
				.chain(encoded_bn)
				.copied()
				.collect::<Vec<u8>>()
		})
	}

	pub fn get_indexing_data(block_number: T::BlockNumber) -> Option<IndexingData<AccountIdOf<T>>> {
		let key = Self::derived_key(block_number);
		let storage_ref = StorageValueRef::persistent(&key);

		match storage_ref.get::<IndexingData<AccountIdOf<T>>>() {
			Ok(data) => data,
			Err(_) => None,
		}
	}

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

	pub fn submit_unsigned_transaction(call: Call<T>) -> Result<(), &'static str> {
		match SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()) {
			Ok(_) => Ok(()),
			Err(_) => Err("Failed in offchain_unsigned_tx"),
		}
	}

	pub fn validate_transaction_parameters(
		block_number: &T::BlockNumber,
		tag: &'static str,
	) -> TransactionValidity {
		let current_block = <system::Pallet<T>>::block_number();
		if &current_block < block_number {
			return InvalidTransaction::Future.into()
		}

		ValidTransaction::with_tag_prefix(tag)
			.and_provides(block_number)
			.propagate(true)
			.build()
	}

	pub fn get_tips_balance(tips_balance_info: &TipsBalanceInfo) -> Option<TipsBalanceOf<T>> {
		let reference_type = tips_balance_info.get_reference_type();
		let reference_id = tips_balance_info.get_reference_id();
		let server_id = tips_balance_info.get_server_id();
		let ft_identifier = tips_balance_info.get_ft_identifier();

		Self::tips_balance_by_reference((server_id, reference_type, reference_id, ft_identifier))
	}

	pub fn create_tips_balance(
		tips_balance_info: &TipsBalanceInfo,
		account_id: &Option<AccountIdOf<T>>,
		amount: &Option<BalanceOf<T>>,
	) -> TipsBalanceOf<T> {
		let server_id = tips_balance_info.get_server_id();
		let reference_type = tips_balance_info.get_reference_type();
		let reference_id = tips_balance_info.get_reference_id();
		let ft_identifier = tips_balance_info.get_ft_identifier();
		let amount = if amount.is_some() { amount.unwrap() } else { Zero::zero() };
		let tips_balance = TipsBalance::new(tips_balance_info, account_id, &amount);

		TipsBalanceByReference::<T>::insert(
			(server_id, reference_type, reference_id, ft_identifier),
			tips_balance.clone(),
		);

		tips_balance
	}

	pub fn update_tips_balance(tips_balance: &TipsBalanceOf<T>) -> TipsBalanceOf<T> {
		let tips_balance_info = tips_balance.get_tips_balance_info();
		let server_id = tips_balance_info.get_server_id();
		let reference_type = tips_balance_info.get_reference_type();
		let reference_id = tips_balance_info.get_reference_id();
		let ft_identifier = tips_balance_info.get_ft_identifier();

		TipsBalanceByReference::<T>::insert(
			(server_id, reference_type, reference_id, ft_identifier),
			tips_balance.clone(),
		);

		tips_balance.clone()
	}

	pub fn default_tips_balances(
		tips_balance_info: &TipsBalanceInfo,
	) -> (TipsBalanceOf<T>, Option<TipsBalanceOf<T>>) {
		(TipsBalance::new(tips_balance_info, &None, &Zero::zero()), None)
	}

	pub fn get_api_url(server_id: &[u8], endpoint: &str) -> Result<Vec<u8>, Error<T>> {
		let server = T::Server::get_by_id(server_id);

		if let Some(server_info) = server {
			let mut api_url = server_info.get_api_url().to_vec();
			let mut endpoint = endpoint.as_bytes().to_vec();

			api_url.append(&mut endpoint);

			return Ok(api_url)
		}

		Err(Error::<T>::ServerNotRegister)
	}

	pub fn parse_user_social_media(data: &str) -> UserSocialMedia {
		let data = str::replace(data, "createdAt", "created_at");
		let data = str::replace(&data, "updatedAt", "updated_at");
		let data = str::replace(&data, "peopleId", "people_id");
		let data = str::replace(&data, "userId", "user_id");

		match serde_json::from_str::<UserSocialMedia>(&data) {
			Ok(result) => result,
			Err(_) => UserSocialMedia::default(),
		}
	}

	pub fn is_integer(ft_identifier: &[u8]) -> bool {
		if ft_identifier == "native".as_bytes() {
			return true
		};

		let str_num = match String::from_utf8(ft_identifier.to_vec()) {
			Ok(res) => res,
			Err(err) => err.to_string(),
		};

		str_num.parse::<u16>().is_ok()
	}
}
