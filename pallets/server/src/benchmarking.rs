#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::{Action, Pallet as Server, ServerInterface};

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

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);
	}: _(RawOrigin::Signed(caller), vec![s as u8])

	update_api_url {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);
		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
	}: _(RawOrigin::Signed(caller), server_id, new_api_url)

	transfer_owner {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);
		let new_owner: T::AccountId = account("new_owner", 0, SEED);
	}: _(RawOrigin::Signed(caller), server_id, new_owner)

	unregister {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);
	}: _(RawOrigin::Signed(caller), server_id)

	update_stake_amount {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);
		let stake_amount = 10_000_000_000_000_000_000u128.saturated_into(); // 10 MYRIA
	}: _(RawOrigin::Signed(caller), server_id, Action::Stake(stake_amount))

	on_initialize_server {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		// Default balance
		let balance = 100_000_000_000_000_000_000_000u128.saturated_into(); // 100_000 MYRIA

		// Caller initial balance
		let _ = <T as Config>::Currency::deposit_creating(&caller, balance);

		let server_id = 0u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);

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
