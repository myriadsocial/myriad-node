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

	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, sp_runtime::traits::Hash,
	};
	use frame_system::pallet_prelude::*;

	use sp_std::vec::Vec;

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct Server<AccountId, Hash> {
		pub id: Hash,
		pub owner: AccountId,
		pub name: Vec<u8>,
	}
	impl<AccountId, Hash> Server<AccountId, Hash> {
		pub fn new(id: Hash, owner: AccountId, name: Vec<u8>) -> Self {
			Self { id, owner, name }
		}

		pub fn get_id(&self) -> &Hash {
			&self.id
		}

		pub fn get_owner(&self) -> &AccountId {
			&self.owner
		}

		pub fn get_name(&self) -> &Vec<u8> {
			&self.name
		}

		pub fn transfer_owner(&mut self, account_id: AccountId) {
			self.owner = account_id;
		}
	}
	impl<T, AccountId, Hash> ServerInfo<T> for Server<AccountId, Hash>
	where
		T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
	{
		fn get_id(&self) -> &Hash {
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
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type ServerOf<T> = Server<AccountIdOf<T>, HashOf<T>>;
	pub type ServerIdOf<T> = HashOf<T>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn server_by_id)]
	pub(super) type ServerById<T: Config> = StorageMap<_, Blake2_128Concat, HashOf<T>, ServerOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Register server success. [who, server]
		Registered(T::AccountId, ServerOf<T>),
		/// Name updated success. [who, name, server_id]
		NameUpdated(T::AccountId, Vec<u8>, T::Hash),
		/// Owner transferred success. [who, new_owner, server_id]
		OwnerTransferred(T::AccountId, T::AccountId, T::Hash),
		/// Unregister server success. [who, server_id]
		Unregistered(T::AccountId, ServerIdOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		NotExists,
		Unauthorized,
		OwnerNotChanged,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::register(name.len() as u32))]
		pub fn register(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let server = <Self as ServerInterface<T>>::register(&who, &name);

			Self::deposit_event(Event::Registered(who, server));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::transfer_owner())]
		pub fn transfer_owner(
			origin: OriginFor<T>,
			server_id: HashOf<T>,
			new_owner: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServerInterface<T>>::transfer_owner(&server_id, &who, &new_owner) {
				Ok(server) => {
					Self::deposit_event(Event::OwnerTransferred(who, server.owner, server.id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_name(new_name.len() as u32))]
		pub fn update_name(
			origin: OriginFor<T>,
			server_id: HashOf<T>,
			new_name: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServerInterface<T>>::update_name(&server_id, &who, &new_name) {
				Ok(server) => {
					Self::deposit_event(Event::NameUpdated(who, server.name, server.id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::unregister())]
		pub fn unregister(
			origin: OriginFor<T>,
			server_id: ServerIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServerInterface<T>>::unregister(&server_id, &who) {
				Ok(_) => {
					Self::deposit_event(Event::Unregistered(who, server_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn generate_server_id(owner: &T::AccountId, name: &[u8]) -> HashOf<T> {
			let mut seed = owner.encode();
			let account_info = frame_system::Pallet::<T>::account(owner);

			seed.append(&mut account_info.nonce.encode());
			seed.append(&mut name.encode());

			T::Hashing::hash(&seed)
		}
	}

	impl<T: Config> ServerInterface<T> for Pallet<T> {
		type Error = Error<T>;
		type Server = ServerOf<T>;
		type Name = Vec<u8>;

		fn get_by_id(server_id: &T::Hash) -> Option<Self::Server> {
			Self::server_by_id(server_id)
		}

		fn register(account_id: &T::AccountId, name: &Self::Name) -> Self::Server {
			let id = Self::generate_server_id(account_id, name);
			let server = Server::new(id, account_id.clone(), name.to_vec());

			ServerById::<T>::insert(id, server.clone());

			server
		}

		fn transfer_owner(
			server_id: &T::Hash,
			account_id: &T::AccountId,
			new_owner: &T::AccountId,
		) -> Result<Self::Server, Self::Error> {
			if !ServerById::<T>::contains_key(server_id) {
				return Err(Error::<T>::NotExists)
			}

			let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();
			let current_owner = server.get_owner().clone();

			if &current_owner != account_id {
				return Err(Error::<T>::Unauthorized)
			}

			if &current_owner == new_owner {
				return Ok(server)
			}

			server.transfer_owner(new_owner.clone());

			ServerById::<T>::insert(server_id, server.clone());

			Ok(server)
		}

		fn update_name(
			server_id: &T::Hash,
			account_id: &T::AccountId,
			new_name: &Self::Name,
		) -> Result<Self::Server, Self::Error> {
			if !ServerById::<T>::contains_key(server_id) {
				return Err(Error::<T>::NotExists)
			}

			let mut server = <Self as ServerInterface<T>>::get_by_id(server_id).unwrap();

			if server.get_owner() != account_id {
				return Err(Error::<T>::Unauthorized)
			}

			server.name = new_name.to_vec();

			ServerById::<T>::insert(server_id, server.clone());

			Ok(server)
		}

		fn unregister(server_id: &T::Hash, account_id: &T::AccountId) -> Result<(), Self::Error> {
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

		fn get_by_id(id: &T::Hash) -> Option<ServerOf<T>> {
			<Self as ServerInterface<T>>::get_by_id(id)
		}
	}
}
