use sp_std::vec::Vec;

pub trait ServerInfo<T: frame_system::Config> {
	fn get_id(&self) -> &Vec<u8>;
	fn get_owner(&self) -> &T::AccountId;
	fn get_name(&self) -> &Vec<u8>;
	fn get_api_url(&self) -> &Vec<u8>;
	fn get_web_url(&self) -> &Vec<u8>;
}

pub trait ServerProvider<T: frame_system::Config> {
	type Error;
	type Server: ServerInfo<T> + sp_std::fmt::Debug;

	fn get_by_id(server_id: &[u8]) -> Option<Self::Server>;
}

pub trait ServerInterface<T: frame_system::Config> {
	type Error;
	type Server;

	fn get_by_id(server_id: &[u8]) -> Option<Self::Server>;

	fn register(
		owner: &T::AccountId,
		name: &[u8],
		api_url: &[u8],
		web_url: &[u8],
	) -> Result<Self::Server, Self::Error>;

	fn transfer_owner(
		server_id: u64,
		owner: &T::AccountId,
		new_owner: &T::AccountId,
	) -> Result<(), Self::Error>;

	fn update_name(
		server_id: u64,
		owner: &T::AccountId,
		new_name: &[u8],
	) -> Result<(), Self::Error>;

	fn update_api_url(
		server_id: u64,
		owner: &T::AccountId,
		new_api_url: &[u8],
	) -> Result<(), Self::Error>;

	fn update_web_url(
		server_id: u64,
		owner: &T::AccountId,
		new_web_url: &[u8],
	) -> Result<(), Self::Error>;

	fn unregister(server_id: u64, owner: &T::AccountId) -> Result<(), Self::Error>;
}
