use super::*;

#[allow(unused)]
use crate::Pallet as Currency;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

const SEED: u32 = 0;

benchmarks! {
	add_currency {
		let caller: T::AccountId = whitelisted_caller();
	}: add_currency(
		RawOrigin::Signed(caller),
		vec![b'X';256],
		18,
		vec![b'X';256],
		true
	)

	update_balance {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recepient", 0, SEED);
	}: update_balance(
		RawOrigin::Signed(caller),
		recipient,
		vec![b'X';256],
		s.into()
	)

	transfer {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recepient", 0, SEED);
	}: transfer(
		RawOrigin::Signed(caller),
		recipient,
		vec![b'X';256],
		s.into()
	)
}

impl_benchmark_test_suite! {Currency, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
