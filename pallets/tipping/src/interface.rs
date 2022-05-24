pub trait TippingInterface<T: frame_system::Config> {
	type Error;
	type TipsBalanceInfo;
	type Balance;
	type ReferenceId;
	type ReferenceType;
	type TipsBalance;
	type TipsBalances;
	type FtIdentifier;

	fn send_tip(
		sender: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
		amount: &Self::Balance,
	) -> Result<Self::TipsBalance, Self::Error>;

	fn claim_tip(
		receiver: &T::AccountId,
		tips_balance_info: &Self::TipsBalanceInfo,
	) -> Result<(Self::Balance, Self::FtIdentifier), Self::Error>;

	fn claim_reference(
		sender: &Option<T::AccountId>,
		tips_balance_info: &Self::TipsBalanceInfo,
		reference_type: &Self::ReferenceType,
		reference_id: &Self::ReferenceId,
		account_id: &Option<T::AccountId>,
		verify_owner: bool,
	) -> Result<Self::TipsBalances, Self::Error>;

	fn verify_social_media(
		sender: &T::AccountId,
		server_id: &[u8],
		access_token: &[u8],
		username: &[u8],
		platform: &[u8],
		ft_identifier: &[u8],
	) -> Result<(), Self::Error>;

	fn remove_user_social_media_unsigned(
		server_id: &[u8],
		access_token: &[u8],
		user_social_media_id: &[u8],
	) -> Result<(), Self::Error>;
}
