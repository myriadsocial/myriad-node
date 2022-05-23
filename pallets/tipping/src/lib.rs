#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use frame_support::traits::StorageVersion;
pub use pallet::*;
pub use pallet_server::interface::{ServerInfo, ServerProvider};
pub use scale_info::{prelude::string::*, TypeInfo};

pub mod function;
pub mod impl_tipping;
pub mod interface;
pub mod types;
pub mod weights;

pub use crate::interface::TippingInterface;
pub use types::*;
pub use weights::WeightInfo;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, sp_runtime::SaturatedConversion,
		traits::Currency,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Server: ServerProvider<Self>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn tips_balance_by_reference)]
	pub(super) type TipsBalanceByReference<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, ServerId>,
			NMapKey<Blake2_128Concat, ReferenceType>,
			NMapKey<Blake2_128Concat, ReferenceId>,
			NMapKey<Blake2_128Concat, FtIdentifier>,
		),
		TipsBalanceOf<T>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Send tip success. [who, pot, tips_balance]
		SendTip(T::AccountId, T::AccountId, TipsBalanceOf<T>),
		/// Claim tip success. [pot, who, amount, ft_identifier]
		ClaimTip(T::AccountId, T::AccountId, BalanceOf<T>, FtIdentifier),
		/// Claim balance success. [tips_balance, tips_balance]
		ClaimReference(TipsBalanceOf<T>, Option<TipsBalanceOf<T>>),
	}

	#[pallet::error]
	pub enum Error<T> {
		InsufficientBalance,
		BadSignature,
		NothingToClaimed,
		Unauthorized,
		ServerNotRegister,
		ReceiverNotExists,
		FtNotExists,
		FtMustEmpty,
		NotExists,
		WrongFormat,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::send_tip((*amount).saturated_into::<u32>()))]
		pub fn send_tip(
			origin: OriginFor<T>,
			tips_balance_info: TipsBalanceInfo,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let receiver = Self::tipping_account_id();

			match <Self as TippingInterface<T>>::send_tip(&sender, &tips_balance_info, &amount) {
				Ok(tips_balance) => {
					Self::deposit_event(Event::SendTip(sender, receiver, tips_balance));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::claim_tip())]
		pub fn claim_tip(
			origin: OriginFor<T>,
			tips_balance_info: TipsBalanceInfo,
		) -> DispatchResultWithPostInfo {
			let sender = Self::tipping_account_id();
			let receiver = ensure_signed(origin)?;

			match <Self as TippingInterface<T>>::claim_tip(&receiver, &tips_balance_info) {
				Ok(result) => {
					Self::deposit_event(Event::ClaimTip(sender, receiver, result.0, result.1));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::claim_reference())]
		pub fn claim_reference(
			origin: OriginFor<T>,
			tips_balance_info: TipsBalanceInfo,
			reference_type: ReferenceType,
			reference_id: ReferenceId,
			account_id: Option<AccountIdOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as TippingInterface<T>>::claim_reference(
				&who,
				&tips_balance_info,
				&reference_type,
				&reference_id,
				&account_id,
				true,
			) {
				Ok(tips_balances) => {
					Self::deposit_event(Event::ClaimReference(tips_balances.0, tips_balances.1));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
