use super::*;

#[allow(unused)]
use crate::Pallet as Escrow;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	send_tip {
		let post = Post {
			post_id: String::from("60efac8c565ab8004ed28bb3").into_bytes(),
			people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
			platform: String::from("twitter").into_bytes()
		};

		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: send_tip(RawOrigin::Signed(caller), post, String::from("MYRIA").into_bytes(), s.into())
}

impl_benchmark_test_suite! {Escrow, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
