use super::*;

#[allow(unused)]
use crate::Pallet as Platform;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_platform {
		let s in 0 .. 100;
		let t = format!("{}{}", "twitter", s);
		let caller: T::AccountId = whitelisted_caller();
	}: add_platform(RawOrigin::Signed(caller), t.into_bytes())
}

impl_benchmark_test_suite! {Platform, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
