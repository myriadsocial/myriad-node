use super::*;

use frame_support::{
	sp_runtime::traits::Zero,
	traits::{Currency, ExistenceRequirement, WithdrawReasons},
};

impl<T: Config> TippingInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type TipsBalance = TipsBalanceOf<T>;
	type TipsBalances = (TipsBalanceOf<T>, Option<TipsBalanceOf<T>>);
	type TipsBalanceInfo = TipsBalanceInfo;
	type Balance = BalanceOf<T>;
	type ReferenceId = ReferenceId;
	type ReferenceType = ReferenceType;
	type FtIdentifier = FtIdentifier;

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
				let tips_balance = match Self::get_tips_balance(tips_balance_info) {
					Some(mut result) => {
						let total_amount = *result.get_amount() + tip_amount;

						result.set_amount(total_amount);
						Self::update_tips_balance(&result)
					},
					None => Self::create_tips_balance(tips_balance_info, &None, &Some(tip_amount)),
				};
				let receiver = Self::tipping_account_id();

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

	fn claim_reference(
		sender: &Option<T::AccountId>,
		tips_balance_info: &Self::TipsBalanceInfo,
		reference_type: &Self::ReferenceType,
		reference_id: &Self::ReferenceId,
		account_id: &Option<T::AccountId>,
		verify_owner: bool,
	) -> Result<Self::TipsBalances, Self::Error> {
		let server_id = tips_balance_info.get_server_id();
		let verified = Self::verify_server(sender, server_id, verify_owner);

		if let Err(err) = verified {
			return Err(err)
		}

		if tips_balance_info.get_ft_identifier() != "native".as_bytes() {
			return Err(Error::<T>::FtNotExists)
		}

		let mut tips_balances = Self::default_tips_balances(tips_balance_info);

		if tips_balance_info.get_reference_type() == reference_type {
			if account_id.is_none() {
				return Err(Error::<T>::ReceiverNotExists)
			}

			if tips_balance_info.get_reference_id() != reference_id {
				return Err(Error::<T>::NotExists)
			}

			tips_balances.0 = match Self::get_tips_balance(tips_balance_info) {
				Some(mut result) => {
					result.set_account_id(account_id);
					Self::update_tips_balance(&result)
				},
				None => Self::create_tips_balance(tips_balance_info, account_id, &None),
			};
		} else {
			// Reference from tips balance info
			let mut initial_balance: BalanceOf<T> = Zero::zero();

			tips_balances.1 = match Self::get_tips_balance(tips_balance_info) {
				Some(mut result) => {
					initial_balance += *result.get_amount();

					if !initial_balance.is_zero() {
						result.set_amount(Zero::zero());
						Some(Self::update_tips_balance(&result))
					} else {
						Some(result)
					}
				},
				None => Some(Self::create_tips_balance(tips_balance_info, &None, &None)),
			};

			// Create or update reference from param
			let mut tips_balance_info = tips_balance_info.clone();

			tips_balance_info.set_reference_type(reference_type);
			tips_balance_info.set_reference_id(reference_id);

			tips_balances.0 = match Self::get_tips_balance(&tips_balance_info) {
				Some(mut result) => {
					let total_amount = *result.get_amount() + initial_balance;

					result.set_amount(total_amount);

					if account_id.is_some() {
						result.set_account_id(account_id);
					}

					Self::update_tips_balance(&result)
				},
				None => Self::create_tips_balance(
					&tips_balance_info,
					account_id,
					&Some(initial_balance),
				),
			};
		}

		Ok(tips_balances)
	}
}
