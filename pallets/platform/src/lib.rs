#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use sp_std::vec::Vec;

	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	pub type Platform = Vec<u8>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn platforms)]
	pub(super) type Platforms<T: Config> = StorageValue<_, Vec<Vec<u8>>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Platform add success. [platform, who]
		PlatformAdded(Platform, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		PlatformExist,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_platform(platform.len() as u32))]
		pub fn add_platform(origin: OriginFor<T>, platform: Platform) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut platforms = Self::platforms().unwrap_or_default();
			let found = platforms.iter().find(|x| x == &&platform);
			ensure!(found.is_none(), Error::<T>::PlatformExist);
			platforms.push(platform.clone());

			Platforms::<T>::put(platforms);

			Self::deposit_event(Event::PlatformAdded(platform, who));

			Ok(())
		}
	}
}
