use crate::*;

use frame_support::{pallet_prelude::*, sp_runtime::traits::Saturating, traits::Currency};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub type ServerId = Vec<u8>;
pub type FtIdentifier = Vec<u8>;
pub type ReferenceId = Vec<u8>;
pub type ReferenceType = Vec<u8>;

pub type TipsBalanceKey = (ServerId, ReferenceType, ReferenceId, FtIdentifier);
pub type TipsBalanceTuppleOf<T> = (TipsBalanceKey, BalanceOf<T>);

pub type AccountBalancesOf<T> = Vec<(FtIdentifier, AccountIdOf<T>, BalanceOf<T>)>;
pub type AccountBalancesTuppleOf<T> = (AccountBalancesOf<T>, Option<AccountBalancesOf<T>>);

pub type AssetId = u32;
pub type AssetBalance = u128;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CurrencyOf<T> = <T as self::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type TipsBalanceOf<T> = TipsBalance<BalanceOf<T>, AccountIdOf<T>>;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct TipsBalance<Balance, AccountId> {
	tips_balance_info: TipsBalanceInfo,
	account_id: Option<AccountId>,
	amount: Balance,
}
impl<Balance, AccountId> TipsBalance<Balance, AccountId>
where
	Balance: Clone + Saturating,
	AccountId: Clone,
{
	pub fn new(tips_balance_info: &TipsBalanceInfo, amount: &Balance) -> Self {
		Self {
			tips_balance_info: tips_balance_info.clone(),
			account_id: None,
			amount: amount.clone(),
		}
	}

	pub fn key(&self) -> TipsBalanceKey {
		self.tips_balance_info.key()
	}

	pub fn get_tips_balance_info(&self) -> &TipsBalanceInfo {
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

	pub fn get_ft_identifier(&self) -> &Vec<u8> {
		self.tips_balance_info.get_ft_identifier()
	}

	pub fn get_account_id(&self) -> &Option<AccountId> {
		&self.account_id
	}

	pub fn set_tips_balance_info(&mut self, tips_balance_info: &TipsBalanceInfo) {
		self.tips_balance_info = tips_balance_info.clone();
	}

	pub fn set_amount(&mut self, amount: Balance) {
		self.amount = amount;
	}

	pub fn add_amount(&mut self, amount: Balance) {
		self.amount = self.amount.clone().saturating_add(amount);
	}

	pub fn set_account_id(&mut self, account_id: &AccountId) {
		self.account_id = Some(account_id.clone());
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct TipsBalanceInfo {
	server_id: Vec<u8>,
	reference_type: Vec<u8>,
	reference_id: Vec<u8>,
	ft_identifier: Vec<u8>,
}
impl TipsBalanceInfo {
	pub fn new(
		server_id: &[u8],
		reference_type: &[u8],
		reference_id: &[u8],
		ft_identifier: &[u8],
	) -> Self {
		Self {
			server_id: server_id.to_vec(),
			reference_type: reference_type.to_vec(),
			reference_id: reference_id.to_vec(),
			ft_identifier: ft_identifier.to_vec(),
		}
	}

	pub fn key(&self) -> TipsBalanceKey {
		(
			self.server_id.clone(),
			self.reference_type.clone(),
			self.reference_id.clone(),
			self.ft_identifier.clone(),
		)
	}

	pub fn get_reference_id(&self) -> &Vec<u8> {
		&self.reference_id
	}

	pub fn get_reference_type(&self) -> &Vec<u8> {
		&self.reference_type
	}

	pub fn get_server_id(&self) -> &Vec<u8> {
		&self.server_id
	}

	pub fn get_ft_identifier(&self) -> &Vec<u8> {
		&self.ft_identifier
	}

	pub fn set_server_id(mut self, server_id: &[u8]) -> Self {
		self.server_id = server_id.to_vec();
		self
	}

	pub fn set_reference_id(&mut self, reference_id: &[u8]) {
		self.reference_id = reference_id.to_vec();
	}

	pub fn set_reference_type(&mut self, reference_type: &[u8]) {
		self.reference_type = reference_type.to_vec();
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct References {
	reference_type: ReferenceType,
	reference_ids: Vec<ReferenceId>,
}
impl References {
	pub fn new(reference_type: &[u8], reference_ids: &[Vec<u8>]) -> Self {
		Self { reference_type: reference_type.to_vec(), reference_ids: reference_ids.to_vec() }
	}

	pub fn get_reference_type(&self) -> &Vec<u8> {
		&self.reference_type
	}

	pub fn get_reference_ids(&self) -> &Vec<Vec<u8>> {
		&self.reference_ids
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum Status {
	OnProgress,
	Success,
	Failed,
}
impl Default for Status {
	fn default() -> Self {
		Status::OnProgress
	}
}
