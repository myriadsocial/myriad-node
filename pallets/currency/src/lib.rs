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
pub use scale_info::TypeInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use sp_std::vec::Vec;

	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	pub type Amount = u128;
	pub type CurrencyId = Vec<u8>;

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct CurrencyInfo {
		pub decimal: u16,
		pub rpc_url: Vec<u8>,
		pub native: bool,
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct CurrencyBalance {
		pub free: u128,
	}

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
	#[pallet::getter(fn accounts)]
	pub type Accounts<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		CurrencyId,
		CurrencyBalance,
	>;

	#[pallet::storage]
	#[pallet::getter(fn currency)]
	pub type Currency<T: Config> = StorageMap<_, Blake2_128Concat, CurrencyId, CurrencyInfo>;

	#[pallet::storage]
	#[pallet::getter(fn currencies)]
	pub(super) type Currencies<T: Config> = StorageValue<_, Vec<Vec<u8>>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Currency add success. [currency_id, who]
		NewCurrencyAdded(CurrencyId, T::AccountId),
		/// Update balance success. [currency_id, amount, to, who]
		BalanceUpdated(CurrencyId, Amount, T::AccountId, T::AccountId),
		/// Currency transfer success. [currency_id, amount, to, who]
		Transferred(CurrencyId, Amount, T::AccountId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		CurrencyExist,
		CurrencyNotExist,
		BadOrigin,
		InsufficientFunds,
		OverflowAmount,
		InsufficientAmount,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_currency())]
		pub fn add_currency(
			origin: OriginFor<T>,
			currency_id: Vec<u8>,
			decimal: u16,
			rpc_url: Vec<u8>,
			native: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let currency = Self::currency(currency_id.clone());
			let currency_info = CurrencyInfo { decimal, rpc_url, native };

			ensure!(currency.is_none(), Error::<T>::CurrencyExist);

			let mut currencies = Self::currencies().unwrap_or_default();
			currencies.push(currency_id.clone());

			Currency::<T>::insert(&currency_id, &currency_info);
			Currencies::<T>::put(currencies);

			Self::deposit_event(Event::NewCurrencyAdded(currency_id, who));

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_balance(currency_id.len() as u32))]
		pub fn update_balance(
			origin: OriginFor<T>,
			to: T::AccountId,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let currency = Self::currency(currency_id.clone());

			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			let receiver_balance =
				Self::accounts(&to, &currency_id).unwrap_or(CurrencyBalance { free: 0 });
			let updated_to_balance =
				receiver_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			Accounts::<T>::insert(&to, &currency_id, CurrencyBalance { free: updated_to_balance });

			Self::deposit_event(Event::BalanceUpdated(currency_id, amount, to, who));

			Ok(())
		}

		/// Transfer tokens from one account to another
		#[pallet::weight(T::WeightInfo::transfer(currency_id.len() as u32 ))]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let currency = Self::currency(currency_id.clone());

			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(who != to, Error::<T>::BadOrigin);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			let sender_balance =
				Self::accounts(&who, &currency_id).unwrap_or(CurrencyBalance { free: 0 });
			let receiver_balance =
				Self::accounts(&to, &currency_id).unwrap_or(CurrencyBalance { free: 0 });

			// Calculate new balances
			let updated_from_balance =
				sender_balance.free.checked_sub(amount).ok_or(Error::<T>::InsufficientFunds)?;
			let updated_to_balance =
				receiver_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			// Write new balances to storage
			Accounts::<T>::insert(
				&who,
				&currency_id,
				CurrencyBalance { free: updated_from_balance },
			);
			Accounts::<T>::insert(&to, &currency_id, CurrencyBalance { free: updated_to_balance });

			Self::deposit_event(Event::Transferred(currency_id, amount, to, who));

			Ok(())
		}
	}
}
