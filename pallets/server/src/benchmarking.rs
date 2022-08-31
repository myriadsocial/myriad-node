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
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_web_url = "https://app.dev.myriad.social".as_bytes().to_vec();
	}: _(RawOrigin::Signed(caller), vec![s as u8], server_api_url, server_web_url)

	update_name {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0;
		let server_name = "myriad".as_bytes().to_vec();
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_web_url = "https://app.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_name, server_api_url, server_web_url);
		let new_name = "debio".as_bytes().to_vec();
	}: _(RawOrigin::Signed(caller), server_id, new_name)

	update_api_url {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0;
		let server_name = "myriad".as_bytes().to_vec();
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_web_url = "https://app.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_name, server_api_url, server_web_url);
		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
	}: _(RawOrigin::Signed(caller), server_id, new_api_url)

	update_web_url {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0;
		let server_name = "myriad".as_bytes().to_vec();
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_web_url = "https://app.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_name, server_api_url, server_web_url);
		let new_web_url = "https://app.testnet.myriad.social".as_bytes().to_vec();
	}: _(RawOrigin::Signed(caller), server_id, new_web_url)

	transfer_owner {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0;
		let server_name = "myriad".as_bytes().to_vec();
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_web_url = "https://app.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_name, server_api_url, server_web_url);
		let new_owner: T::AccountId = account("new_owner", 0, SEED);
	}: _(RawOrigin::Signed(caller), server_id, new_owner)

	unregister {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let server_id = 0;
		let server_name = "myriad".as_bytes().to_vec();
		let server_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_web_url = "https://app.dev.myriad.social".as_bytes().to_vec();

		let _ = Server::<T>::register(caller_origin.clone(), server_name, server_api_url, server_web_url);
	}: _(RawOrigin::Signed(caller), server_id)
}

impl_benchmark_test_suite! {Server, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
