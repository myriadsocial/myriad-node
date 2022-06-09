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
use sp_io::offchain_index;
use sp_std::{str, vec::Vec};

const PALLET_ID: PalletId = PalletId(*b"Tipping!");
const ONCHAIN_TX_KEY: &[u8] = b"pallet_tipping::indexing";
const UNSIGNED_TXS_PRIORITY: u64 = 100;

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

		if !verify_owner {
			return Ok(())
		}

		if sender.is_none() {
			return Err(Error::<T>::Unauthorized)
		}

		let sender = sender.clone().unwrap();
		let server = server.unwrap();
		let server_owner = server.get_owner();

		if &sender != server_owner {
			return Err(Error::<T>::Unauthorized)
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
		let mut storage_ref = StorageValueRef::persistent(&key);

		match storage_ref.get::<IndexingData<AccountIdOf<T>>>() {
			Ok(data) => {
				storage_ref.clear();
				data
			},
			Err(_) => None,
		}
	}

	pub fn validate_transaction_parameters(
		block_number: &T::BlockNumber,
		provide: &[u8],
	) -> TransactionValidity {
		let current_block = <system::Pallet<T>>::block_number();
		if &current_block < block_number {
			return InvalidTransaction::Future.into()
		}

		let valid_tx = |provide| {
			ValidTransaction::with_tag_prefix("pallet_tipping")
				.priority(UNSIGNED_TXS_PRIORITY)
				.and_provides([&provide])
				.longevity(5)
				.propagate(true)
				.build()
		};

		valid_tx(provide.to_vec())
	}

	pub fn get_api_url(server_id: &[u8], endpoint: &str) -> Result<Vec<u8>, Error<T>> {
		let server = T::Server::get_by_id(server_id);

		if server.is_none() {
			return Err(Error::<T>::ServerNotRegister)
		}

		let mut api_url = server.unwrap().get_api_url().to_vec();
		let mut endpoint = endpoint.as_bytes().to_vec();

		api_url.append(&mut endpoint);

		Ok(api_url)
	}

	pub fn store_deleted_payload(
		server_id: &[u8],
		access_token: &[u8],
		data_type: &DataType,
	) -> Result<(), Error<T>> {
		let endpoint = match data_type {
			DataType::UserSocialMedia(user_social_media_info) => {
				Self::deposit_event(Event::<T>::VerifyingSocialMedia(Status::Failed, None));

				let mut endpoint = String::from("/user-social-medias/");
				let id = str::from_utf8(user_social_media_info.get_id()).unwrap_or("id");

				endpoint.push_str(id);

				endpoint
			},
			DataType::Wallet(wallet_info) => {
				Self::deposit_event(Event::<T>::ConnectingAccount(Status::Failed, None));

				let mut endpoint = String::from("/wallets/");
				let id = str::from_utf8(wallet_info.get_id()).unwrap_or("id");

				endpoint.push_str(id);

				endpoint
			},
		};

		match Self::get_api_url(server_id, &endpoint) {
			Ok(api_url) => {
				let payload = Payload::<AccountIdOf<T>>::init(server_id, &api_url, access_token);
				let key = Self::derived_key(<frame_system::Pallet<T>>::block_number());
				let data = IndexingData::init(b"remove_data_unsigned", payload);

				offchain_index::set(&key, &data.encode());

				Ok(())
			},
			Err(err) => Err(err),
		}
	}

	pub fn parse_user_social_media(data: &str) -> Option<DataType> {
		let data = str::replace(data, "createdAt", "created_at");
		let data = str::replace(&data, "updatedAt", "updated_at");
		let data = str::replace(&data, "peopleId", "people_id");
		let data = str::replace(&data, "userId", "user_id");

		match serde_json::from_str::<UserSocialMedia>(&data) {
			Ok(result) => Some(DataType::UserSocialMedia(result)),
			Err(_) => None,
		}
	}

	pub fn parse_wallet(data: &str) -> Option<DataType> {
		let data = str::replace(data, "createdAt", "created_at");
		let data = str::replace(&data, "updatedAt", "updated_at");
		let data = str::replace(&data, "networkId", "network_id");
		let data = str::replace(&data, "userId", "user_id");

		match serde_json::from_str::<Wallet>(&data) {
			Ok(result) => Some(DataType::Wallet(result)),
			Err(_) => None,
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
