#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod benchmarking;

pub use pallet::*;
pub use scale_info::{prelude::string::*, TypeInfo};

pub mod functions;
pub mod impl_tipping;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

pub use crate::interface::TippingInterface;
pub use types::*;
pub use weights::WeightInfo;

pub use frame_support::traits::{StorageVersion, UnixTime};

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{tokens::fungibles, Currency},
		Blake2_128Concat,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeCall: From<Call<Self>>;
		type TimeProvider: UnixTime;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Assets: fungibles::Transfer<
			<Self as frame_system::Config>::AccountId,
			AssetId = AssetId,
			Balance = AssetBalance,
		>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn tips_balance_by_reference)]
	pub(super) type TipsBalanceByReference<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, ServerIdOf<T>>,
			NMapKey<Blake2_128Concat, ReferenceType>,
			NMapKey<Blake2_128Concat, ReferenceId>,
			NMapKey<Blake2_128Concat, FtIdentifier>,
		),
		TipsBalanceOf<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn receipts)]
	pub(super) type Receipts<T: Config> = StorageMap<_, Blake2_128Concat, HashOf<T>, ReceiptOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn withdrawal_balance)]
	pub(super) type WithdrawalBalance<T: Config> =
		StorageMap<_, Blake2_128Concat, FtIdentifier, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn reward_balance)]
	pub(super) type RewardBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		FtIdentifier,
		BalanceOf<T>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn receipt_ids)]
	pub(super) type ReceiptIds<T: Config> = StorageValue<_, Vec<HashOf<T>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Send tip success. [who, pot, (data, balance)]
		SendTip(T::AccountId, T::AccountId, (TipsBalanceKeyOf<T>, BalanceOf<T>)),
		/// Claim tip success [pot, (succeed, failed)]
		ClaimTip(T::AccountId, (AccountBalancesOf<T>, Option<AccountBalancesOf<T>>)),
		/// Claim reference success. [Vec<tips_balance>]
		ClaimReference(Vec<TipsBalanceOf<T>>),
		/// Pay unlockable content success. [who, receiver, (data, balance)]
		PayUnlockableContent(ReceiptOf<T>),
		/// Withdrawal succes [who, receiver, detail]
		Withdrawal(T::AccountId, T::AccountId, Vec<(FtIdentifier, BalanceOf<T>)>),
	}

	#[pallet::error]
	pub enum Error<T> {
		InsufficientBalance,
		Unauthorized,
		ServerNotRegister,
		WrongFormat,
		NotExists,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::migrate::<T>()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::pay_content())]
		pub fn pay_content(
			origin: OriginFor<T>,
			receiver: AccountIdOf<T>,
			tips_balance_info: TipsBalanceInfoOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let receipt = <Self as TippingInterface<T>>::pay_content(
				&sender,
				&receiver,
				&tips_balance_info,
				&amount,
			)?;

			Self::deposit_event(Event::PayUnlockableContent(receipt));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::withdraw_fee())]
		pub fn withdraw_fee(
			origin: OriginFor<T>,
			receiver: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let sender = Self::tipping_account_id();
			let data = <Self as TippingInterface<T>>::withdraw_fee(&sender, &receiver)?;

			Self::deposit_event(Event::Withdrawal(sender, receiver, data));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::withdraw_reward())]
		pub fn withdraw_reward(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = Self::tipping_account_id();
			let receiver = ensure_signed(origin)?;
			let data = <Self as TippingInterface<T>>::withdraw_reward(&sender, &receiver)?;

			Self::deposit_event(Event::Withdrawal(sender, receiver, data));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::send_tip())]
		pub fn send_tip(
			origin: OriginFor<T>,
			tips_balance_info: TipsBalanceInfoOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(
				tips_balance_info.get_reference_type() != b"unlockable_content",
				Error::<T>::Unauthorized
			);

			let (receiver, data) =
				<Self as TippingInterface<T>>::send_tip(&sender, &tips_balance_info, &amount)?;

			Self::deposit_event(Event::SendTip(sender, receiver, data));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::claim_tip())]
		pub fn claim_tip(
			origin: OriginFor<T>,
			server_id: ServerIdOf<T>,
			reference_type: ReferenceType,
			reference_id: ReferenceId,
			ft_identifiers: Vec<FtIdentifier>,
		) -> DispatchResultWithPostInfo {
			let receiver = ensure_signed(origin)?;
			let mut ft_identifiers = ft_identifiers;

			ft_identifiers.sort_unstable();
			ft_identifiers.dedup();

			let tips_balance_key = (server_id, reference_type, reference_id, b"".to_vec());
			let (sender, data) = <Self as TippingInterface<T>>::claim_tip(
				&receiver,
				&tips_balance_key,
				&ft_identifiers,
			)?;

			Self::deposit_event(Event::ClaimTip(sender, data));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::claim_reference())]
		pub fn claim_reference(
			origin: OriginFor<T>,
			server_id: ServerIdOf<T>,
			references: References,
			main_references: References,
			ft_identifiers: Vec<FtIdentifier>,
			account_id: AccountIdOf<T>,
			tx_fee: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(who == server_id, Error::<T>::Unauthorized);

			let mut ft_identifiers = ft_identifiers;

			ft_identifiers.sort_unstable();
			ft_identifiers.dedup();

			let tips_balances = <Self as TippingInterface<T>>::claim_reference(
				&who,
				&server_id,
				&references,
				&main_references,
				&ft_identifiers,
				&account_id,
				&tx_fee,
			)?;

			Self::deposit_event(Event::ClaimReference(tips_balances));
			Ok(().into())
		}
	}
}
