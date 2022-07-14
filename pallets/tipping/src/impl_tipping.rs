use super::*;

use codec::Encode;
use frame_support::{
	sp_runtime::traits::Zero,
	traits::{Currency, ExistenceRequirement, WithdrawReasons},
};
use serde_json::json;
use sp_io::offchain_index;
use sp_std::{str, vec, vec::Vec};

impl<T: Config> TippingInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type TipsBalance = TipsBalanceOf<T>;
	type TipsBalances = Vec<TipsBalanceOf<T>>;
	type TipsBalanceInfo = TipsBalanceInfo;
	type Balance = BalanceOf<T>;
	type UserCredential = UserCredential;
	type SocialMediaCredential = SocialMediaCredential;
	type ServerId = ServerId;
	type References = References;
	type ReferenceType = ReferenceType;
	type ReferenceId = ReferenceId;
	type ReferenceIds = Vec<ReferenceId>;
	type FtIdentifier = FtIdentifier;
	type AccessToken = AccessToken;
	type DataId = Vec<u8>;
	type DataType = DataType;
	type FtIdentifiers = Vec<FtIdentifier>;
	type FtIdentifierBalances = Vec<(BalanceOf<T>, FtIdentifier)>;

	fn send_tip(
		sender: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::TipsBalance, Self::Error> {
		let server_id = tips_balance_info.get_server_id();
		let ft_identifier = tips_balance_info.get_ft_identifier();
		let tip_amount = *amount;

		if T::Server::get_by_id(server_id).is_none() {
			return Err(Error::<T>::ServerNotRegister)
		}

		if !Self::is_integer(ft_identifier) {
			return Err(Error::<T>::WrongFormat)
		}

		if ft_identifier != "native".as_bytes() {
			return Err(Error::<T>::FtNotExists)
		}

		if CurrencyOf::<T>::free_balance(sender) < tip_amount {
			return Err(Error::<T>::InsufficientBalance)
		}

		match CurrencyOf::<T>::withdraw(
			sender,
			tip_amount,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				let receiver = Self::tipping_account_id();
				let tips_balance = match Self::get_tips_balance(tips_balance_info) {
					Some(mut result) => {
						let total_amount = *result.get_amount() + tip_amount;

						result.set_amount(total_amount);
						Self::update_tips_balance(&result)
					},
					None => Self::create_tips_balance(tips_balance_info, &None, &Some(tip_amount)),
				};

				CurrencyOf::<T>::resolve_creating(&receiver, imb);

				Ok(tips_balance)
			},
			_ => Err(Error::<T>::BadSignature),
		}
	}

	fn claim_tip(
		receiver: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
	) -> Result<(Self::Balance, Self::FtIdentifier), Self::Error> {
		let sender = Self::tipping_account_id();
		let tips_balance = Self::get_tips_balance(tips_balance_info);

		if tips_balance.is_none() {
			return Err(Error::<T>::NotExists)
		}

		let mut tips_balance = tips_balance.unwrap();
		let ft_identifier = tips_balance.get_ft_identifier().clone();
		let account_id = tips_balance.get_account_id().as_ref();
		let amount = *tips_balance.get_amount();

		if amount == Zero::zero() {
			return Err(Error::<T>::NothingToClaimed)
		}

		if account_id.is_none() {
			return Err(Error::<T>::ReceiverNotExists)
		}

		if account_id.unwrap() != receiver {
			return Err(Error::<T>::Unauthorized)
		}

		if !Self::is_integer(&ft_identifier) {
			return Err(Error::<T>::WrongFormat)
		}

		if ft_identifier != "native".as_bytes() {
			return Err(Error::<T>::FtNotExists)
		}

		tips_balance.set_amount(Zero::zero());

		match CurrencyOf::<T>::withdraw(
			&sender,
			amount,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(receiver, imb);

				let _ = Self::update_tips_balance(&tips_balance);

				Ok((amount, ft_identifier))
			},
			_ => Err(Error::<T>::BadSignature),
		}
	}

	fn batch_claim_tip(
		receiver: &T::AccountId,
		server_id: &Self::ServerId,
		reference_type: &Self::ReferenceType,
		reference_id: &Self::ReferenceId,
		ft_identifiers: &Self::FtIdentifiers,
	) -> Result<Self::FtIdentifierBalances, Self::Error> {
		let tips_balances: Vec<TipsBalanceOf<T>> = (0..ft_identifiers.len())
			.filter_map(|index| {
				let ft_identifier = &ft_identifiers[index];

				if ft_identifier != b"native" {
					return None
				}

				let tips_balance_info =
					TipsBalanceInfo::new(server_id, reference_type, reference_id, ft_identifier);
				let tips_balance = Self::get_tips_balance(&tips_balance_info);

				if let Some(tips_balance) = tips_balance {
					if tips_balance.get_amount() == &Zero::zero() {
						return None
					}

					if tips_balance.get_account_id().is_none() {
						return None
					}

					if tips_balance.get_account_id().as_ref().unwrap() != receiver {
						return None
					}

					return Some(tips_balance)
				}

				None
			})
			.collect();

		if tips_balances.is_empty() {
			return Err(Error::<T>::NothingToClaimed)
		}

		let sender = Self::tipping_account_id();
		let mut success_claim = Vec::<(BalanceOf<T>, FtIdentifier)>::new();

		for tips_balance in tips_balances.iter() {
			let ft = tips_balance.get_ft_identifier();
			let tips_amount = tips_balance.get_amount();
			let receiver = tips_balance.get_account_id().as_ref().unwrap();
			let mut tips_balance = tips_balance.clone();

			tips_balance.set_amount(Zero::zero());

			if ft != b"native" {
				continue
			}

			if let Ok(imb) = CurrencyOf::<T>::withdraw(
				&sender,
				*tips_amount,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::KeepAlive,
			) {
				CurrencyOf::<T>::resolve_creating(receiver, imb);

				let _ = Self::update_tips_balance(&tips_balance);

				success_claim.push((*tips_amount, ft.to_vec()));
			}
		}

		Ok(success_claim)
	}

	fn claim_reference(
		sender: &Option<T::AccountId>,
		tips_balance_info: &Self::TipsBalanceInfo,
		reference_type: &Self::ReferenceType,
		reference_id: &Self::ReferenceId,
		account_id: &Option<T::AccountId>,
		tx_fee: &Self::Balance,
		verify_owner: bool,
	) -> Result<Self::TipsBalances, Self::Error> {
		let server_id = tips_balance_info.get_server_id();
		Self::verify_server(sender, server_id, verify_owner)?;

		if account_id.is_none() {
			return Err(Error::<T>::ReceiverNotExists)
		}

		if tips_balance_info.get_ft_identifier() != "native".as_bytes() {
			return Err(Error::<T>::FtNotExists)
		}

		let native_info = TipsBalanceInfo::new(server_id, reference_type, reference_id, b"native");
		let result = Self::get_tips_balance(&native_info);

		if let Some(tips_balance) = result.clone() {
			let amount = tips_balance.get_amount();

			if amount == &Zero::zero() {
				return Err(Error::<T>::FailedToVerify)
			}

			if amount < tx_fee {
				return Err(Error::<T>::FailedToVerify)
			}
		} else {
			return Err(Error::<T>::FailedToVerify)
		}

		match CurrencyOf::<T>::withdraw(
			&Self::tipping_account_id(),
			*tx_fee,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(sender.as_ref().unwrap(), imb);

				let ref_type = tips_balance_info.get_reference_type();
				let ref_id = tips_balance_info.get_reference_id();
				let ft_identifier = tips_balance_info.get_ft_identifier();
				let ft_identifiers = if "native".as_bytes() == ft_identifier {
					vec![ft_identifier.to_vec()]
				} else {
					vec![b"native".to_vec(), ft_identifier.to_vec()]
				};

				let tips_balances = Self::update_tips_balances(
					server_id,
					&References::new(ref_type, &[ref_id.to_vec()]),
					&References::new(reference_type, &[reference_id.to_vec()]),
					&ft_identifiers,
					&account_id.clone().unwrap(),
					tx_fee,
					&result.unwrap(),
				);

				Ok(tips_balances)
			},
			_ => Err(Error::<T>::BadSignature),
		}
	}

	fn batch_claim_reference(
		sender: &T::AccountId,
		server_id: &Self::ServerId,
		references: &Self::References,
		main_references: &Self::References,
		ft_identifiers: &Self::FtIdentifiers,
		account_id: &T::AccountId,
		tx_fee: &Self::Balance,
		verify_owner: bool,
	) -> Result<Self::TipsBalances, Self::Error> {
		if sender == account_id {
			return Err(Error::<T>::FailedToVerify)
		}

		Self::verify_server(&Some(sender.clone()), server_id, verify_owner)?;

		if tx_fee == &Zero::zero() {
			return Err(Error::<T>::FailedToVerify)
		}

		if main_references.get_reference_ids().len() != 1 {
			return Err(Error::<T>::FailedToVerify)
		}

		let ref_type = main_references.get_reference_type();
		let ref_id = &main_references.get_reference_ids()[0];
		let native_info = TipsBalanceInfo::new(server_id, ref_type, ref_id, b"native");
		let result = Self::get_tips_balance(&native_info);

		if let Some(tips_balance) = result.clone() {
			let amount = tips_balance.get_amount();

			if amount == &Zero::zero() {
				return Err(Error::<T>::FailedToVerify)
			}

			if amount < tx_fee {
				return Err(Error::<T>::FailedToVerify)
			}
		} else {
			return Err(Error::<T>::FailedToVerify)
		}

		match CurrencyOf::<T>::withdraw(
			&Self::tipping_account_id(),
			*tx_fee,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(sender, imb);

				let tips_balances = Self::update_tips_balances(
					server_id,
					references,
					main_references,
					ft_identifiers,
					account_id,
					tx_fee,
					&result.unwrap(),
				);

				Ok(tips_balances)
			},
			_ => Err(Error::<T>::BadSignature),
		}
	}

	fn verify_social_media(
		sender: &T::AccountId,
		server_id: &Self::ServerId,
		access_token: &Self::AccessToken,
		social_media_credential: &Self::SocialMediaCredential,
		ft_identifier: &Self::FtIdentifier,
	) -> Result<(), Self::Error> {
		if !Self::is_integer(ft_identifier) {
			return Err(Error::<T>::WrongFormat)
		}

		if ft_identifier != "native".as_bytes() {
			return Err(Error::<T>::FtNotExists)
		}

		match Self::get_api_url(server_id, "/user-social-medias/verify") {
			Ok(api_url) => {
				let mut bearer = "Bearer ".as_bytes().to_vec();
				bearer.append(&mut access_token.to_vec());

				let mut address = String::from("0x");
				address.push_str(&hex::encode(&sender.encode()));

				let username = social_media_credential.get_username();
				let platform = social_media_credential.get_platform();
				let body = json!({
					"address": &address,
					"username": str::from_utf8(username).unwrap_or("username"),
					"platform": str::from_utf8(platform).unwrap_or("platform"),
				})
				.to_string();

				let payload = Payload::<AccountIdOf<T>>::init(server_id, &api_url, &bearer)
					.set_body(body.as_bytes())
					.set_account_id(sender)
					.set_ft_identifier(ft_identifier)
					.set_payload_type(PayloadType::Create);

				let key = Self::derived_key(<frame_system::Pallet<T>>::block_number());
				let data = IndexingData::init(b"verify_social_media", payload);

				offchain_index::set(&key, &data.encode());

				Ok(())
			},
			Err(err) => Err(err),
		}
	}

	fn connect_account(
		sender: &T::AccountId,
		server_id: &Self::ServerId,
		access_token: &Self::AccessToken,
		user_credential: &Self::UserCredential,
		ft_identifier: &Self::FtIdentifier,
	) -> Result<(), Self::Error> {
		if !Self::is_integer(ft_identifier) {
			return Err(Error::<T>::WrongFormat)
		}

		if ft_identifier != "native".as_bytes() {
			return Err(Error::<T>::FtNotExists)
		}

		let user_id = user_credential.get_user_id();
		let result = str::from_utf8(user_id);
		if result.is_err() {
			return Err(Error::<T>::WrongFormat)
		}

		let mut endpoint = String::from("/users/");
		endpoint.push_str(result.unwrap());
		endpoint.push_str("/wallets");

		match Self::get_api_url(server_id, &endpoint) {
			Ok(api_url) => {
				let mut bearer = "Bearer ".as_bytes().to_vec();
				bearer.append(&mut access_token.to_vec());

				let mut address = String::from("0x");
				address.push_str(&hex::encode(&sender.encode()));

				let nonce = *user_credential.get_nonce();
				let mut signature = "0x".as_bytes().to_vec();
				signature.append(&mut user_credential.get_signature().to_vec());

				let body = json!({
					"nonce": nonce,
					"publicAddress": &address,
					"signature": str::from_utf8(&signature).unwrap_or("signature"),
					"walletType": "polkadot{.js}",
					"networkType": "myriad",
					"data": {
						"id": &address,
					},
				})
				.to_string();

				let payload = Payload::<AccountIdOf<T>>::init(server_id, &api_url, &bearer)
					.set_body(body.as_bytes())
					.set_account_id(sender)
					.set_ft_identifier(ft_identifier)
					.set_payload_type(PayloadType::Connect);

				let key = Self::derived_key(<frame_system::Pallet<T>>::block_number());
				let data = IndexingData::init(b"connect_account", payload);

				offchain_index::set(&key, &data.encode());

				Ok(())
			},
			Err(err) => Err(err),
		}
	}
}
