#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PlatformCreated(Vec<u8>),
		PlatformDeleted(Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		PlatformAlreadyExist,
		PlatformNotExist,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn platforms)]
	pub(super) type Platforms<T: Config> = StorageValue<_, Vec<Vec<u8>>>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn insert_platform(
			origin: OriginFor<T>,
			platform: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _creator = ensure_signed(origin)?;

			match Self::create_platform(&platform) {
				Ok(_) => {
					Self::deposit_event(Event::PlatformCreated(platform));

					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_platform(
			origin: OriginFor<T>,
			platform: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _destroyer = ensure_signed(origin)?;

			match Self::delete_platform(&platform) {
				Ok(_) => {
					Self::deposit_event(Event::PlatformDeleted(platform));

					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn create_platform(platform: &[u8]) -> Result<(), Error<T>> {
			let mut platforms: Vec<Vec<u8>> = Self::platforms().unwrap_or_default();

			let found_platform = platforms.iter().find(|e| *e == platform);

			if found_platform.is_none() {
				platforms.push(platform.into());
			} else {
				return Err(Error::<T>::PlatformAlreadyExist)
			}

			Platforms::<T>::put(platforms.clone());

			Ok(())
		}

		pub fn delete_platform(platform: &[u8]) -> Result<(), Error<T>> {
			let platforms: Vec<Vec<u8>> = Self::platforms().unwrap_or_default();

			let updated_platforms: Vec<Vec<u8>> =
				platforms.clone().into_iter().filter(|e| e != platform).collect();

			if updated_platforms.len() == platforms.len() {
				return Err(Error::<T>::PlatformNotExist)
			}

			Platforms::<T>::put(updated_platforms);

			Ok(())
		}
	}
}
