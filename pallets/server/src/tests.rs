use crate::{self as pallet_server, mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{BlakeTwo256, Hash},
};

#[test]
fn register_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

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
		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_ok!(Server::transfer_owner(Origin::signed(1), server_id, 2));

		assert_eq!(
			Server::server_by_id(server_id),
			Some(pallet_server::Server {
				id: server_id,
				owner: 2,
				name: "myriad".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn change_name_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_ok!(Server::update_name(Origin::signed(1), server_id, "local".as_bytes().to_vec()));

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
		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_ok!(Server::unregister(Origin::signed(1), server_id));

		assert_eq!(Server::server_by_id(server_id), None);
	})
}

#[test]
pub fn cant_transfer_owner_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Server::transfer_owner(Origin::signed(1), BlakeTwo256::hash("server_id".as_bytes()), 2),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_transfer_owner_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_noop!(
			Server::transfer_owner(Origin::signed(2), server_id, 1),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_name_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Server::update_name(
				Origin::signed(1),
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
		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_noop!(
			Server::update_name(Origin::signed(2), server_id, "local".as_bytes().to_vec()),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_deregister_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Server::unregister(Origin::signed(1), BlakeTwo256::hash("server_id".as_bytes())),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_deregister_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		assert_noop!(Server::unregister(Origin::signed(2), server_id), Error::<Test>::Unauthorized);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Server::register(Origin::signed(1), "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);

		System::assert_last_event(Event::Server(crate::Event::Registered(
			1,
			pallet_server::Server { id: server_id, owner: 1, name: "myriad".as_bytes().to_vec() },
		)));

		assert_ok!(Server::transfer_owner(Origin::signed(1), server_id, 2));

		System::assert_last_event(Event::Server(crate::Event::OwnerTransferred(1, 2, server_id)));

		assert_ok!(Server::update_name(Origin::signed(2), server_id, "local".as_bytes().to_vec()));

		System::assert_last_event(Event::Server(crate::Event::NameUpdated(
			2,
			"local".as_bytes().to_vec(),
			server_id,
		)));

		assert_ok!(Server::unregister(Origin::signed(2), server_id));

		System::assert_last_event(Event::Server(crate::Event::Unregistered(2, server_id)));
	})
}
