use crate::*;

use codec::Encode;
use frame_support::{
	sp_runtime::{
		offchain::storage::StorageValueRef,
		traits::AccountIdConversion,
		transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	},
	PalletId,
};
use frame_system as system;
use sp_std::{str, vec::Vec};

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