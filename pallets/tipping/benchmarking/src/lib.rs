#![cfg_attr(not(feature = "std"), no_std)]

mod mock;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{sp_runtime::SaturatedConversion, traits::Currency};
use frame_system::RawOrigin;
use pallet_server::{Config as ServerConfig, Pallet as Server};
use pallet_tipping::{
	Call, Config as TippingConfig, Pallet as Tipping, References, TipsBalanceInfo,
};
use sp_std::vec;

const SEED: u32 = 0;

pub struct Pallet<T: Config>(Tipping<T>);

pub trait Config: TippingConfig + ServerConfig {}

benchmarks! {
	send_tip {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();

		// Default balance
		let balance = 1000000000000000000000u128.saturated_into();
		let amount = 1000000000000000u128.saturated_into();

		// Caller initial balance
		let _ = <T as TippingConfig>::Currency::deposit_creating(&caller, balance);

		// Generate server id
		let server_id = b"0".to_vec();

		// Register server id
		let server_account: T::AccountId = account("server_account", 0, SEED);
		let server_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(server_account));
		let server_api_url = b"https://api.dev.myriad.social".to_vec();
		let _ = Server::<T>::register(server_origin, server_api_url);

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
		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_1, balance);
		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_2, balance);
		let _ = <T as TippingConfig>::Currency::deposit_creating(&tipping_account_id, balance);

		// Generate Server Id
		let server_id = b"0".to_vec();

		// Register Server
		let server_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let server_api_url = b"https://api.dev.myriad.social".to_vec();
		let _ = Server::<T>::register(server_origin, server_api_url);

		// Send Tip by account_1
		let account_1_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_1));
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"people",
			b"people_id",
			b"native"
		);
		let _ = Tipping::<T>::send_tip(account_1_origin, tips_balance_info, tipping_amount);

		// Send Tip by account_2
		let account_2_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_2));
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"user",
			b"user_id",
			b"native"
		);
		let _ = Tipping::<T>::send_tip(account_2_origin, tips_balance_info, tipping_amount);

		// Claim reference data
		let trx_fee = 10000000000000u128.saturated_into();
		let references = References::new(b"people", &[b"people_id".to_vec()]);
		let main_references = References::new(b"user", &[b"user_id".to_vec()]);
		let ft_identifiers = vec![b"native".to_vec()];
	}: _(RawOrigin::Signed(caller), server_id, references, main_references, ft_identifiers, account_3, trx_fee)

	claim_tip {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let account_1: T::AccountId = account("account", 0, SEED);
		let account_2: T::AccountId = account("account", 2, SEED);
		let server_account: T::AccountId = account("server_account", 0, SEED);
		let tipping_account_id: T::AccountId = Tipping::<T>::tipping_account_id();

		// Default balance
		let balance = 1000000000000000000000u128.saturated_into();
		let tipping_amount = 10000000000000000000u128.saturated_into();

		let _ = <T as TippingConfig>::Currency::deposit_creating(&caller, balance);
		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_1, balance);
		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_2, balance);
		let _ = <T as TippingConfig>::Currency::deposit_creating(&tipping_account_id, balance);

		// Generate Server Id
		let server_id = b"0".to_vec();

		// Register Server
		let server_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(server_account));
		let server_api_url = b"https://api.dev.myriad.social".to_vec();
		let _ = Server::<T>::register(server_origin.clone(), server_api_url);

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
	}: _(RawOrigin::Signed(caller), server_id, b"user".to_vec(), b"user_id".to_vec(), vec![b"native".to_vec()])
}
