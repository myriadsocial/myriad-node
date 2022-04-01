#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
pub use pallet_server::interface::{ServerInfo, ServerProvider};
pub use scale_info::{prelude::string::*, TypeInfo};

pub mod interface;
pub mod weights;
pub use crate::interface::TippingInterface;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use sp_std::vec::Vec;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		sp_runtime::{
			traits::{AccountIdConversion, Zero},
			SaturatedConversion,
		},
		traits::{Currency, ExistenceRequirement, WithdrawReasons},
		PalletId,
	};
	use frame_system::pallet_prelude::*;

	const PALLET_ID: PalletId = PalletId(*b"Tipping!");

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct TipsBalance<Hash, Balance, AccountId> {
		pub tips_balance_info: TipsBalanceInfo<Hash>,
		pub account_id: Option<AccountId>,
		pub amount: Balance,
	}
	impl<Hash: Clone, Balance: Clone, AccountId: Clone> TipsBalance<Hash, Balance, AccountId> {
		pub fn new(
			tips_balance_info: &TipsBalanceInfo<Hash>,
			account_id: &Option<AccountId>,
			amount: &Balance,
		) -> Self {
			Self {
				tips_balance_info: tips_balance_info.clone(),
				account_id: account_id.clone(),
				amount: amount.clone(),
			}
		}

		pub fn get_tips_balance_info(&self) -> &TipsBalanceInfo<Hash> {
			&self.tips_balance_info
		}

		pub fn get_amount(&self) -> &Balance {
			&self.amount
		}

		pub fn get_reference_id(&self) -> &Vec<u8> {
			self.tips_balance_info.get_reference_id()
		}

		pub fn get_reference_type(&self) -> &Vec<u8> {
			self.tips_balance_info.get_reference_type()
		}

		pub fn get_server_id(&self) -> &Hash {
			self.tips_balance_info.get_server_id()
		}

		pub fn get_ft_identifier(&self) -> &Vec<u8> {
			self.tips_balance_info.get_ft_identifier()
		}

		pub fn get_account_id(&self) -> &Option<AccountId> {
			&self.account_id
		}

		pub fn set_tips_balance_info(&mut self, tips_balance_info: &TipsBalanceInfo<Hash>) {
			self.tips_balance_info = tips_balance_info.clone();
		}

		pub fn set_amount(&mut self, amount: Balance) {
			self.amount = amount;
		}

		pub fn set_account_id(&mut self, account_id: &Option<AccountId>) {
			self.account_id = account_id.clone();
		}
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct TipsBalanceInfo<Hash> {
		pub server_id: Hash,
		pub reference_type: Vec<u8>,
		pub reference_id: Vec<u8>,
		pub ft_identifier: Vec<u8>,
	}
	impl<Hash: Clone> TipsBalanceInfo<Hash> {
		pub fn new(
			server_id: &Hash,
			reference_type: &[u8],
			reference_id: &[u8],
			ft_identifier: &[u8],
		) -> Self {
			Self {
				server_id: server_id.clone(),
				reference_type: reference_type.to_vec(),
				reference_id: reference_id.to_vec(),
				ft_identifier: ft_identifier.to_vec(),
			}
		}

		pub fn get_reference_id(&self) -> &Vec<u8> {
			&self.reference_id
		}

		pub fn get_reference_type(&self) -> &Vec<u8> {
			&self.reference_type
		}

		pub fn get_server_id(&self) -> &Hash {
			&self.server_id
		}

		pub fn get_ft_identifier(&self) -> &Vec<u8> {
			&self.ft_identifier
		}

		pub fn set_reference_id(&mut self, reference_id: &[u8]) {
			self.reference_id = reference_id.to_vec();
		}

		pub fn set_reference_type(&mut self, reference_type: &[u8]) {
			self.reference_type = reference_type.to_vec();
		}
	}

	pub type FtIdentifier = Vec<u8>;
	pub type ReferenceId = Vec<u8>;
	pub type ReferenceType = Vec<u8>;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type TipsBalanceOf<T> = TipsBalance<HashOf<T>, BalanceOf<T>, AccountIdOf<T>>;
	pub type TipsBalanceInfoOf<T> = TipsBalanceInfo<HashOf<T>>;
	pub type ServerIdOf<T> = HashOf<T>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Server: ServerProvider<Self>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
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
			tips_balance_info: TipsBalanceInfoOf<T>,
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
			tips_balance_info: TipsBalanceInfoOf<T>,
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
			tips_balance_info: TipsBalanceInfoOf<T>,
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
			) {
				Ok(tips_balances) => {
					Self::deposit_event(Event::ClaimReference(tips_balances.0, tips_balances.1));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}

	impl<T: Config> TippingInterface<T> for Pallet<T> {
		type Error = Error<T>;
		type TipsBalance = TipsBalanceOf<T>;
		type TipsBalances = (TipsBalanceOf<T>, Option<TipsBalanceOf<T>>);
		type TipsBalanceInfo = TipsBalanceInfoOf<T>;
		type Balance = BalanceOf<T>;
		type ReferenceId = ReferenceId;
		type ReferenceType = ReferenceType;
		type FtIdentifier = FtIdentifier;

		fn send_tip(
			sender: &T::AccountId,
			tips_balance_info: &Self::TipsBalanceInfo,
			amount: &Self::Balance,
		) -> Result<Self::TipsBalance, Self::Error> {
			let server_id = tips_balance_info.get_server_id();
			let ft_identifier = tips_balance_info.get_ft_identifier();
			let tip_amount = *amount;

			if T::Server::get_by_id(server_id).is_none() {
				return Err(Error::<T>::ServerNotRegister)
			}

			if !Self::is_integer(ft_identifier) {
				return Err(Error::<T>::WrongFormat)
			}

			if ft_identifier != "native".as_bytes() {
				return Err(Error::<T>::FtNotExists)
			}

			if CurrencyOf::<T>::free_balance(sender) < tip_amount {
				return Err(Error::<T>::InsufficientBalance)
			}

			match CurrencyOf::<T>::withdraw(
				sender,
				tip_amount,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::KeepAlive,
			) {
				Ok(imb) => {
					let tips_balance = match Self::get_tips_balance(tips_balance_info) {
						Some(mut result) => {
							let total_amount = *result.get_amount() + tip_amount;

							result.set_amount(total_amount);
							Self::update_tips_balance(&result)
						},
						None =>
							Self::create_tips_balance(tips_balance_info, &None, &Some(tip_amount)),
					};
					let receiver = Self::tipping_account_id();

					CurrencyOf::<T>::resolve_creating(&receiver, imb);

					Ok(tips_balance)
				},
				_ => Err(Error::<T>::BadSignature),
			}
		}

		fn claim_tip(
			receiver: &T::AccountId,
			tips_balance_info: &Self::TipsBalanceInfo,
		) -> Result<(Self::Balance, Self::FtIdentifier), Self::Error> {
			let sender = Self::tipping_account_id();
			let tips_balance = Self::get_tips_balance(tips_balance_info);

			if tips_balance.is_none() {
				return Err(Error::<T>::NotExists)
			}

			let mut tips_balance = tips_balance.unwrap();
			let ft_identifier = tips_balance.get_ft_identifier().clone();
			let account_id = tips_balance.get_account_id().as_ref();
			let amount = *tips_balance.get_amount();

			if amount == Zero::zero() {
				return Err(Error::<T>::NothingToClaimed)
			}

			if account_id.is_none() {
				return Err(Error::<T>::ReceiverNotExists)
			}

			if account_id.unwrap() != receiver {
				return Err(Error::<T>::Unauthorized)
			}

			if !Self::is_integer(&ft_identifier) {
				return Err(Error::<T>::WrongFormat)
			}

			if ft_identifier != "native".as_bytes() {
				return Err(Error::<T>::FtNotExists)
			}

			tips_balance.set_amount(Zero::zero());

			match CurrencyOf::<T>::withdraw(
				&sender,
				amount,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::KeepAlive,
			) {
				Ok(imb) => {
					CurrencyOf::<T>::resolve_creating(receiver, imb);

					let _ = Self::update_tips_balance(&tips_balance);

					Ok((amount, ft_identifier))
				},
				_ => Err(Error::<T>::BadSignature),
			}
		}

		fn claim_reference(
			sender: &T::AccountId,
			tips_balance_info: &Self::TipsBalanceInfo,
			reference_type: &Self::ReferenceType,
			reference_id: &Self::ReferenceId,
			account_id: &Option<T::AccountId>,
		) -> Result<Self::TipsBalances, Self::Error> {
			let server_id = tips_balance_info.get_server_id();
			let server = T::Server::get_by_id(server_id);
			let ft_identifier = tips_balance_info.get_ft_identifier();

			let mut tips_balances = Self::default_tips_balances(tips_balance_info);

			if server.is_none() {
				return Err(Error::<T>::ServerNotRegister)
			}

			if server.unwrap().get_owner() != sender {
				return Err(Error::<T>::Unauthorized)
			}

			if ft_identifier != "native".as_bytes() {
				return Err(Error::<T>::FtNotExists)
			}

			if tips_balance_info.get_reference_type() == reference_type {
				if account_id.is_none() {
					return Err(Error::<T>::ReceiverNotExists)
				}

				if tips_balance_info.get_reference_id() != reference_id {
					return Err(Error::<T>::NotExists)
				}

				tips_balances.0 = match Self::get_tips_balance(tips_balance_info) {
					Some(mut result) => {
						result.set_account_id(account_id);
						Self::update_tips_balance(&result)
					},
					None => Self::create_tips_balance(tips_balance_info, account_id, &None),
				};
			} else {
				// Reference from tips balance info
				let mut initial_balance: BalanceOf<T> = Zero::zero();

				tips_balances.1 = match Self::get_tips_balance(tips_balance_info) {
					Some(mut result) => {
						initial_balance += *result.get_amount();

						if !initial_balance.is_zero() {
							result.set_amount(Zero::zero());
							Some(Self::update_tips_balance(&result))
						} else {
							Some(result)
						}
					},
					None => Some(Self::create_tips_balance(tips_balance_info, &None, &None)),
				};

				// Create or update reference from param
				let mut tips_balance_info = tips_balance_info.clone();

				tips_balance_info.set_reference_type(reference_type);
				tips_balance_info.set_reference_id(reference_id);

				tips_balances.0 = match Self::get_tips_balance(&tips_balance_info) {
					Some(mut result) => {
						let total_amount = *result.get_amount() + initial_balance;

						result.set_amount(total_amount);

						if account_id.is_some() {
							result.set_account_id(account_id);
						}

						Self::update_tips_balance(&result)
					},
					None => Self::create_tips_balance(
						&tips_balance_info,
						account_id,
						&Some(initial_balance),
					),
				};
			}

			Ok(tips_balances)
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID that holds tipping's funds
		pub fn tipping_account_id() -> T::AccountId {
			PALLET_ID.into_account()
		}

		fn get_tips_balance(tips_balance_info: &TipsBalanceInfoOf<T>) -> Option<TipsBalanceOf<T>> {
			let reference_type = tips_balance_info.get_reference_type();
			let reference_id = tips_balance_info.get_reference_id();
			let server_id = tips_balance_info.get_server_id();
			let ft_identifier = tips_balance_info.get_ft_identifier();

			Self::tips_balance_by_reference((
				server_id,
				reference_type,
				reference_id,
				ft_identifier,
			))
		}

		fn create_tips_balance(
			tips_balance_info: &TipsBalanceInfoOf<T>,
			account_id: &Option<AccountIdOf<T>>,
			amount: &Option<BalanceOf<T>>,
		) -> TipsBalanceOf<T> {
			let server_id = tips_balance_info.get_server_id();
			let reference_type = tips_balance_info.get_reference_type();
			let reference_id = tips_balance_info.get_reference_id();
			let ft_identifier = tips_balance_info.get_ft_identifier();
			let amount = if amount.is_some() { amount.unwrap() } else { Zero::zero() };
			let tips_balance = TipsBalance::new(tips_balance_info, account_id, &amount);

			TipsBalanceByReference::<T>::insert(
				(server_id, reference_type, reference_id, ft_identifier),
				tips_balance.clone(),
			);

			tips_balance
		}

		fn update_tips_balance(tips_balance: &TipsBalanceOf<T>) -> TipsBalanceOf<T> {
			let tips_balance_info = tips_balance.get_tips_balance_info();
			let server_id = tips_balance_info.get_server_id();
			let reference_type = tips_balance_info.get_reference_type();
			let reference_id = tips_balance_info.get_reference_id();
			let ft_identifier = tips_balance_info.get_ft_identifier();

			TipsBalanceByReference::<T>::insert(
				(server_id, reference_type, reference_id, ft_identifier),
				tips_balance.clone(),
			);

			tips_balance.clone()
		}

		fn default_tips_balances(
			tips_balance_info: &TipsBalanceInfoOf<T>,
		) -> (TipsBalanceOf<T>, Option<TipsBalanceOf<T>>) {
			(TipsBalance::new(tips_balance_info, &None, &Zero::zero()), None)
		}

		fn is_integer(ft_identifier: &[u8]) -> bool {
			if ft_identifier == "native".as_bytes() {
				return true
			};

			let str_num = match String::from_utf8(ft_identifier.to_vec()) {
				Ok(res) => res,
				Err(err) => err.to_string(),
			};

			str_num.parse::<u16>().is_ok()
		}
	}
}
