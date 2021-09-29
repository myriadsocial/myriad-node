use super::*;

#[allow(unused)]
use crate::Pallet as Escrow;
use frame_system::RawOrigin;
use frame_system::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};

benchmarks! {
	donate {
		pallet_platform::Platforms::<T>::put(String::from("twitter").into_bytes());
		pallet_currency::Currency::<T>::insert(String::from("ACA").into_bytes(), pallet_currency::CurrencyInfo {
			decimal: 12,
			rpc_url: String::from("wss://rpc.myriad.systems"),
			native: true
		});

		let post = Post {
			post_id: String::from("60efac8c565ab8004ed28bb3").into_bytes(),
			people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
			platform: String::from("twitter").into_bytes()
		};

		let s in 1 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: donate(RawOrigin::Signed(caller), String::from("ACA"), post, s)
}

impl_benchmark_test_suite!(Escrow, crate::mock::ExternalityBuilder::build(), crate::mock::Test);
