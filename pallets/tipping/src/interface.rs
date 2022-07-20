pub trait TippingInterface<T: frame_system::Config> {
	type Error;
	type TipsBalanceInfo;
	type TipsBalanceKey;
	type Balance;
	type ServerId;
	type References;
	type ReferenceType;
	type ReferenceId;
	type FtIdentifier;
	type FtIdentifiers;
	type SendTipResult;
	type ClaimTipResult;
	type ClaimReferenceResult;

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
