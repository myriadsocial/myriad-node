#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::{Pallet as Server, ServerInterface};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

const SEED: u32 = 0;

benchmarks! {
	register {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), vec![s as u8])

	update_api_url {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0_u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);
		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
	}: _(RawOrigin::Signed(caller), server_id, new_api_url)

	transfer_owner {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0_u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);
		let new_owner: T::AccountId = account("new_owner", 0, SEED);
	}: _(RawOrigin::Signed(caller), server_id, new_owner)

	unregister {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0_u64;
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_api_url);
	}: _(RawOrigin::Signed(caller), server_id)
}

impl_benchmark_test_suite! {Server, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
