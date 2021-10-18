#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use sp_std::vec::Vec;

	use frame_support::{
		dispatch::DispatchResult, pallet_prelude::*, sp_runtime::traits::AccountIdConversion,
		PalletId,
	};
	use frame_system::pallet_prelude::*;

	pub type CurrencyBalance = pallet_currency::CurrencyBalance;
	pub type Amount = u128;
	pub type CurrencyId = pallet_currency::CurrencyId;
	pub type PeopleId = Vec<u8>;
	pub type PostId = Vec<u8>;
	pub type Platform = Vec<u8>;
	pub type TotalBalance = u128;

	const PALLET_ID: PalletId = PalletId(*b"Tipping!");

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct Post {
		pub post_id: Vec<u8>,
		pub people_id: Vec<u8>,
		pub platform: Vec<u8>,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: WeightInfo;
		type Platform: WeightInfo;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
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

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Currency tip success. [currency_id, amount, pot_balance, pot, who]
		TipReceived(CurrencyId, Amount, CurrencyBalance, T::AccountId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		PlatformNotExist,
		CurrencyNotExist,
		InsufficientAmount,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::send_tip(currency_id.len() as u32))]
		pub fn send_tip(
			origin: OriginFor<T>,
			post: Post,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let platforms = pallet_platform::Pallet::<T>::platforms().unwrap_or_default();
			let platform = platforms.iter().find(|x| x == &&post.platform);
			let currency = pallet_currency::Pallet::<T>::currency(currency_id.clone());

			ensure!(platform.is_some(), Error::<T>::PlatformNotExist);
			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			Self::do_update_balance(post, currency_id.clone(), amount);
			Self::deposit_event(Event::TipReceived(
				currency_id.clone(),
				amount,
				Self::pot(currency_id),
				Self::account_id(),
				who,
			));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID that holds tipping's funds
		pub fn account_id() -> T::AccountId {
			PALLET_ID.into_account()
		}

		/// The tipping's balance
		fn pot(currency_id: CurrencyId) -> CurrencyBalance {
			pallet_currency::Pallet::<T>::accounts(&Self::account_id(), currency_id)
				.unwrap_or(CurrencyBalance { free: 0 })
		}

		fn do_update_balance(post: Post, currency_id: CurrencyId, amount: u128) {
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
}
