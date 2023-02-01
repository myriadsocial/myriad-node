use sp_std::vec::Vec;

pub trait TippingInterface<T: frame_system::Config> {
	type Error;
	type TipsBalance;
	type TipsBalanceInfo;
	type TipsBalanceKey;
	type Balance;
	type References;
	type Receipt;
	type WithdrawalResult;

	fn pay_content(
		sender: &T::AccountId,
		instance_id: u64,
		receiver: &Option<T::AccountId>,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
		account_reference: &Option<Vec<u8>>,
	) -> Result<Self::Receipt, Self::Error>;

	fn withdraw_fee(
		sender: &T::AccountId,
		receiver: &T::AccountId,
	) -> Result<(Self::WithdrawalResult, Self::WithdrawalResult), Self::Error>;

	fn withdraw_reward(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		instance_id: u64,
	) -> Result<(Self::WithdrawalResult, Self::WithdrawalResult), Self::Error>;

	fn send_tip(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::TipsBalance, Self::Error>;

	fn claim_tip(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		tips_balance_key: &Self::TipsBalanceKey,
		ft_identifiers: &[Vec<u8>],
	) -> Result<(Self::WithdrawalResult, Self::WithdrawalResult), Self::Error>;

	fn claim_reference(
		receiver: &T::AccountId,
		server_id: &T::AccountId,
		references: &Self::References,
		main_references: &Self::References,
		ft_identifiers: &[Vec<u8>],
		account_id: &T::AccountId,
		tx_fee: &Self::Balance,
	) -> Result<Vec<Self::TipsBalance>, Self::Error>;
}
