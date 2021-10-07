use super::*;

#[allow(unused)]
use crate::Pallet as Escrow;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

benchmarks! {
	send_tip {
		let post = Post {
			post_id: vec![b'X';256],
			people_id: vec![b'X';256],
			platform: vec![b'X';256],
		};

		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: send_tip(RawOrigin::Signed(caller), post, vec![b'X';256], s.into())
}

impl_benchmark_test_suite! {Escrow, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
