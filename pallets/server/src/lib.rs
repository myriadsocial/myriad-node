#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
pub use scale_info::TypeInfo;

pub mod interface;
pub mod weights;
pub use crate::interface::{ServerInfo, ServerInterface, ServerProvider};
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	use sp_std::vec::Vec;

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct Server<AccountId> {
		pub id: Vec<u8>,
		pub owner: AccountId,
		pub name: Vec<u8>,
	}
	impl<AccountId: Clone> Server<AccountId> {
		pub fn new(id: &[u8], owner: &AccountId, name: &[u8]) -> Self {
			Self { id: id.to_vec(), owner: owner.clone(), name: name.to_vec() }
		}

		pub fn get_id(&self) -> &Vec<u8> {
			&self.id
		}

		pub fn get_owner(&self) -> &AccountId {
			&self.owner
		}

		pub fn get_name(&self) -> &Vec<u8> {
			&self.name
		}

		pub fn set_owner(&mut self, account_id: &AccountId) {
			self.owner = account_id.clone();
		}

		pub fn set_name(&mut self, name: &[u8]) {
			self.name = name.to_vec();
		}
	}
	impl<T, AccountId: Clone> ServerInfo<T> for Server<AccountId>
	where
		T: frame_system::Config<AccountId = AccountId>,
	{
		fn get_id(&self) -> &Vec<u8> {
			self.get_id()
		}

		fn get_owner(&self) -> &AccountId {
			self.get_owner()
		}

		fn get_name(&self) -> &Vec<u8> {
			self.get_name()
		}
	}

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type ServerOf<T> = Server<AccountIdOf<T>>;
	pub type ServerId = Vec<u8>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin_key: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { admin_key: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			AdminKey::<T>::put(&self.admin_key);
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

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
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::register(name.len() as u32))]
		pub fn register(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			server_id: Vec<u8>,
			name: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == AdminKey::<T>::get(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::register(&server_id, &account_id, &name) {
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

			ensure!(admin == AdminKey::<T>::get(), Error::<T>::Unauthorized);

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

			ensure!(admin == AdminKey::<T>::get(), Error::<T>::Unauthorized);

			match <Self as ServerInterface<T>>::update_name(&server_id, &account_id, &new_name) {
				Ok(_) => {
					Self::deposit_event(Event::NameUpdated(new_name, server_id));
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

			ensure!(admin == AdminKey::<T>::get(), Error::<T>::Unauthorized);

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

			ensure!(admin == AdminKey::<T>::get(), Error::<T>::Unauthorized);

			AdminKey::<T>::put(account_id.clone());

			Self::deposit_event(Event::AdminKeyTransferred(admin, account_id));

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::force_transfer_admin_key())]
		pub fn force_transfer_admin_key(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;

			AdminKey::<T>::put(account_id.clone());

			Self::deposit_event(Event::SetAdminKey(account_id));

			Ok(().into())
		}
	}

	impl<T: Config> ServerInterface<T> for Pallet<T> {
		type Error = Error<T>;
		type Server = ServerOf<T>;

		fn get_by_id(server_id: &[u8]) -> Option<Self::Server> {
			Self::server_by_id(server_id)
		}

		fn register(
			server_id: &[u8],
			account_id: &T::AccountId,
			name: &[u8],
		) -> Result<Self::Server, Self::Error> {
			if ServerById::<T>::contains_key(server_id) {
				return Err(Error::<T>::AlreadyExists)
			}
			let server = Server::new(server_id, account_id, name);

			ServerById::<T>::insert(server_id, server.clone());

			Ok(server)
		}

		fn transfer_owner(
			server_id: &[u8],
			account_id: &T::AccountId,
			new_owner: &T::AccountId,
		) -> Result<(), Self::Error> {
			if !ServerById::<T>::contains_key(server_id) {
				return Err(Error::<T>::NotExists)
			}

			let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();
			let current_owner = server.get_owner();

			if current_owner != account_id {
				return Err(Error::<T>::Unauthorized)
			}

			if current_owner == new_owner {
				return Ok(())
			}

			server.set_owner(new_owner);

			ServerById::<T>::insert(server_id, server);

			Ok(())
		}

		fn update_name(
			server_id: &[u8],
			account_id: &T::AccountId,
			new_name: &[u8],
		) -> Result<(), Self::Error> {
			if !ServerById::<T>::contains_key(server_id) {
				return Err(Error::<T>::NotExists)
			}

			let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();

			if server.get_owner() != account_id {
				return Err(Error::<T>::Unauthorized)
			}

			server.set_name(new_name);

			ServerById::<T>::insert(server_id, server);

			Ok(())
		}

		fn unregister(server_id: &[u8], account_id: &T::AccountId) -> Result<(), Self::Error> {
			if !ServerById::<T>::contains_key(server_id) {
				return Err(Error::<T>::NotExists)
			}

			let server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();

			if server.get_owner() != account_id {
				return Err(Error::<T>::Unauthorized)
			}

			ServerById::<T>::remove(server_id);

			Ok(())
		}
	}

	impl<T: Config> ServerProvider<T> for Pallet<T>
	where
		ServerOf<T>: ServerInfo<T>,
	{
		type Error = Error<T>;
		type Server = ServerOf<T>;

		fn get_by_id(id: &[u8]) -> Option<ServerOf<T>> {
			<Self as ServerInterface<T>>::get_by_id(id)
		}
	}
}
