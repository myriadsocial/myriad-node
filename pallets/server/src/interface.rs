use sp_std::vec::Vec;

pub trait ServerInfo<T: frame_system::Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner(&self) -> &T::AccountId;
	fn get_name(&self) -> &Vec<u8>;
}

pub trait ServerProvider<T: frame_system::Config> {
	type Error;
	type Server: ServerInfo<T> + sp_std::fmt::Debug;

	fn get_by_id(server_id: &T::Hash) -> Option<Self::Server>;
}

pub trait ServerInterface<T: frame_system::Config> {
	type Error;
	type Server;
	type Name;

	fn get_by_id(server_id: &T::Hash) -> Option<Self::Server>;

	fn register(account_id: &T::AccountId, name: &Self::Name) -> Self::Server;

	fn transfer_owner(
		server_id: &T::Hash,
		account_id: &T::AccountId,
		new_owner: &T::AccountId,
	) -> Result<Self::Server, Self::Error>;

	fn update_name(
		server_id: &T::Hash,
		account_id: &T::AccountId,
		new_name: &Self::Name,
	) -> Result<Self::Server, Self::Error>;

	fn unregister(server_id: &T::Hash, account_id: &T::AccountId) -> Result<(), Self::Error>;
}
