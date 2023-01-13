use sp_std::vec::Vec;

pub trait ServerInfo<T: frame_system::Config> {
	fn get_id(&self) -> u64;
	fn get_owner(&self) -> &T::AccountId;
	fn get_api_url(&self) -> &Vec<u8>;
}

pub trait ServerProvider<T: frame_system::Config> {
	type Error;
	type Server: ServerInfo<T> + sp_std::fmt::Debug;

	fn get_by_id(server_id: u64) -> Option<Self::Server>;
}

pub trait ServerInterface<T: frame_system::Config> {
	type Error;
	type Server;
	type Balance: Copy;
	type ActionType;

	fn register(
		owner: &T::AccountId,
		api_url: &[u8],
		stake_amount: Option<Self::Balance>,
	) -> Result<Self::Server, Self::Error>;

	fn update_server(
		server_id: u64,
		owner: &T::AccountId,
		action: &Self::ActionType,
	) -> Result<(), Self::Error>;

	fn unregister(server_id: u64, owner: &T::AccountId) -> Result<T::BlockNumber, Self::Error>;

	fn cancel_unregister(
		server_id: u64,
		owner: &T::AccountId,
	) -> Result<T::BlockNumber, Self::Error>;
}
