use super::*;

#[allow(unused)]
use crate::Pallet as Currency;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

const SEED: u32 = 0;

benchmarks! {
	add_currency {
		let caller: T::AccountId = whitelisted_caller();
	}: add_currency(
		RawOrigin::Signed(caller),
		String::from("MYRIA").into_bytes(),
		18,
		String::from("wss://rpc.myriad.systems").into_bytes(),
		true
	)

	update_balance {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recepient", 0, SEED);
	}: update_balance(
		RawOrigin::Signed(caller),
		recipient,
		String::from("MYRIA").into_bytes(),
		s.into()
	)

	transfer {
		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recepient", 0, SEED);
	}: transfer(
		RawOrigin::Signed(caller),
		recipient,
		String::from("MYRIA").into_bytes(),
		s.into()
	)
}

impl_benchmark_test_suite! {Currency, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
