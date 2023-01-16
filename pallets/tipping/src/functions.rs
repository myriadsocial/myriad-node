use crate::*;

use frame_support::{
	dispatch::DispatchError,
	pallet_prelude::Encode,
	sp_runtime::traits::{AccountIdConversion, Hash, SaturatedConversion, Saturating, Zero},
	traits::{fungibles, Currency, ExistenceRequirement, Get},
	PalletId,
};
use sp_std::vec::*;

const PALLET_ID: PalletId = PalletId(*b"Tipping!");

impl<T: Config> Pallet<T> {
	/// The account ID that holds tipping's funds
	pub fn tipping_account_id() -> T::AccountId {
		PALLET_ID.into_account_truncating()
	}

	pub fn generate_receipt_id(
		from: &T::AccountId,
		to: &T::AccountId,
		info: &TipsBalanceInfoOf<T>,
	) -> T::Hash {
		let mut from_bytes = from.encode();

		let mut to_bytes = to.encode();
		let mut server_id_bytes = info.get_server_id().encode();
		let mut reference_type_bytes = info.get_reference_type().to_vec();
		let mut reference_id_bytes = info.get_reference_id().to_vec();
		let mut ft_identifier_bytes = info.get_ft_identifier().to_vec();

		let from_info = frame_system::Pallet::<T>::account(from);

		let mut nonce_bytes = from_info.nonce.encode();

		from_bytes.append(&mut to_bytes);
		from_bytes.append(&mut server_id_bytes);
		from_bytes.append(&mut reference_type_bytes);
		from_bytes.append(&mut reference_id_bytes);
		from_bytes.append(&mut ft_identifier_bytes);
		from_bytes.append(&mut nonce_bytes);

		let seed = &from_bytes;
		T::Hashing::hash(seed)
	}

	pub fn can_update_balance(key: &TipsBalanceKeyOf<T>) -> bool {
		TipsBalanceByReference::<T>::contains_key(key)
	}

	pub fn can_pay_content(
		sender: &T::AccountId,
		amount: &BalanceOf<T>,
	) -> Result<FeeDetail<BalanceOf<T>>, Error<T>> {
		let tx_fee_denom = 100u8
			.checked_div(T::TransactionFee::get())
			.filter(|value| value <= &100u8)
			.ok_or(Error::<T>::InsufficientFee)?;

		let fee: BalanceOf<T> = *amount / tx_fee_denom.saturated_into();
		let minimum_balance = CurrencyOf::<T>::minimum_balance();
		let total_transfer = *amount + fee;

		let current_balance = CurrencyOf::<T>::free_balance(sender);
		let transferable_balance = current_balance - minimum_balance;

		if total_transfer > transferable_balance {
			return Err(Error::<T>::InsufficientBalance)
		}

		let admin_fee_denom = 100u8
			.checked_div(T::AdminFee::get())
			.filter(|value| value <= &100u8)
			.ok_or(Error::<T>::InsufficientFee)?;

		let admin_fee = fee / admin_fee_denom.saturated_into();
		let server_fee = fee - admin_fee;
		let fee_detail = FeeDetail::new(admin_fee, server_fee, fee);

		Ok(fee_detail)
	}

	pub fn can_pay_fee(key: &TipsBalanceKeyOf<T>, tx_fee: &BalanceOf<T>) -> Result<(), Error<T>> {
		if tx_fee == &Zero::zero() {
			return Err(Error::<T>::InsufficientBalance)
		}

		let tips_balance = Self::tips_balance_by_reference(key).ok_or(Error::<T>::NotExists)?;
		let amount = tips_balance.get_amount();

		if amount == &Zero::zero() {
			return Err(Error::<T>::InsufficientBalance)
		}

		if amount < tx_fee {
			return Err(Error::<T>::InsufficientBalance)
		}

		Ok(())
	}

