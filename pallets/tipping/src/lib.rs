#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use frame_support::traits::StorageVersion;
pub use pallet::*;
pub use pallet_server::interface::{ServerInfo, ServerProvider};
pub use scale_info::{prelude::string::*, TypeInfo};

pub mod crypto;
pub mod functions;
pub mod impl_tipping;
pub mod interface;
pub mod types;
pub mod weights;

pub use crate::interface::TippingInterface;
pub use types::{api_response::*, payload::*, tips_balance::*};
pub use weights::WeightInfo;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		sp_runtime::{
			transaction_validity::{InvalidTransaction, TransactionValidity},
			SaturatedConversion,
		},
		traits::Currency,
	};
	use frame_system::{
		offchain::{AppCrypto, CreateSignedTransaction},
		pallet_prelude::*,
	};
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type Call: From<Call<Self>>;
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
		/// Claim balance success. [tips_balance, Option<tips_balance>]
		ClaimReference(TipsBalanceOf<T>, Option<TipsBalanceOf<T>>),
		/// Verify social media [status, Option<user_social_media>]
		VerifyingSocialMedia(Status, Option<UserSocialMediaInfo>),
		/// Delete social media [status]
		DeletingSocialMedia(Status),
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
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let _ = Self::verify_social_media_and_send_unsigned(block_number);
		}
	}

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
				&Some(who),
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

		#[pallet::weight(T::WeightInfo::claim_reference())]
		pub fn claim_reference_unsigned(
			origin: OriginFor<T>,
			_block_number: T::BlockNumber,
			tips_balance_info: TipsBalanceInfo,
			reference_type: ReferenceType,
			reference_id: ReferenceId,
			account_id: Option<AccountIdOf<T>>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			match <Self as TippingInterface<T>>::claim_reference(
				&None,
				&tips_balance_info,
				&reference_type,
				&reference_id,
				&account_id,
				false,
			) {
				Ok(tips_balances) => {
					Self::deposit_event(Event::ClaimReference(tips_balances.0, tips_balances.1));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::claim_reference())]
		pub fn verify_social_media(
			origin: OriginFor<T>,
			server_id: Vec<u8>,
			access_token: Vec<u8>,
			username: Vec<u8>,
			platform: Vec<u8>,
			ft_identifier: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as TippingInterface<T>>::verify_social_media(
				&who,
				&server_id,
				&access_token,
				&username,
				&platform,
				&ft_identifier,
			) {
				Ok(()) => {
					Self::deposit_event(Event::VerifyingSocialMedia(Status::default(), None));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn remove_user_social_media_unsigned(
			origin: OriginFor<T>,
			_block_number: T::BlockNumber,
			server_id: Vec<u8>,
			access_token: Vec<u8>,
			user_social_media_id: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			match <Self as TippingInterface<T>>::remove_user_social_media_unsigned(
				&server_id,
				&access_token,
				&user_social_media_id,
			) {
				Ok(()) => {
					Self::deposit_event(Event::DeletingSocialMedia(Status::OnProgress));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn call_event_unsigned(
			origin: OriginFor<T>,
			_block_number: T::BlockNumber,
			event: Event<T>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			Self::deposit_event(event);

			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			match call {
				Call::claim_reference_unsigned {
					block_number,
					tips_balance_info: _,
					reference_type: _,
					reference_id: _,
					account_id: _,
				} => Self::validate_transaction_parameters(
					block_number,
					"pallet_tipping::claim_reference_unsigned",
				),
				Call::remove_user_social_media_unsigned {
					block_number,
					server_id: _,
					access_token: _,
					user_social_media_id: _,
				} => Self::validate_transaction_parameters(
					block_number,
					"pallet_tipping::remove_user_social_media_unsigned",
				),
				Call::call_event_unsigned { block_number, event: _ } =>
					Self::validate_transaction_parameters(
						block_number,
						"pallet_tipping::call_event_unsigned",
					),
				_ => InvalidTransaction::Call.into(),
			}
		}
	}
}
