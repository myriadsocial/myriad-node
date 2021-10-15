use super::*;

#[allow(unused)]
use crate::Pallet as Platform;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

benchmarks! {
	add_platform {
		let s in 0 .. 100;
		let t: Vec<u8> = vec![s as u8];
		let caller: T::AccountId = whitelisted_caller();
	}: add_platform(RawOrigin::Signed(caller), t)
}

impl_benchmark_test_suite! {Platform, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
