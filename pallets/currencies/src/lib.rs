#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	pub type Amount = u128;
	pub type CurrencyId = Vec<u8>;
	// pub type TokenId<T> = Currencies::Pallet<T>::get();

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Currency transfer success. \[currency_id, from, to, amount\]
		Transferred(CurrencyId, T::AccountId, T::AccountId, Amount),
		/// Update balance success. \[currency_id, who, amount\]
		BalanceUpdated(CurrencyId, T::AccountId, Amount),
		/// Token add success. \[currency_id\]
		NewCurrencyAdded(CurrencyId),
	}

	#[pallet::error]
	pub enum Error<T> {
		BadOrigin,
		InsufficientFunds,
		OverflowAmount,
		InsufficientAmount,
		CurrencyExist,
		CurrencyNotExist,
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

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_balance(
			origin: OriginFor<T>,
			to: T::AccountId,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let currency = Self::currency(currency_id.clone());

			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			let receiver_balance =
				Self::accounts(&to, &currency_id).unwrap_or(CurrencyBalance { free: 0 });
			let updated_to_balance =
				receiver_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			Accounts::<T>::insert(&to, &currency_id, CurrencyBalance { free: updated_to_balance });

			Self::deposit_event(Event::BalanceUpdated(currency_id, to, amount));

			Ok(().into())
		}
		/// Transfer tokens from one account to another
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let currency = Self::currency(currency_id.clone());

			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(from != to, Error::<T>::BadOrigin);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			let sender_balance =
				Self::accounts(&from, &currency_id).unwrap_or(CurrencyBalance { free: 0 });
			let receiver_balance =
				Self::accounts(&to, &currency_id).unwrap_or(CurrencyBalance { free: 0 });

			// Calculate new balances
			let updated_from_balance =
				sender_balance.free.checked_sub(amount).ok_or(Error::<T>::InsufficientFunds)?;
			let updated_to_balance =
				receiver_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			// Write new balances to storage
			Accounts::<T>::insert(
				&from,
				&currency_id,
				CurrencyBalance { free: updated_from_balance },
			);
			Accounts::<T>::insert(&to, &currency_id, CurrencyBalance { free: updated_to_balance });

			Self::deposit_event(Event::Transferred(currency_id, from, to, amount));

			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_currency(
			origin: OriginFor<T>,
			currency_id: Vec<u8>,
			name: Vec<u8>,
			decimal: u16,
			rpc_url: Vec<u8>,
			native: bool,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let currency = Self::currency(currency_id.clone());

			ensure!(currency.is_none(), Error::<T>::CurrencyExist);

			let new_currency = CurrencyInfo { name, decimal, rpc_url, native };
			let mut currencies = Self::currencies().unwrap_or(Vec::new());

			currencies.push(currency_id.clone());

			Currency::<T>::insert(&currency_id, &new_currency);
			Currencies::<T>::put(currencies);

			Self::deposit_event(Event::NewCurrencyAdded(currency_id));

			Ok(().into())
		}
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct CurrencyBalance {
		pub free: u128,
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct CurrencyInfo {
		pub name: Vec<u8>,
		pub decimal: u16,
		pub rpc_url: Vec<u8>,
		pub native: bool,
	}
}
