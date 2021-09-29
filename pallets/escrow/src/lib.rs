#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
		sp_runtime::traits::AccountIdConversion, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	const PALLET_ID: PalletId = PalletId(*b"Tipping!");

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_currency::Config + pallet_platform::Config
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	pub type CurrencyBalance = pallet_currency::CurrencyBalance;
	pub type Amount = u128;
	pub type CurrencyId = pallet_currency::CurrencyId;
	pub type PeopleId = Vec<u8>;
	pub type PostId = Vec<u8>;
	pub type Platform = Vec<u8>;
	pub type TotalBalance = u128;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Currency donate success. \[currency_id, pot, donor, amount, pot_balance\]
		DonationReceived(CurrencyId, T::AccountId, T::AccountId, Amount, CurrencyBalance),
	}

	#[pallet::error]
	pub enum Error<T> {
		InsufficientAmount,
		CurrencyNotExist,
		PlatformNotExist,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn people_balance)]
	pub(super) type PeopleBalance<T> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CurrencyId>,
			NMapKey<Blake2_128Concat, PeopleId>,
			NMapKey<Blake2_128Concat, Platform>,
		),
		TotalBalance,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn post_balance)]
	pub(super) type PostBalance<T> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CurrencyId>,
			NMapKey<Blake2_128Concat, PostId>,
			NMapKey<Blake2_128Concat, PeopleId>,
			NMapKey<Blake2_128Concat, Platform>,
		),
		TotalBalance,
		ValueQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Donate some funds to the charity
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn donate(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			post: Post,
			amount: u128,
		) -> DispatchResultWithPostInfo {
			let donor = ensure_signed(origin.clone())?;
			let currency = pallet_currency::Pallet::<T>::currency(currency_id.clone());
			let platforms = pallet_platform::Pallet::<T>::platforms().unwrap_or(Vec::new());
			let platform = platforms.iter().find(|x| x == &&post.platform);

			ensure!(platform.is_some(), Error::<T>::PlatformNotExist);
			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			Self::do_update_balance(post, amount, currency_id.clone());
			Self::deposit_event(Event::DonationReceived(
				currency_id.clone(),
				donor,
				Self::account_id(),
				amount,
				Self::pot(&currency_id),
			));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID that holds the Charity's funds
		pub fn account_id() -> T::AccountId {
			PALLET_ID.into_account()
		}

		/// The Charity's balance
		fn pot(currency_id: &CurrencyId) -> CurrencyBalance {
			return pallet_currency::Pallet::<T>::accounts(&Self::account_id(), currency_id)
				.unwrap_or(CurrencyBalance { free: 0 });
		}

		fn do_update_balance(post: Post, amount: u128, currency_id: CurrencyId) {
			let Post { post_id, people_id, platform } = post;

			// Get balances
			let people_balance = Self::people_balance((&currency_id, &people_id, &platform));
			let post_balance = Self::post_balance((&currency_id, &post_id, &people_id, &platform));
			let pot_balance =
				pallet_currency::Pallet::<T>::accounts(Self::account_id(), &currency_id)
					.unwrap_or(CurrencyBalance { free: 0 });

			// Calculate people and post balance
			let updated_people_balance = people_balance.checked_add(amount).unwrap_or(0);
			let updated_post_balance = post_balance.checked_add(amount).unwrap_or(0);
			let updated_pot_balance = pot_balance.free.checked_add(amount).unwrap_or(0);

			// Store in storage
			PeopleBalance::<T>::insert(
				(&currency_id, &people_id, &platform),
				updated_people_balance,
			);
			PostBalance::<T>::insert(
				(&currency_id, &post_id, &people_id, &platform),
				updated_post_balance,
			);

			pallet_currency::Accounts::<T>::insert(
				Self::account_id(),
				&currency_id,
				CurrencyBalance { free: updated_pot_balance },
			);
		}
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct Post {
		pub post_id: Vec<u8>,
		pub people_id: Vec<u8>,
		pub platform: Vec<u8>,
	}
}
