use crate::*;

use frame_support::{
	dispatch::DispatchError,
	sp_runtime::traits::{AccountIdConversion, SaturatedConversion, Saturating, Zero},
	traits::{fungibles, Currency, ExistenceRequirement, WithdrawReasons},
	PalletId,
};
use sp_std::vec::Vec;

const PALLET_ID: PalletId = PalletId(*b"Tipping!");

impl<T: Config> Pallet<T> {
	/// The account ID that holds tipping's funds
	pub fn tipping_account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	pub fn can_update_balance(tips_balance_key: &TipsBalanceKeyOf<T>) -> bool {
		TipsBalanceByReference::<T>::contains_key(tips_balance_key)
	}

	pub fn can_pay_fee(
		tips_balance_key: &TipsBalanceKeyOf<T>,
		tx_fee: &BalanceOf<T>,
	) -> Result<(), Error<T>> {
		if tx_fee == &Zero::zero() {
			return Err(Error::<T>::InsufficientBalance)
		}

		let tips_balance =
			Self::tips_balance_by_reference(tips_balance_key).ok_or(Error::<T>::NotExists)?;

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
		tips_balance_key: &TipsBalanceKeyOf<T>,
		receiver: &AccountIdOf<T>,
	) -> Option<TipsBalanceOf<T>> {
		if let Some(tips_balance) = Self::tips_balance_by_reference(tips_balance_key) {
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

	pub fn do_store_tips_balance(
		tips_balance: &TipsBalanceOf<T>,
		set_empty: bool,
		tx_fee: Option<BalanceOf<T>>,
	) -> BalanceOf<T> {
		let tips_balance_key = tips_balance.key();
		let amount = *tips_balance.get_amount();
		let account_id = tips_balance.get_account_id();
		let ft_identifier = tips_balance.get_ft_identifier();

		//  Total tip that has been send and claim
		let mut total_tip: BalanceOf<T> = amount;

		if Self::can_update_balance(&tips_balance_key) {
			TipsBalanceByReference::<T>::mutate(
				tips_balance_key,
				|tips_balance| match tips_balance {
					Some(tips_balance) => {
						if set_empty {
							tips_balance.set_amount(Zero::zero());
						} else if tx_fee.is_some() && ft_identifier == b"native" {
							let current_balance = *tips_balance.get_amount();
							let final_balance = current_balance
								.saturating_sub(tx_fee.unwrap())
								.saturating_add(amount);
							tips_balance.set_amount(final_balance);
							total_tip = final_balance;
						} else {
							tips_balance.add_amount(amount);
						}

						if account_id.is_some() {
							tips_balance.set_account_id(account_id.as_ref().unwrap());
						}
					},
					None => (),
				},
			);
		} else {
			TipsBalanceByReference::<T>::insert(tips_balance.key(), tips_balance);
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
			let imb = CurrencyOf::<T>::withdraw(
				sender,
				amount,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::KeepAlive,
			)?;

			CurrencyOf::<T>::resolve_creating(receiver, imb);
		} else {
			let asset_id = Self::asset_id(ft_identifier)?;
			let _ = <T::Assets as fungibles::Mutate<T::AccountId>>::teleport(
				asset_id,
				sender,
				receiver,
				amount.saturated_into(),
			)?;
		}

		Ok(())
	}

	pub fn do_store_tips_balances(
		server_id: &AccountIdOf<T>,
		references: &References,
		main_references: &References,
		ft_identifiers: &[FtIdentifier],
		account_id: &AccountIdOf<T>,
		tx_fee: &BalanceOf<T>,
	) -> Vec<TipsBalanceOf<T>> {
		let mut main_tips_balances = Vec::<TipsBalanceOf<T>>::new();
		let ref_type = main_references.get_reference_type();
		let ref_id = &main_references.get_reference_ids()[0];

		for ft_identifier in ft_identifiers.iter() {
			let mut tip: BalanceOf<T> = Zero::zero();
			let reference_type = references.get_reference_type();
			let reference_ids = references.get_reference_ids();

			for reference_id in reference_ids {
				let server_id = server_id.clone();
				let tips_balance_key = (server_id, reference_type, reference_id, ft_identifier);
				if let Some(tips_balance) = Self::tips_balance_by_reference(&tips_balance_key) {
					let amount = *tips_balance.get_amount();
					if amount > Zero::zero() {
						tip = tip.saturating_add(amount);
						Self::do_store_tips_balance(&tips_balance, true, None);
					}
				}
			}

			let main_info = TipsBalanceInfo::new(server_id, ref_type, ref_id, ft_identifier);
			let mut main_balance = TipsBalance::new(&main_info, &tip);

			main_balance.set_account_id(account_id);

			let total_tip = Self::do_store_tips_balance(&main_balance, false, Some(*tx_fee));

			main_balance.set_amount(total_tip);
			main_tips_balances.push(main_balance);
		}

		main_tips_balances
	}

	pub fn asset_id(ft_identifier: &[u8]) -> Result<u32, Error<T>> {
		let str_num =
			String::from_utf8(ft_identifier.to_vec()).map_err(|_| Error::<T>::WrongFormat)?;

		str_num.parse::<u32>().map_err(|_| Error::<T>::WrongFormat)
	}
}
