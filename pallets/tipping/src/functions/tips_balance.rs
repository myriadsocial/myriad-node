use crate::*;

use frame_support::sp_runtime::traits::Zero;

impl<T: Config> Pallet<T> {
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
}
