#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
pub use scale_info::TypeInfo;

pub mod impl_server;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

pub use crate::interface::{ServerInfo, ServerInterface, ServerProvider};
pub use types::*;
pub use weights::WeightInfo;

use frame_support::traits::StorageVersion;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { admin_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref admin_key) = self.admin_key {
				AdminKey::<T>::put(admin_key);
			}
		}
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn server_by_id)]
	pub(super) type ServerById<T: Config> = StorageMap<_, Blake2_128Concat, ServerId, ServerOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Register server success. [server]
		Registered(ServerOf<T>),
		/// Name updated success. [name, server_id]
		NameUpdated(Vec<u8>, ServerId),
		/// Api url updated success. [api_url, server_id]
		ApiUrlUpdated(Vec<u8>, ServerId),
		/// Web url updated success. [web_url, server_id]
		WebUrlUpdated(Vec<u8>, ServerId),
		/// Owner transferred success. [new_owner, server_id]
		OwnerTransferred(T::AccountId, ServerId),
		/// Unregister server success. [server_id]
		Unregistered(ServerId),
		/// Transfer admin key [current_admin_key, new_admin_key]
		AdminKeyTransferred(T::AccountId, T::AccountId),
		/// Set admin key [new_admin_key]
		SetAdminKey(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyExists,
		NotExists,
		Unauthorized,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::migrate::<T>()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::register(name.len() as u32))]
		pub fn register(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			server_id: Vec<u8>,
			name: Vec<u8>,
			api_url: Vec<u8>,
			web_url: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == Self::admin_key().unwrap(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::register(
				&server_id,
				&account_id,
				&name,
				&api_url,
				&web_url,
			) {
				Ok(server) => {
					Self::deposit_event(Event::Registered(server));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::transfer_owner())]
		pub fn transfer_owner(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			server_id: Vec<u8>,
			new_owner: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == Self::admin_key().unwrap(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::transfer_owner(&server_id, &account_id, &new_owner)
			{
				Ok(_) => {
					Self::deposit_event(Event::OwnerTransferred(new_owner, server_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_name(new_name.len() as u32))]
		pub fn update_name(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			server_id: Vec<u8>,
			new_name: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == Self::admin_key().unwrap(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::update_name(&server_id, &account_id, &new_name) {
				Ok(_) => {
					Self::deposit_event(Event::NameUpdated(new_name, server_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_api_url(new_api_url.len() as u32))]
		pub fn update_api_url(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			server_id: Vec<u8>,
			new_api_url: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == Self::admin_key().unwrap(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::update_api_url(
				&server_id,
				&account_id,
				&new_api_url,
			) {
				Ok(_) => {
					Self::deposit_event(Event::ApiUrlUpdated(new_api_url, server_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_web_url(new_web_url.len() as u32))]
		pub fn update_web_url(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			server_id: Vec<u8>,
			new_web_url: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == Self::admin_key().unwrap(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::update_web_url(
				&server_id,
				&account_id,
				&new_web_url,
			) {
				Ok(_) => {
					Self::deposit_event(Event::WebUrlUpdated(new_web_url, server_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::unregister())]
		pub fn unregister(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			server_id: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == Self::admin_key().unwrap(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::unregister(&server_id, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::Unregistered(server_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::transfer_admin_key())]
		pub fn transfer_admin_key(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == Self::admin_key().unwrap(), Error::<T>::Unauthorized);

			AdminKey::<T>::put(account_id.clone());

			Self::deposit_event(Event::AdminKeyTransferred(admin, account_id));

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::force_transfer_admin_key())]
		pub fn force_transfer_admin_key(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			AdminKey::<T>::put(account_id.clone());

			Self::deposit_event(Event::SetAdminKey(account_id));

			Ok(().into())
		}
	}
}