	pub fn can_claim_tip(
		key: &TipsBalanceKeyOf<T>,
		receiver: &AccountIdOf<T>,
	) -> Option<TipsBalanceOf<T>> {
		if let Some(tips_balance) = Self::tips_balance_by_reference(key) {
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
	}

	pub fn do_update_withdrawal_balance(ft_identifier: &[u8], balance: BalanceOf<T>) {
		WithdrawalBalance::<T>::mutate(ft_identifier, |value| {
			*value += balance;
		});
	}

	pub fn do_update_reward_balance(
		tips_balance_info: &TipsBalanceInfoOf<T>,
		balance: BalanceOf<T>,
	) {
		let server_id = tips_balance_info.get_server_id();
		let ft_identifier = tips_balance_info.get_ft_identifier();
		RewardBalance::<T>::mutate(server_id, ft_identifier, |value| {
			*value += balance;
		});
	}

	pub fn do_store_receipt(
		from: &T::AccountId,
		to: &T::AccountId,
		detail: &TipsBalanceInfoOf<T>,
		total_paid: &BalanceOf<T>,
		total_fee: &BalanceOf<T>,
	) -> ReceiptOf<T> {
		let id = Self::generate_receipt_id(from, to, detail);
		let now = T::TimeProvider::now().as_millis();
		let receipt = Receipt::new(&id, from, to, detail, total_paid, total_fee, now);

		Receipts::<T>::insert(id, &receipt);
		ReceiptIds::<T>::mutate(|value| {
			value.push(id);
		});

		receipt
	}

	pub fn do_store_tips_balance(
		tips_balance: &TipsBalanceOf<T>,
		set_empty: bool,
		tx_fee: Option<BalanceOf<T>>,
	) -> BalanceOf<T> {
		let key = tips_balance.key();
		let amount = *tips_balance.get_amount();
		let account_id = tips_balance.get_account_id();
		let ft_identifier = tips_balance.get_ft_identifier();

		//  Total tip that has been send and claim
		let mut total_tip: BalanceOf<T> = amount;

		if Self::can_update_balance(&key) {
			TipsBalanceByReference::<T>::mutate(key, |tips_balance| match tips_balance {
				Some(tips_balance) => {
					if set_empty {
						tips_balance.set_amount(Zero::zero()); // Set balance to zero
					} else if tx_fee.is_some() && ft_identifier == b"native" {
						// Reduce user balance by the tx fee
						// As user ask admin server to claim references
						let current_balance = *tips_balance.get_amount();
						let final_balance =
							current_balance.saturating_sub(tx_fee.unwrap()).saturating_add(amount);
						tips_balance.set_amount(final_balance);
						total_tip = final_balance;
					} else {
						// There is an increase in balance
						tips_balance.add_amount(amount);
					}

					// Claim tips balance by account_id
					if account_id.is_some() {
						tips_balance.set_account_id(account_id.as_ref().unwrap());
					}
				},
				None => (),
			});
		} else {
			TipsBalanceByReference::<T>::insert(key, tips_balance);
		}

		total_tip
	}

	pub fn do_transfer(
		ft_identifier: &[u8],
		sender: &AccountIdOf<T>,
		receiver: &AccountIdOf<T>,
		amount: BalanceOf<T>,
	) -> Result<(), DispatchError> {
		if ft_identifier == b"native" {
			CurrencyOf::<T>::transfer(sender, receiver, amount, ExistenceRequirement::KeepAlive)?;
		} else {
			let asset_id = Self::asset_id(ft_identifier)?;
			let _ = <T::Assets as fungibles::Transfer<T::AccountId>>::transfer(
				asset_id,
				sender,
				receiver,
				amount.saturated_into(),
				true,
			)?;
		}

		Ok(())
	}

	pub fn do_store_tips_balances(
		server_id: &AccountIdOf<T>,
		references: &References,
		account_references: &References,
		ft_identifiers: &[FtIdentifier],
		account_id: &AccountIdOf<T>,
		tx_fee: &BalanceOf<T>,
	) -> Vec<TipsBalanceOf<T>> {
		let mut account_tips_balances = Vec::<TipsBalanceOf<T>>::new();

		let account_reference_type = account_references.get_reference_type();
		let account_reference_id = &account_references.get_reference_ids()[0];

		for ft_identifier in ft_identifiers.iter() {
			let mut tip: BalanceOf<T> = Zero::zero();

			let reference_type = references.get_reference_type();
			let reference_ids = references.get_reference_ids();

			// Get balance for references
			// Store the balance to account reference balance
			for reference_id in reference_ids {
				let server_id = server_id.clone();
				let key = (server_id, reference_type, reference_id, ft_identifier);
				let tips_balance = TipsBalanceByReference::<T>::take(&key);

				if let Some(tips_balance) = tips_balance {
					let amount = tips_balance.get_amount();
					if *amount > Zero::zero() {
						tip = tip.saturating_add(*amount);
					}
				}
			}

			let account_tips_balance_info = TipsBalanceInfo::new(
				server_id,
				account_reference_type,
				account_reference_id,
				ft_identifier,
			);

			let mut account_tips_balance = TipsBalance::new(&account_tips_balance_info, &tip);

			account_tips_balance.set_account_id(account_id);

			let tips = Self::do_store_tips_balance(&account_tips_balance, false, Some(*tx_fee));

			account_tips_balance.set_amount(tips);
			account_tips_balances.push(account_tips_balance);
		}

		account_tips_balances
	}

	pub fn asset_id(ft_identifier: &[u8]) -> Result<u32, Error<T>> {
		let str_num =
			String::from_utf8(ft_identifier.to_vec()).map_err(|_| Error::<T>::WrongFormat)?;

		str_num.parse::<u32>().map_err(|_| Error::<T>::WrongFormat)
	}
}
