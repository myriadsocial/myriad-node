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
	use sp_io::offchain_index;
	use sp_std::{str, vec::Vec};

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
		/// Connect account [status, Option<wallet>]
		ConnectingAccount(Status, Option<WalletInfo>),
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

		#[pallet::weight(0)]
		pub fn claim_reference_unsigned(
			origin: OriginFor<T>,
			_block_number: T::BlockNumber,
			api_response: APIResult<AccountIdOf<T>>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			let server_id = api_response.get_server_id();
			let ft_identifier = api_response.get_ft_identifier();
			let access_token = api_response.get_access_token();
			let account_id = api_response.get_account_id();
			let data_type = api_response.get_data_type().clone().unwrap_or_default();

			let reference_type: ReferenceType;
			let reference_id: ReferenceId;
			let tips_balance_info = match data_type.clone() {
				DataType::UserSocialMedia(user_social_media_info) => {
					reference_type = "user".as_bytes().to_vec();
					reference_id = user_social_media_info.get_user_id().to_vec();

					TipsBalanceInfo::new(
						server_id,
						"people".as_bytes(),
						user_social_media_info.get_people_id(),
						ft_identifier,
					)
				},
				DataType::Wallet(wallet_info) => {
					reference_type = "user".as_bytes().to_vec();
					reference_id = wallet_info.get_user_id().to_vec();

					TipsBalanceInfo::new(server_id, &reference_type, &reference_id, ft_identifier)
				},
			};

			let result: Result<(), Error<T>> = match <Self as TippingInterface<T>>::claim_reference(
				&None,
				&tips_balance_info,
				&reference_type,
				&reference_id,
				account_id,
				false,
			) {
				Ok(tips_balances) => {
					Self::deposit_event(Event::ClaimReference(tips_balances.0, tips_balances.1));
					Self::deposit_event(match data_type.clone() {
						DataType::UserSocialMedia(user_social_media_info) =>
							Event::<T>::VerifyingSocialMedia(
								Status::Success,
								Some(user_social_media_info),
							),
						DataType::Wallet(wallet_info) =>
							Event::<T>::ConnectingAccount(Status::Success, Some(wallet_info)),
					});

					Ok(())
				},
				Err(error) => Err(error),
			};

			if result.is_ok() {
				return Ok(().into())
			}

			let endpoint = match data_type {
				DataType::UserSocialMedia(user_social_media_info) => {
					Self::deposit_event(Event::<T>::VerifyingSocialMedia(Status::Failed, None));

					let mut endpoint = String::from("/user-social-medias/");
					let id = str::from_utf8(user_social_media_info.get_id()).unwrap_or("id");

					endpoint.push_str(id);

					endpoint
				},
				DataType::Wallet(wallet_info) => {
					Self::deposit_event(Event::<T>::ConnectingAccount(Status::Failed, None));

					let mut endpoint = String::from("/wallets/");
					let id = str::from_utf8(wallet_info.get_id()).unwrap_or("id");

					endpoint.push_str(id);

					endpoint
				},
			};

			match Self::get_api_url(server_id, &endpoint) {
				Ok(api_url) => {
					let payload =
						Payload::<AccountIdOf<T>>::init(server_id, &api_url, access_token);
					let key = Self::derived_key(<frame_system::Pallet<T>>::block_number());
					let data = IndexingData::init(b"remove_data_unsigned", payload);

					offchain_index::set(&key, &data.encode());

					Ok(().into())
				},
				Err(err) => Err(err.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::claim_reference())]
		pub fn verify_social_media(
			origin: OriginFor<T>,
			server_id: Vec<u8>,
			access_token: Vec<u8>,
			social_media_credential: SocialMediaCredential,
			ft_identifier: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as TippingInterface<T>>::verify_social_media(
				&who,
				&server_id,
				&access_token,
				&social_media_credential,
				&ft_identifier,
			) {
				Ok(()) => {
					Self::deposit_event(Event::VerifyingSocialMedia(Status::default(), None));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::claim_reference())]
		pub fn connect_account(
			origin: OriginFor<T>,
			server_id: Vec<u8>,
			access_token: Vec<u8>,
			user_credential: UserCredential,
			ft_identifier: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as TippingInterface<T>>::connect_account(
				&who,
				&server_id,
				&access_token,
				&user_credential,
				&ft_identifier,
			) {
				Ok(()) => {
					Self::deposit_event(Event::ConnectingAccount(Status::default(), None));
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
				Call::claim_reference_unsigned { block_number, api_response: _ } =>
					Self::validate_transaction_parameters(
						block_number,
						"pallet_tipping::claim_reference_unsigned",
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
