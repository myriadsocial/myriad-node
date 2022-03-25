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
	}: register(RawOrigin::Signed(caller), vec![s as u8])

	update_name {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let server_id = Server::<T>::generate_server_id(&caller, &"myriad".as_bytes().to_vec());
		let _server = Server::<T>::register(caller_origin.clone(), "myriad".as_bytes().to_vec());
	}: update_name(RawOrigin::Signed(caller), server_id, vec![s as u8])

	transfer_owner {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let server_id = Server::<T>::generate_server_id(&caller, &"myriad".as_bytes().to_vec());
		let _server = Server::<T>::register(caller_origin.clone(), "myriad".as_bytes().to_vec());
		let new_owner: T::AccountId = account("new_owner", 0, SEED);
	}: transfer_owner(RawOrigin::Signed(caller), server_id, new_owner)

	unregister {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let server_id = Server::<T>::generate_server_id(&caller, &"myriad".as_bytes().to_vec());
		let _server = Server::<T>::register(caller_origin.clone(), "myriad".as_bytes().to_vec());
	}: unregister(RawOrigin::Signed(caller), server_id)
}

impl_benchmark_test_suite! {Server, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
