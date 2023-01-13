#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::{Action, ActionType, Pallet as Server, ServerInterface};

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	sp_runtime::SaturatedConversion,
	traits::{Currency, Get, OnInitialize},
};
use frame_system::{Pallet as System, RawOrigin};
use sp_std::vec;

const SEED: u32 = 0;

benchmarks! {
	register {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA
		let stake_amount = 60_000_000_000_000_000_000_000u128.saturated_into();

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);
	}: _(RawOrigin::Signed(caller), vec![s as u8], Some(stake_amount))

	update_server {
		let new_owner = account("new_owner", 0, SEED);
		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
		let new_stake_amount = 10_000_000_000_000_000_000u128.saturated_into(); // 10 MYRIA
		let action_types = vec![ActionType::StakeAmount(new_stake_amount), ActionType::UpdateApiUrl(new_api_url), ActionType::TransferOwner(new_owner)];

		let s in 0 .. 2;
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url, None);
	}: _(RawOrigin::Signed(caller), server_id, action_types[s as usize].clone())

	unregister {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url, None);
	}: _(RawOrigin::Signed(caller), server_id)

	cancel_unregister {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url, None);
		let _ = Server::<T>::unregister(caller_origin, server_id);
	}: _(RawOrigin::Signed(caller), server_id)

	on_initialize_server {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url, None);

		// Current block
		let current_block = System::<T>::block_number();

		let _ = Server::<T>::unregister(caller_origin, server_id);

		let scheduled_block_number = current_block + T::ScheduledBlockTime::get();

		// Set blocknumber
		System::<T>::set_block_number(scheduled_block_number);
	}: {
		Server::<T>::on_initialize(scheduled_block_number)
	}
}

impl_benchmark_test_suite! {Server, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
