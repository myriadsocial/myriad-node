#![cfg_attr(not(feature = "std"), no_std)]

mod mock;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{sp_runtime::SaturatedConversion, traits::Currency};
use frame_system::RawOrigin;
use pallet_server::{AdminKey, Config as ServerConfig, Pallet as Server};
use pallet_tipping::{
	Call, Config as TippingConfig, Pallet as Tipping, References, TipsBalanceInfo,
};
use sp_std::vec;

const SEED: u32 = 0;

pub struct Pallet<T: Config>(Tipping<T>);

pub trait Config: TippingConfig + ServerConfig {}

benchmarks! {
	send_tip {
		let balance = 1000000000000000000000u128.saturated_into();
		let caller: T::AccountId = whitelisted_caller();
		let _ = <T as TippingConfig>::Currency::deposit_creating(&caller, balance);

		let admin: T::AccountId = AdminKey::<T>::get().unwrap();
		let server_account: T::AccountId = account("server_account", 0, SEED);
		let server_name = b"myriad".to_vec();
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let server_id = b"server".to_vec();
		let server_api_url = b"https://api.dev.myriad.social".to_vec();
		let server_web_url = b"https://app.dev.myriad.social".to_vec();
		let _server = Server::<T>::register(
			admin_origin,
			server_account,
			server_id.clone(),
			server_name,
			server_api_url,
			server_web_url
		);

		let amount = 1000000000000000u128.saturated_into();
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
		let caller: T::AccountId = whitelisted_caller();
		let tipping_account_id = Tipping::<T>::tipping_account_id();

		let balance = 1000000000000000000000u128.saturated_into();
		let _ = <T as TippingConfig>::Currency::deposit_creating(&tipping_account_id, balance);

		let admin: T::AccountId = AdminKey::<T>::get().unwrap();
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let server_name = b"myriad".to_vec();
		let server_id = b"server".to_vec();
		let server_api_url = b"https://api.dev.myriad.social".to_vec();
		let server_web_url = b"https://app.dev.myriad.social".to_vec();
		let _ = Server::<T>::register(
			admin_origin,
			caller.clone(),
			server_id.clone(),
			server_name,
			server_api_url,
			server_web_url
		);

		// Send Tipping
		let account_1: T::AccountId = account("account_1", 0, SEED);
		let account_1_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_1.clone()));
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"people",
			b"people_id",
			b"native"
		);

		let tipping_amount = 10000000000000000000u128.saturated_into();
		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_1, balance);
		let _ = Tipping::<T>::send_tip(account_1_origin, tips_balance_info, tipping_amount);

		// Send Tipping
		let account_2: T::AccountId = account("account_2", 0, SEED);
		let account_2_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_2.clone()));
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"user",
			b"user_id",
			b"native"
		);

		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_2, balance);
		let _ = Tipping::<T>::send_tip(account_2_origin, tips_balance_info, tipping_amount);

		let trx_fee = 10000000000000u128.saturated_into();
		let account_3: T::AccountId = account("account_3", 0, SEED);
		let references = References::new(b"people", &[b"people_id".to_vec()]);
		let main_references = References::new(b"user", &[b"user_id".to_vec()]);
		let ft_identifiers = vec![b"native".to_vec()];
	}: _(RawOrigin::Signed(caller), server_id, references, main_references, ft_identifiers, account_3, trx_fee)

	claim_tip {
		let caller: T::AccountId = whitelisted_caller();
		let tipping_account_id = Tipping::<T>::tipping_account_id();

		let balance = 1000000000000000000000u128.saturated_into();
		let _ = <T as TippingConfig>::Currency::deposit_creating(&caller, balance);
		let _ = <T as TippingConfig>::Currency::deposit_creating(&tipping_account_id, balance);

		// Register Server
		// Server admin => server_account
		let admin: T::AccountId = AdminKey::<T>::get().unwrap();
		let server_account: T::AccountId = account("server_account", 0, SEED);
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let server_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(server_account.clone()));
		let server_id = b"server".to_vec();
		let server_api_url = b"https://api.dev.myriad.social".to_vec();
		let server_web_url = b"https://app.dev.myriad.social".to_vec();

		let _ = Server::<T>::register(
			admin_origin,
			server_account,
			server_id.clone(),
			b"myriad".to_vec(),
			server_api_url,
			server_web_url
		);

		// Send Tipping
		let account_1: T::AccountId = account("account", 0, SEED);
		let account_1_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_1.clone()));
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"people",
			b"people_id",
			b"native"
		);

		let tipping_amount = 10000000000000000000u128.saturated_into();
		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_1, balance);
		let _ = Tipping::<T>::send_tip(account_1_origin, tips_balance_info, tipping_amount);

		// Claim Reference
		let account_2: T::AccountId = account("account", 2, SEED);
		let account_2_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(account_2.clone()));
		let tips_balance_info_user = TipsBalanceInfo::new(
			&server_id,
			b"user",
			b"user_id",
			b"native"
		);
		let _ = <T as TippingConfig>::Currency::deposit_creating(&account_2, balance);
		let _ = Tipping::<T>::send_tip(account_2_origin, tips_balance_info_user, tipping_amount);

		let tx_fee = 10000000000000u128.saturated_into();
		let _ = Tipping::<T>::claim_reference(
			server_origin,
			b"server".to_vec(),
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec()],
			caller.clone(),
			tx_fee,
		);
	}: _(RawOrigin::Signed(caller), server_id, b"user".to_vec(), b"user_id".to_vec(), vec![b"native".to_vec()])
}
