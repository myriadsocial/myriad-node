use crate::*;

use frame_support::sp_runtime::traits::Zero;
use sp_std::vec::Vec;

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

	pub fn update_tips_balances(
		server_id: &[u8],
		references: &References,
		main_references: &References,
		ft_identifiers: &[FtIdentifier],
		account_id: &AccountIdOf<T>,
		tx_fee: &BalanceOf<T>,
		tips_balance: &TipsBalanceOf<T>,
	) -> Vec<TipsBalanceOf<T>> {
		let mut main_tips_balances = Vec::<TipsBalanceOf<T>>::new();
		let ref_type = main_references.get_reference_type();
		let ref_id = &main_references.get_reference_ids()[0];
		for ft_identifier in ft_identifiers.iter() {
			// Remove when other token is implemented
			// Check existing currency
			if ft_identifier != b"native" {
				continue
			}

			let mut tip: BalanceOf<T> = Zero::zero();
			let reference_type = references.get_reference_type();
			let reference_ids = references.get_reference_ids();
			for reference_id in reference_ids {
				let tips_balance_info =
					TipsBalanceInfo::new(server_id, reference_type, reference_id, ft_identifier);

				if let Some(mut tips_balance) = Self::get_tips_balance(&tips_balance_info) {
					if *tips_balance.get_amount() > Zero::zero() {
						tip += *tips_balance.get_amount();
						tips_balance.set_amount(Zero::zero());
						Self::update_tips_balance(&tips_balance);
					}
				}
			}

			let main_info = TipsBalanceInfo::new(server_id, ref_type, ref_id, ft_identifier);
			let main_balance = if ft_identifier != b"native" {
				match Self::get_tips_balance(&main_info) {
					Some(mut res) => {
						let amount = *res.get_amount();
						res.set_amount(tip + amount);
						res.set_account_id(&Some(account_id.clone()));
						res
					},
					None => TipsBalance::new(&main_info, &Some(account_id.clone()), &tip),
				}
			} else {
				let mut tips_balance = tips_balance.clone();
				let amount = *tips_balance.get_amount();

				tips_balance.set_amount(tip + amount - *tx_fee);
				tips_balance.set_account_id(&Some(account_id.clone()));
				tips_balance.clone()
			};

			Self::update_tips_balance(&main_balance);
			main_tips_balances.push(main_balance);
		}

		main_tips_balances
	}
}
