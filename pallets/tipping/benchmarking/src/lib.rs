#![cfg_attr(not(feature = "std"), no_std)]

mod mock;

const SEED: u32 = 0;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{sp_runtime::SaturatedConversion, traits::Currency};
use frame_system::RawOrigin;
use pallet_server::{AdminKey, Config as ServerConfig, Pallet as Server};
#[allow(unused)]
use pallet_tipping::{Call, Config as TippingConfig, Pallet as Tipping, TipsBalance, TipsBalanceInfo};

pub struct Pallet<T: Config>(Tipping<T>);

pub trait Config: TippingConfig + ServerConfig {}

benchmarks! {
	send_tip {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let _ = <T as TippingConfig>::Currency::deposit_creating(&caller, 1000000000000000000000u128.saturated_into());

		let admin: T::AccountId = AdminKey::<T>::get();
		let server_account: T::AccountId = account("server_account", 0, SEED);
		let server_name = "myriad".as_bytes().to_vec();
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let server_id = Server::<T>::generate_server_id(&server_account, &server_name);
		let _server = Server::<T>::register(admin_origin, server_account, server_name);

		let reference_id = "people_id".as_bytes().to_vec();
		let reference_type = "people".as_bytes().to_vec();
		let ft_identifier = "native".as_bytes().to_vec();
		let tips_balance_info = TipsBalanceInfo::new(&server_id, &reference_type, &reference_id, &ft_identifier);
	}: _(RawOrigin::Signed(caller), tips_balance_info, s.into())

	claim_reference {
		let caller: T::AccountId = whitelisted_caller();

		let admin: T::AccountId = AdminKey::<T>::get();
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let server_name = "myriad".as_bytes().to_vec();
		let server_id = Server::<T>::generate_server_id(&caller, &server_name);
		let _server = Server::<T>::register(admin_origin, caller.clone(), server_name);

		let reference_id = "people_id".as_bytes().to_vec();
		let reference_type = "people".as_bytes().to_vec();
		let ft_identifier = "native".as_bytes().to_vec();
		let tips_balance_info = TipsBalanceInfo::new(&server_id, &reference_type, &reference_id, &ft_identifier);
	}: _(RawOrigin::Signed(caller), tips_balance_info, "user".as_bytes().to_vec(), "user_id".as_bytes().to_vec(), None)

	claim_tip {
		let caller: T::AccountId = whitelisted_caller();
		let tipping_account_id = Tipping::<T>::tipping_account_id();

		let _ = <T as TippingConfig>::Currency::deposit_creating(&caller, 1000000000000000000000u128.saturated_into());
		let _ = <T as TippingConfig>::Currency::deposit_creating(&tipping_account_id, 1000000000000000000000u128.saturated_into());

		// Register Server
		// Server admin => server_account
		let admin: T::AccountId = AdminKey::<T>::get();
		let server_account: T::AccountId = account("server_account", 0, SEED);
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let server_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(server_account.clone()));
		let server_id = Server::<T>::generate_server_id(&server_account, "myriad".as_bytes());

		let _ = Server::<T>::register(admin_origin, server_account, "myriad".as_bytes().to_vec());

		// Send Tipping
		let account_1: T::AccountId = account("account", 0, SEED);
		let account_1_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_1.clone()));
		let tips_balance_info = TipsBalanceInfo::new(&server_id, "people".as_bytes(), "people_id".as_bytes(), "native".as_bytes());

		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_1, 1000000000000000000000u128.saturated_into());
		let _ = Tipping::<T>::send_tip(account_1_origin, tips_balance_info.clone(), 10000000000000000000u128.saturated_into());

		// Claim Reference
		let tips_balance_info_user = TipsBalanceInfo::new(&server_id, "user".as_bytes(), "user_id".as_bytes(), "native".as_bytes());

		let _ = Tipping::<T>::claim_reference(server_origin.clone(), tips_balance_info_user.clone(), "user".as_bytes().to_vec(), "user_id".as_bytes().to_vec(), Some(caller.clone()));
		let _ = Tipping::<T>::claim_reference(server_origin, tips_balance_info, "user".as_bytes().to_vec(), "user_id".as_bytes().to_vec(), None);
	}: _(RawOrigin::Signed(caller), tips_balance_info_user)
}
