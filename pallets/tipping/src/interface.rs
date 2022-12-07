pub trait TippingInterface<T: frame_system::Config> {
	type Error;
	type TipsBalanceInfo;
	type TipsBalanceKey;
	type Balance;
	type ServerId;
	type References;
	type FtIdentifier;
	type FtIdentifiers;
	type SendTipResult;
	type ClaimTipResult;
	type ClaimReferenceResult;
	type Receipt;
	type WithdrawalResult;

	fn pay_content(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::Receipt, Self::Error>;

	fn withdrawal_balance(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		ft_identifiers: &Self::FtIdentifiers,
	) -> Result<Self::WithdrawalResult, Self::Error>;

	fn send_tip(
		sender: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::SendTipResult, Self::Error>;

	fn claim_tip(
		receiver: &T::AccountId,
		tips_balance_key: &Self::TipsBalanceKey,
		ft_identifiers: &Self::FtIdentifiers,
	) -> Result<Self::ClaimTipResult, Self::Error>;

	fn claim_reference(
		receiver: &T::AccountId,
		server_id: &Self::ServerId,
		references: &Self::References,
		main_references: &Self::References,
		ft_identifiers: &Self::FtIdentifiers,
		account_id: &T::AccountId,
		tx_fee: &Self::Balance,
	) -> Result<Self::ClaimReferenceResult, Self::Error>;
}
