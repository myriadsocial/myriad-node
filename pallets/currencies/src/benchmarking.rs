use super::*;

#[allow(unused)]
use crate::Pallet as Currency;
use frame_system::RawOrigin;
use frame_system::{account, benchmarking, impl_benchmark_test_suite, whitelisted_caller};

const SEED: u32 = 0;

benchmarks! {
	add_currency {
		let caller: T::AccountId = whitelisted_caller();
	}: add_currency(
		RawOrigin::Signed(caller),
		String::from("ACA").into_bytes(),
		12,
		String::from("wss://rpc.myriad.systems").into_bytes(),
		true
	),

	transfer {
		Currency::<T>::insert(String::from("ACA").into_bytes(), CurrencyInfo {
			decimal: 12,
			rpc_url: String::from("wss://rpc.myriad.systems"),
			native: true
		});
		Accounts::<T>::insert(&caller, String::from("ACA").into_bytes(), CurrencyBalance {free: 1200000});

		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recepient", 0, SEED);
	}: transfer(
		RawOrigin::Signed(caller),
		recepient,
		String::from("ACA").into_bytes(),
		s
	)

	update_balance {
		Currency::<T>::insert(String::from("ACA").into_bytes(), CurrencyInfo {
			decimal: 12,
			rpc_url: String::from("wss://rpc.myriad.systems"),
			native: true
		});

		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recepient", 0, SEED);
	}: update_balance(
		RawOrigin::Root(caller),
		recipient,
		String::from("ACA").into_bytes(),
		s
	)
}

impl_benchmark_test_suite!(Currency, crate::mock::ExternalityBuilder::build(), crate::mock::Test);
