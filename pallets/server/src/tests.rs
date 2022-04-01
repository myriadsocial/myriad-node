use crate::{self as pallet_server, mock::*, AdminKey, Error};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{BlakeTwo256, Hash},
};

#[test]
fn register_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_eq!(
			Server::server_by_id(server_id),
			Some(pallet_server::Server {
				id: server_id,
				owner: 1,
				name: "myriad".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn transfer_owner_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_ok!(Server::transfer_owner(Origin::signed(2), 1, server_id, 3));

		assert_eq!(
			Server::server_by_id(server_id),
			Some(pallet_server::Server {
				id: server_id,
				owner: 3,
				name: "myriad".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn change_name_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_ok!(Server::update_name(
			Origin::signed(2),
			1,
			server_id,
			"local".as_bytes().to_vec()
		));

		assert_eq!(
			Server::server_by_id(server_id),
			Some(pallet_server::Server {
				id: server_id,
				owner: 1,
				name: "local".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn deregister_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_ok!(Server::unregister(Origin::signed(2), 1, server_id));

		assert_eq!(Server::server_by_id(server_id), None);
	})
}

#[test]
pub fn transfer_admin_key_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(1);

		assert_ok!(Server::transfer_admin_key(Origin::signed(1), 2));

		assert_eq!(Server::admin_key(), 2);
	})
}

#[test]
pub fn cant_transfer_owner_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_noop!(
			Server::transfer_owner(
				Origin::signed(2),
				1,
				BlakeTwo256::hash("server_id".as_bytes()),
				2
			),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_transfer_owner_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_noop!(
			Server::transfer_owner(Origin::signed(2), 3, server_id, 1),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_name_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_noop!(
			Server::update_name(
				Origin::signed(2),
				1,
				BlakeTwo256::hash("server_id".as_bytes()),
				"local".as_bytes().to_vec()
			),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_change_name_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_noop!(
			Server::update_name(Origin::signed(2), 3, server_id, "local".as_bytes().to_vec()),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_deregister_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_noop!(
			Server::unregister(Origin::signed(2), 1, BlakeTwo256::hash("server_id".as_bytes())),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_deregister_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_noop!(
			Server::unregister(Origin::signed(2), 3, server_id),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(Origin::signed(2), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		System::assert_last_event(Event::Server(crate::Event::Registered(pallet_server::Server {
			id: server_id,
			owner: 1,
			name: "myriad".as_bytes().to_vec(),
		})));

		assert_ok!(Server::transfer_owner(Origin::signed(2), 1, server_id, 3));

		System::assert_last_event(Event::Server(crate::Event::OwnerTransferred(3, server_id)));

		assert_ok!(Server::update_name(
			Origin::signed(2),
			3,
			server_id,
			"local".as_bytes().to_vec()
		));

		System::assert_last_event(Event::Server(crate::Event::NameUpdated(
			"local".as_bytes().to_vec(),
			server_id,
		)));

		assert_ok!(Server::unregister(Origin::signed(2), 3, server_id));

		System::assert_last_event(Event::Server(crate::Event::Unregistered(server_id)));
	})
}

#[test]
pub fn cant_transfer_admin_key_when_not_as_admin() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(1);

		assert_noop!(Server::transfer_admin_key(Origin::signed(3), 2), Error::<Test>::Unauthorized,);
	})
}
