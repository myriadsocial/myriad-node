#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::{AdminKey, Pallet as Server, ServerInterface};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

const SEED: u32 = 0;

benchmarks! {
	register {
		let s in 1 .. 100;
		let caller: T::AccountId = AdminKey::<T>::get();
		let account_id: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), account_id, vec![s as u8])

	update_name {
		let s in 1 .. 100;
		let caller: T::AccountId = AdminKey::<T>::get();
		let account_id: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let server_id = Server::<T>::generate_server_id(&account_id, &"myriad".as_bytes().to_vec());
		let _server = Server::<T>::register(caller_origin.clone(), account_id.clone(), "myriad".as_bytes().to_vec());
	}: _(RawOrigin::Signed(caller), account_id, server_id, vec![s as u8])

	transfer_owner {
		let caller: T::AccountId = AdminKey::<T>::get();
		let account_id: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let server_id = Server::<T>::generate_server_id(&account_id, &"myriad".as_bytes().to_vec());
		let _server = Server::<T>::register(caller_origin.clone(), account_id.clone(), "myriad".as_bytes().to_vec());
		let new_owner: T::AccountId = account("new_owner", 0, SEED);
	}: _(RawOrigin::Signed(caller), account_id, server_id, new_owner)

	unregister {
		let caller: T::AccountId = AdminKey::<T>::get();
		let account_id: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let server_id = Server::<T>::generate_server_id(&account_id, &"myriad".as_bytes().to_vec());
		let _server = Server::<T>::register(caller_origin.clone(), account_id.clone(), "myriad".as_bytes().to_vec());
	}: _(RawOrigin::Signed(caller), account_id, server_id)

	transfer_admin_key {
		let caller: T::AccountId = AdminKey::<T>::get();
		let account_id: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), account_id)

	force_transfer_admin_key {
		let new_admin: T::AccountId = account("new_admin", 0, SEED);
	}: _(RawOrigin::Root, new_admin)
}

impl_benchmark_test_suite! {Server, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
