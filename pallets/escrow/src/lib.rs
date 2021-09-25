#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
		sp_runtime::traits::AccountIdConversion, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	const PALLET_ID: PalletId = PalletId(*b"Charity!");

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
		/// Currency donate success. \[currency_id, donor, pot, amount, charity_balance\]
		DonationReceived(CurrencyId, T::AccountId, T::AccountId, Amount, CurrencyBalance),
		/// Fund allocate success. \[currency_id, to, amount, charity_balance\]
		FundsAllocated(CurrencyId, T::AccountId, Amount, CurrencyBalance),
	}

	#[pallet::error]
	pub enum Error<T> {
		InsufficientAmount,
		InsufficientFunds,
		OverflowAmount,
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

			let sender_balance = pallet_currency::Pallet::<T>::accounts(&donor, &currency_id)
				.unwrap_or(CurrencyBalance { free: 0 });
			let receiver_balance =
				pallet_currency::Pallet::<T>::accounts(Self::account_id(), &currency_id)
					.unwrap_or(CurrencyBalance { free: 0 });

			// Calculate new balances
			let updated_from_balance =
				sender_balance.free.checked_sub(amount).ok_or(Error::<T>::InsufficientFunds)?;
			let updated_to_balance =
				receiver_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			// Write new balances to storage
			pallet_currency::Accounts::<T>::insert(
				&donor,
				&currency_id,
				CurrencyBalance { free: updated_from_balance },
			);

			pallet_currency::Accounts::<T>::insert(
				Self::account_id(),
				&currency_id,
				CurrencyBalance { free: updated_to_balance },
			);

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

		/// Allocate the Charity's funds
		///
		/// Take funds from the Charity's pot and send them somewhere. This call requires root origin,
		/// which means it must come from a governance mechanism such as Substrate's Democracy pallet.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn allocate(
			origin: OriginFor<T>,
			dest: T::AccountId,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let currency = pallet_currency::Pallet::<T>::currency(currency_id.clone());

			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);

			// Make the transfer requested
			let pot_balance = Self::pot(&currency_id);
			let dest_balance = pallet_currency::Pallet::<T>::accounts(&dest, &currency_id)
				.unwrap_or(CurrencyBalance { free: 0 });

			ensure!(amount <= pot_balance.free, Error::<T>::InsufficientAmount);

			let updated_pot_balance = pot_balance.free.checked_sub(amount);
			let updated_dest_balance =
				dest_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			pallet_currency::Accounts::<T>::insert(
				&Self::account_id(),
				&currency_id,
				CurrencyBalance { free: updated_pot_balance.unwrap_or(0) },
			);

			pallet_currency::Accounts::<T>::insert(
				&dest,
				&currency_id,
				CurrencyBalance { free: updated_dest_balance },
			);

			Self::deposit_event(Event::FundsAllocated(
				currency_id.clone(),
				dest,
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

			// Calculate people and post balance
			let updated_people_balance = people_balance.checked_add(amount);
			let updated_post_balance = post_balance.checked_add(amount);

			// Store in storage
			PeopleBalance::<T>::insert(
				(&currency_id, &people_id, &platform),
				updated_people_balance.unwrap_or(0),
			);
			PostBalance::<T>::insert(
				(&currency_id, &post_id, &people_id, &platform),
				updated_post_balance.unwrap_or(0),
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
