use super::*;

#[allow(unused)]
use crate::Pallet as Platform;
use frame_system::RawOrigin;
use frame_system::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};

benchmarks! {
	add_platform {
		let caller: T::AccountId = whitelisted_caller();
	}: add_platform(RawOrigin::Signed(caller), String::from("twitter").into_bytes())
}

impl_benchmark_test_suite!(Platform, crate::mock::ExternalityBuilder::build(), crate::mock::Test)
