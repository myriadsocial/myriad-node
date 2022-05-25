pub trait TippingInterface<T: frame_system::Config> {
	type Error;
	type TipsBalanceInfo;
	type Balance;
	type TipsBalance;
	type TipsBalances;
	type UserCredential;
	type SocialMediaCredential;
	type ServerId;
	type ReferenceType;
	type ReferenceId;
	type FtIdentifier;
	type AccessToken;
	type DataId;
	type DataType;

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
		server_id: &Self::ServerId,
		access_token: &Self::AccessToken,
		social_media_credential: &Self::SocialMediaCredential,
		ft_identifier: &Self::FtIdentifier,
	) -> Result<(), Self::Error>;

	fn connect_account(
		sender: &T::AccountId,
		server_id: &Self::ServerId,
		access_token: &Self::AccessToken,
		user_credential: &Self::UserCredential,
		ft_identifier: &Self::FtIdentifier,
	) -> Result<(), Self::Error>;

	fn remove_data_unsigned(
		server_id: &Self::ServerId,
		access_token: &Self::AccessToken,
		data_id: &Self::DataId,
		data_type: &Self::DataType,
	) -> Result<(), Self::Error>;
}
