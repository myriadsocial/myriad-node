#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod benchmarking;

pub use pallet::*;
pub use scale_info::TypeInfo;

pub mod functions;
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
const STORAGE_VERSION: StorageVersion = StorageVersion::new(6);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, Get},
		Blake2_128Concat,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MinimumStakeAmount: Get<<Self::Currency as Currency<Self::AccountId>>::Balance>;

		#[pallet::constant]
		type ScheduledBlockTime: Get<Self::BlockNumber>;

		#[pallet::constant]
		type MaxScheduledPerBlock: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn server_count)]
	pub type ServerCount<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn server_index)]
	pub type ServerIndex<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn server_by_id)]
	pub(super) type ServerById<T: Config> = StorageMap<_, Blake2_128Concat, ServerId, ServerOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn server_by_api_url)]
	pub(super) type ServerByApiUrl<T: Config> = StorageMap<_, Blake2_128Concat, ApiUrl, ServerId>;

	#[pallet::storage]
	#[pallet::getter(fn server_by_owner)]
	pub(super) type ServerByOwner<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		ServerId,
		ServerOf<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn tasks)]
	pub(super) type Tasks<T: Config> =
		StorageMap<_, Blake2_128Concat, BlockNumberFor<T>, Vec<ServerId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Register server success. [server]
		Registered(ServerOf<T>),
		/// Server updated success. [account_id, server_id, action_type]
		ServerUpdated(T::AccountId, ServerId, ActionTypeOf<T>),
		/// Unregister server success. [server_id]
		Unregistered(ServerId),
		/// Staked success. [account_id, server_id, amount]
		Staked(T::AccountId, ServerId, BalanceOf<T>),
		/// Unstaked success. [account_id, server_id, amount]
		Unstaked(T::AccountId, ServerId, BalanceOf<T>),
		/// Unstaked scheduled success. { server_id, when, task }
		Scheduled { server_id: ServerId, when: BlockNumberFor<T>, task: Vec<u8>, status: Status },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyExists,
		NotExists,
		Unauthorized,
		Overflow,
		BadSignature,
		InsufficientBalance,
		FailedToSchedule,
		MinimumStakeLimitBalance,
		UnstakingLimitBalance,
		WaitingToUnstaked,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			let tasks = Tasks::<T>::take(n);
			Self::do_remove_servers(n, tasks)
		}

		fn on_runtime_upgrade() -> Weight {
			migrations::migrate::<T>()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::register(api_url.len() as u32))]
		pub fn register(
			origin: OriginFor<T>,
			api_url: Vec<u8>,
			stake_amount: Option<BalanceOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServerInterface<T>>::register(&who, &api_url, stake_amount) {
				Ok(server) => {
					Self::deposit_event(Event::Registered(server.clone()));
					Self::deposit_event(Event::Staked(
						who,
						server.get_id(),
						*server.get_stake_amount(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_server(*server_id as u32))]
		pub fn update_server(
			origin: OriginFor<T>,
			server_id: ServerId,
			action_type: ActionTypeOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServerInterface<T>>::update_server(server_id, &who, &action_type) {
				Ok(_) => {
					Self::deposit_event(Event::ServerUpdated(who, server_id, action_type));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::unregister())]
		pub fn unregister(origin: OriginFor<T>, server_id: ServerId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServerInterface<T>>::unregister(server_id, &who) {
				Ok(when) => {
					Self::deposit_event(Event::Scheduled {
						server_id,
						when,
						task: b"Unstaked".to_vec(),
						status: Status::InProgress,
					});
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::cancel_unregister())]
		pub fn cancel_unregister(
			origin: OriginFor<T>,
			server_id: ServerId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServerInterface<T>>::cancel_unregister(server_id, &who) {
				Ok(when) => {
					Self::deposit_event(Event::Scheduled {
						server_id,
						when,
						task: b"Unstaked".to_vec(),
						status: Status::Cancelled,
					});
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
