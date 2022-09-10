#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::{Config, Pallet as Tipping, TippingInterface};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{sp_runtime::SaturatedConversion, traits::Currency};
use frame_system::RawOrigin;
use sp_std::vec;

const SEED: u32 = 0;

benchmarks! {
	send_tip {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let server_id: T::AccountId = account("server_account", 0, SEED);

		// Default balance
		let balance = 1000000000000000000000u128.saturated_into();
		let amount = 1000000000000000u128.saturated_into();

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		// Send tip
		let reference_id = b"people_id".to_vec();
		let reference_type = b"people".to_vec();
		let ft_identifier = b"native".to_vec();
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			&reference_type,
			&reference_id,
			&ft_identifier
		);
	}: _(RawOrigin::Signed(caller), tips_balance_info, amount)

	claim_reference {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let account_1: T::AccountId = account("account_1", 0, SEED);
		let account_2: T::AccountId = account("account_2", 0, SEED);
		let account_3: T::AccountId = account("account_3", 0, SEED);
		let tipping_account_id: T::AccountId = Tipping::<T>::tipping_account_id();

		// Default balance
		let balance = 1000000000000000000000u128.saturated_into();
		let tipping_amount = 10000000000000000000u128.saturated_into();

		// Tipping_account_id, account_1, and account_2 initial balance
		let _ = <T as Config>::Currency::deposit_creating(&account_1, balance);
		let _ = <T as Config>::Currency::deposit_creating(&account_2, balance);
		let _ = <T as Config>::Currency::deposit_creating(&tipping_account_id, balance);

		// Send Tip by account_1
		let account_1_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_1));
		let tips_balance_info = TipsBalanceInfo::new(
			&caller,
			b"people",
			b"people_id",
			b"native"
		);
		let _ = Tipping::<T>::send_tip(account_1_origin, tips_balance_info, tipping_amount);

		// Send Tip by account_2
		let account_2_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_2));
		let tips_balance_info = TipsBalanceInfo::new(
			&caller,
			b"user",
			b"user_id",
			b"native"
		);
		let _ = Tipping::<T>::send_tip(account_2_origin, tips_balance_info, tipping_amount);

		// Claim reference data
		let server_id = caller.clone();
		let trx_fee = 10000000000000u128.saturated_into();
		let references = References::new(b"people", &[b"people_id".to_vec()]);
		let main_references = References::new(b"user", &[b"user_id".to_vec()]);
		let ft_identifiers = vec![b"native".to_vec()];
	}: _(RawOrigin::Signed(caller), server_id, references, main_references, ft_identifiers,
account_3, trx_fee)

	claim_tip {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let account_1: T::AccountId = account("account", 0, SEED);
		let account_2: T::AccountId = account("account", 2, SEED);
		let server_id: T::AccountId = account("server_account", 0, SEED);
		let tipping_account_id: T::AccountId = Tipping::<T>::tipping_account_id();

		// Default balance
		let balance = 1000000000000000000000u128.saturated_into();
		let tipping_amount = 10000000000000000000u128.saturated_into();

		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);
		let _ = <T as Config>::Currency::deposit_creating(&account_1, balance);
		let _ = <T as Config>::Currency::deposit_creating(&account_2, balance);
		let _ = <T as Config>::Currency::deposit_creating(&tipping_account_id, balance);

		// Send Tip
		let account_1_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_1));
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"people",
			b"people_id",
			b"native"
		);
		let _ = Tipping::<T>::send_tip(account_1_origin, tips_balance_info, tipping_amount);

		// Send Tip
		let account_2_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_2));
		let tips_balance_info_user = TipsBalanceInfo::new(
			&server_id,
			b"user",
			b"user_id",
			b"native"
		);
		let _ = Tipping::<T>::send_tip(account_2_origin, tips_balance_info_user, tipping_amount);

		// Claim Reference
		let server_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(server_id.clone()));
		let tx_fee = 10000000000000u128.saturated_into();
		let _ = Tipping::<T>::claim_reference(
			server_origin,
			server_id.clone(),
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec()],
			caller.clone(),
			tx_fee,
		);
	}: _(RawOrigin::Signed(caller), server_id, b"user".to_vec(), b"user_id".to_vec(),
vec![b"native".to_vec()]) }

impl_benchmark_test_suite! {Server, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
