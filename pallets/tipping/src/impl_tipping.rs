use super::*;

use frame_support::{dispatch::DispatchError, sp_runtime::traits::Zero};
use sp_std::vec::Vec;

impl<T: Config> TippingInterface<T> for Pallet<T> {
	type Error = DispatchError;
	type TipsBalanceInfo = TipsBalanceInfoOf<T>;
	type TipsBalanceKey = TipsBalanceKeyOf<T>;
	type Balance = BalanceOf<T>;
	type ServerId = ServerIdOf<T>;
	type References = References;
	type FtIdentifier = FtIdentifier;
	type FtIdentifiers = Vec<FtIdentifier>;
	type SendTipResult = (AccountIdOf<T>, TipsBalanceTuppleOf<T>);
	type ClaimTipResult = (AccountIdOf<T>, AccountBalancesTuppleOf<T>);
	type ClaimReferenceResult = Vec<TipsBalanceOf<T>>;
	type Receipt = ReceiptOf<T>;
	type WithdrawalResult = Vec<(FtIdentifier, BalanceOf<T>)>;

	fn pay_content(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::Receipt, Self::Error> {
		if sender == receiver {
			return Err(DispatchError::BadOrigin)
		}

		let fee = Self::can_pay_content(sender, amount)?;
		let tips_balance_info = TipsBalanceInfo::new(
			tips_balance_info.get_server_id(),
			b"unlockable_content",
			tips_balance_info.get_reference_id(),
			tips_balance_info.get_ft_identifier(),
		);

		let escrow_id = Self::tipping_account_id();
		let ft_identifier = tips_balance_info.get_ft_identifier();

		Self::do_transfer(ft_identifier, sender, receiver, *amount)?;
		Self::do_transfer(ft_identifier, sender, &escrow_id, fee)?;

		Self::do_update_withdrawal_balance(ft_identifier, fee);
		let receipt = Self::do_store_receipt(sender, receiver, &tips_balance_info, amount, &fee);

		Ok(receipt)
	}

	fn withdrawal_balance(
		sender: &T::AccountId,
		receiver: &T::AccountId,
	) -> Result<Self::WithdrawalResult, Self::Error> {
		let mut success_withdrawal = Vec::new();

		WithdrawalBalance::<T>::translate(|ft: Vec<u8>, amount: BalanceOf<T>| {
			if amount.is_zero() {
				return None
			}

			let result = Self::do_transfer(&ft, sender, receiver, amount);

			if result.is_err() {
				return Some(amount)
			}

			success_withdrawal.push((ft, amount));

			None
		});

		Ok(success_withdrawal)
	}

	fn send_tip(
		sender: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::SendTipResult, Self::Error> {
		let receiver = Self::tipping_account_id();
		let tip_amount = *amount;
		let ft_identifier = tips_balance_info.get_ft_identifier();
		let tips_balance = TipsBalance::new(tips_balance_info, amount);

		Self::do_transfer(ft_identifier, sender, &receiver, tip_amount)?;
		Self::do_store_tips_balance(&tips_balance, false, None);

		Ok((receiver, (tips_balance.key(), tip_amount)))
	}

	fn claim_tip(
		receiver: &T::AccountId,
		tips_balance_key: &Self::TipsBalanceKey,
		ft_identifiers: &Self::FtIdentifiers,
	) -> Result<Self::ClaimTipResult, Self::Error> {
		let sender = Self::tipping_account_id();

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

			match Self::do_transfer(ft, &sender, receiver, amount) {
				Ok(_) => {
					Self::do_store_tips_balance(&tips_balance, true, None);

					success_claim.push((ft.to_vec(), receiver.clone(), amount));
				},
				Err(_) => failed_claim.push((ft.to_vec(), receiver.clone(), amount)),
			};
		}

		if failed_claim.is_empty() {
			return Ok((sender, (success_claim, None)))
		}

		Ok((sender, (success_claim, Some(failed_claim))))
	}

	fn claim_reference(
		receiver: &T::AccountId,
		server_id: &Self::ServerId,
		references: &Self::References,
		main_references: &Self::References,
		ft_identifiers: &Self::FtIdentifiers,
		account_id: &T::AccountId,
		tx_fee: &Self::Balance,
	) -> Result<Self::ClaimReferenceResult, Self::Error> {
		let ref_type = main_references.get_reference_type().clone();
		let ref_ids = main_references.get_reference_ids().clone();

		if receiver == account_id {
			return Err(DispatchError::BadOrigin)
		}

		let sender = Self::tipping_account_id();
		let ref_id = ref_ids[0].clone();
		let tips_balance_key = (server_id.clone(), ref_type, ref_id, b"native".to_vec());

		Self::can_pay_fee(&tips_balance_key, tx_fee)?;
		Self::do_transfer(b"native", &sender, receiver, *tx_fee)?;

		let tips_balances = Self::do_store_tips_balances(
			server_id,
			references,
			main_references,
			ft_identifiers,
			account_id,
			tx_fee,
		);

		Ok(tips_balances)
	}
}
