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
		traits::{tokens::fungibles, Currency, Get},
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

		#[pallet::constant]
		type TransactionFee: Get<u8>;
		type AdminFee: Get<u8>;
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
	#[pallet::getter(fn withdrawal_balance)]
	pub(super) type WithdrawalBalance<T: Config> =
		StorageMap<_, Blake2_128Concat, FtIdentifier, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn reward_balance)]
	pub(super) type RewardBalance<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, ServerIdOf<T>>,
			NMapKey<Blake2_128Concat, u64>,
			NMapKey<Blake2_128Concat, FtIdentifier>,
		),
		BalanceOf<T>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Send tip success. { from, to, tips_balance }
		SendTip { from: T::AccountId, to: T::AccountId, tips_balance: TipsBalanceOf<T> },
		/// Claim tip success { from, to, success, failed }
		ClaimTip {
			from: T::AccountId,
			to: T::AccountId,
			success: Vec<(FtIdentifier, BalanceOf<T>)>,
			failed: Vec<(FtIdentifier, BalanceOf<T>)>,
		},
		/// Claim reference success. [Vec<tips_balance>]
		ClaimReference(Vec<TipsBalanceOf<T>>),
		/// Pay unlockable content success. { from, to, receipt }
		PayUnlockableContent { from: T::AccountId, to: Option<T::AccountId>, receipt: ReceiptOf<T> },
		/// Withdrawal succes { from, to, success, failed }
		Withdrawal {
			from: T::AccountId,
			to: T::AccountId,
			success: Vec<(FtIdentifier, BalanceOf<T>)>,
			failed: Vec<(FtIdentifier, BalanceOf<T>)>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InsufficientBalance,
		Unauthorized,
		ServerNotRegister,
		WrongFormat,
		NotExists,
		InsufficientFee,
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
			receiver: Option<AccountIdOf<T>>,
			instance_id: u64,
			tips_balance_info: TipsBalanceInfoOf<T>,
			amount: BalanceOf<T>,
			account_reference: Option<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let receipt = <Self as TippingInterface<T>>::pay_content(
				&sender,
				instance_id,
				&receiver,
				&tips_balance_info,
				&amount,
				&account_reference,
			)?;

			Self::deposit_event(Event::PayUnlockableContent {
				from: sender,
				to: receiver,
				receipt,
			});
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::withdraw_fee())]
		pub fn withdraw_fee(
			origin: OriginFor<T>,
			receiver: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let sender = Self::tipping_account_id();
			let result = <Self as TippingInterface<T>>::withdraw_fee(&sender, &receiver)?;

			let (success, failed) = result;

			Self::deposit_event(Event::Withdrawal { from: sender, to: receiver, success, failed });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::withdraw_reward())]
		pub fn withdraw_reward(
			origin: OriginFor<T>,
			instance_id: u64,
		) -> DispatchResultWithPostInfo {
			let sender = Self::tipping_account_id();
			let receiver = ensure_signed(origin)?;
			let result =
				<Self as TippingInterface<T>>::withdraw_reward(&sender, &receiver, instance_id)?;

			let (success, failed) = result;

			Self::deposit_event(Event::Withdrawal { from: sender, to: receiver, success, failed });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::send_tip())]
		pub fn send_tip(
			origin: OriginFor<T>,
			info: TipsBalanceInfoOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let receiver = Self::tipping_account_id();

			ensure!(info.get_reference_type() != b"unlockable_content", Error::<T>::Unauthorized);

			let data = <Self as TippingInterface<T>>::send_tip(&sender, &receiver, &info, &amount)?;

			Self::deposit_event(Event::SendTip { from: sender, to: receiver, tips_balance: data });
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
			let sender = Self::tipping_account_id();

			let mut ft_identifiers = ft_identifiers;

			ft_identifiers.sort_unstable();
			ft_identifiers.dedup();

			let tips_balance_key = (server_id, reference_type, reference_id, b"".to_vec());
			let (success, failed) = <Self as TippingInterface<T>>::claim_tip(
				&sender,
				&receiver,
				&tips_balance_key,
				&ft_identifiers,
			)?;

			Self::deposit_event(Event::ClaimTip { from: sender, to: receiver, success, failed });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::claim_reference())]
		pub fn claim_reference(
			origin: OriginFor<T>,
			server_id: ServerIdOf<T>,
			references: References,
			account_references: References,
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
				&account_references,
				&ft_identifiers,
				&account_id,
				&tx_fee,
			)?;

			Self::deposit_event(Event::ClaimReference(tips_balances));
			Ok(().into())
		}
	}
}
