
//! Autogenerated weights for pallet_platform
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-10-07, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// E:\project\myriad\myriad-node\target\release\myriad.exe
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet-platform
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./runtime/src/weights/pallet_platform.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_platform.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_platform::WeightInfo for WeightInfo<T> {
	fn add_platform(_s: u32, ) -> Weight {
		(42_891_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
