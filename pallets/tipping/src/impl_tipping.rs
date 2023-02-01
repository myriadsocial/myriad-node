use super::*;

use frame_support::{dispatch::DispatchError, sp_runtime::traits::Zero};
use sp_std::vec::Vec;

impl<T: Config> TippingInterface<T> for Pallet<T> {
	type Error = DispatchError;
	type TipsBalance = TipsBalanceOf<T>;
	type TipsBalanceInfo = TipsBalanceInfoOf<T>;
	type TipsBalanceKey = TipsBalanceKeyOf<T>;
	type Balance = BalanceOf<T>;
	type References = References;
	type Receipt = ReceiptOf<T>;
	type WithdrawalResult = Vec<(FtIdentifier, BalanceOf<T>)>;

	fn pay_content(
		sender: &T::AccountId,
		instance_id: u64,
		receiver: &Option<T::AccountId>,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
		account_reference: &Option<Vec<u8>>,
	) -> Result<Self::Receipt, Self::Error> {
		if let Some(receiver) = receiver {
			if sender == receiver {
				return Err(DispatchError::BadOrigin)
			}
		}

		let fee_detail = Self::can_pay_content(sender, amount)?;
		let admin_fee = fee_detail.admin_fee();
		let server_fee = fee_detail.server_fee();
		let total_fee = fee_detail.total_fee();

		let info = TipsBalanceInfo::new(
			tips_balance_info.get_server_id(),
			b"unlockable_content",
			tips_balance_info.get_reference_id(),
			tips_balance_info.get_ft_identifier(),
		);

		let escrow_id = Self::tipping_account_id();
		let ft_identifier = info.get_ft_identifier();

		if let Some(receiver) = receiver {
			Self::do_transfer(ft_identifier, sender, receiver, *amount)?;
			Self::do_transfer(ft_identifier, sender, &escrow_id, total_fee)?;
		} else {
			if account_reference.is_none() {
				return Err(DispatchError::BadOrigin)
			}

			let account_reference = account_reference.as_ref().unwrap();
			let account_info = TipsBalanceInfo::new(
				tips_balance_info.get_server_id(),
				b"user",
				account_reference,
				tips_balance_info.get_ft_identifier(),
			);
			let tips_balance = TipsBalance::new(&account_info, amount);

			Self::do_transfer(ft_identifier, sender, &escrow_id, *amount + total_fee)?;
			Self::do_store_tips_balance(&tips_balance, false, None);
		}

		Self::do_update_withdrawal_balance(ft_identifier, admin_fee);
		Self::do_update_reward_balance(instance_id, &info, server_fee);

		let now = T::TimeProvider::now().as_millis();
		let receipt = Receipt::new(sender, receiver, &info, amount, &total_fee, now);

		Ok(receipt)
	}

	fn withdraw_fee(
		sender: &T::AccountId,
		receiver: &T::AccountId,
	) -> Result<(Self::WithdrawalResult, Self::WithdrawalResult), Self::Error> {
		let mut success_withdrawal = Vec::new();
		let mut failed_withdrawal = Vec::new();

		WithdrawalBalance::<T>::translate(|ft: Vec<u8>, amount: BalanceOf<T>| {
			if amount.is_zero() {
				return None
			}

			let result = Self::do_transfer(&ft, sender, receiver, amount);

			if result.is_err() {
				failed_withdrawal.push((ft, amount));
				Some(amount)
			} else {
				success_withdrawal.push((ft, amount));
				None
			}
		});

		Ok((success_withdrawal, failed_withdrawal))
	}

	fn withdraw_reward(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		instance_id: u64,
	) -> Result<(Self::WithdrawalResult, Self::WithdrawalResult), Self::Error> {
		let mut success_withdrawal = Vec::new();
		let mut failed_withdrawal = Vec::new();

		for (ft_identifier, amount) in RewardBalance::<T>::drain_prefix((receiver, instance_id)) {
			let result = Self::do_transfer(&ft_identifier, sender, receiver, amount);

			if result.is_ok() {
				success_withdrawal.push((ft_identifier, amount));
			} else {
				failed_withdrawal.push((ft_identifier, amount));
			}
		}

		// Reinsert again failed transfer
		for (ft_identifier, amount) in failed_withdrawal.iter() {
			RewardBalance::<T>::insert((receiver, instance_id, ft_identifier), amount);
		}

		Ok((success_withdrawal, failed_withdrawal))
	}

	fn send_tip(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::TipsBalance, Self::Error> {
		let tip_amount = *amount;
		let ft_identifier = tips_balance_info.get_ft_identifier();
		let tips_balance = TipsBalance::new(tips_balance_info, amount);

		Self::do_transfer(ft_identifier, sender, receiver, tip_amount)?;
		Self::do_store_tips_balance(&tips_balance, false, None);

		Ok(tips_balance)
	}

	fn claim_tip(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		tips_balance_key: &Self::TipsBalanceKey,
		ft_identifiers: &[Vec<u8>],
	) -> Result<(Self::WithdrawalResult, Self::WithdrawalResult), Self::Error> {
		let mut tips_balance_key = tips_balance_key.clone();
		let mut success_claim = Vec::new();
		let mut failed_claim = Vec::new();

		for ft in ft_identifiers.iter() {
			tips_balance_key.3 = ft.clone();

			let can_claim_tip = Self::can_claim_tip(&tips_balance_key, receiver);

			if can_claim_tip.is_none() {
				continue
			}

			let tips_balance = can_claim_tip.unwrap();
			let amount = *tips_balance.get_amount();

			match Self::do_transfer(ft, sender, receiver, amount) {
				Ok(_) => {
					Self::do_store_tips_balance(&tips_balance, true, None);

					success_claim.push((ft.to_vec(), amount));
				},
				Err(_) => failed_claim.push((ft.to_vec(), amount)),
			};
		}

		Ok((success_claim, failed_claim))
	}

	fn claim_reference(
		receiver: &T::AccountId,
		server_id: &T::AccountId,
		references: &Self::References,
		account_references: &Self::References,
		ft_identifiers: &[Vec<u8>],
		account_id: &T::AccountId,
		tx_fee: &Self::Balance,
	) -> Result<Vec<Self::TipsBalance>, Self::Error> {
		let account_ref_type = account_references.get_reference_type().clone();
		let account_ref_ids = account_references.get_reference_ids().clone();

		if receiver == account_id {
			return Err(DispatchError::BadOrigin)
		}

		if account_ref_ids.is_empty() {
			return Err(DispatchError::BadOrigin)
		}

		// Pay Fee to Server Admin
		let sender = Self::tipping_account_id();
		let account_reference_id = account_ref_ids[0].clone();
		let key = (server_id.clone(), account_ref_type, account_reference_id, b"native".to_vec());

		Self::can_pay_fee(&key, tx_fee)?;
		Self::do_transfer(b"native", &sender, receiver, *tx_fee)?;

		// Recap total tips belong to account
		let tips_balances = Self::do_store_tips_balances(
			server_id,
			references,
			account_references,
			ft_identifiers,
			account_id,
			tx_fee,
		);

		Ok(tips_balances)
	}
}
