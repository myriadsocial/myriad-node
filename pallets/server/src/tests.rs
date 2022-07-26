use crate::{self as pallet_server, mock::*, AdminKey, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn register_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_eq!(
			Server::server_by_id("server_id".as_bytes().to_vec()),
			Some(pallet_server::Server {
				id: "server_id".as_bytes().to_vec(),
				owner: 1,
				name: "myriad".as_bytes().to_vec(),
				api_url: "https://api.dev.myriad.social".as_bytes().to_vec(),
				web_url: "https://app.dev.myriad.social".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn transfer_owner_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_ok!(Server::transfer_owner(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			3
		));

		assert_eq!(
			Server::server_by_id("server_id".as_bytes().to_vec()),
			Some(pallet_server::Server {
				id: "server_id".as_bytes().to_vec(),
				owner: 3,
				name: "myriad".as_bytes().to_vec(),
				api_url: "https://api.dev.myriad.social".as_bytes().to_vec(),
				web_url: "https://app.dev.myriad.social".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn change_name_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_ok!(Server::update_name(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"local".as_bytes().to_vec()
		));

		assert_eq!(
			Server::server_by_id("server_id".as_bytes().to_vec()),
			Some(pallet_server::Server {
				id: "server_id".as_bytes().to_vec(),
				owner: 1,
				name: "local".as_bytes().to_vec(),
				api_url: "https://api.dev.myriad.social".as_bytes().to_vec(),
				web_url: "https://app.dev.myriad.social".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn change_api_url_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_ok!(Server::update_api_url(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"https://api.testnet.myriad.social".as_bytes().to_vec(),
		));

		assert_eq!(
			Server::server_by_id("server_id".as_bytes().to_vec()),
			Some(pallet_server::Server {
				id: "server_id".as_bytes().to_vec(),
				owner: 1,
				name: "myriad".as_bytes().to_vec(),
				api_url: "https://api.testnet.myriad.social".as_bytes().to_vec(),
				web_url: "https://app.dev.myriad.social".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn change_web_url_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_ok!(Server::update_web_url(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"https://app.testnet.myriad.social".as_bytes().to_vec(),
		));

		assert_eq!(
			Server::server_by_id("server_id".as_bytes().to_vec()),
			Some(pallet_server::Server {
				id: "server_id".as_bytes().to_vec(),
				owner: 1,
				name: "myriad".as_bytes().to_vec(),
				api_url: "https://api.dev.myriad.social".as_bytes().to_vec(),
				web_url: "https://app.testnet.myriad.social".as_bytes().to_vec(),
			})
		);
	})
}

#[test]
pub fn deregister_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_ok!(Server::unregister(Origin::signed(2), 1, "server_id".as_bytes().to_vec()));

		assert_eq!(Server::server_by_id("server_id".as_bytes().to_vec()), None);
	})
}

#[test]
pub fn transfer_admin_key_works() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(1);

		assert_ok!(Server::transfer_admin_key(Origin::signed(1), 2));

		assert_eq!(Server::admin_key().unwrap(), 2);
	})
}

#[test]
fn cant_register_when_server_id_already_exists() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Server::register(
				Origin::signed(2),
				1,
				"server_id".as_bytes().to_vec(),
				"myriad".as_bytes().to_vec(),
				"https://api.dev.myriad.social".as_bytes().to_vec(),
				"https://app.dev.myriad.social".as_bytes().to_vec(),
			),
			Error::<Test>::AlreadyExists
		);
	})
}

#[test]
pub fn cant_transfer_owner_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_noop!(
			Server::transfer_owner(Origin::signed(2), 1, "server_id".as_bytes().to_vec(), 2),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_transfer_owner_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Server::transfer_owner(Origin::signed(2), 3, "server_id".as_bytes().to_vec(), 1),
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
				"server_id".as_bytes().to_vec(),
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

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Server::update_name(
				Origin::signed(2),
				3,
				"server_id".as_bytes().to_vec(),
				"local".as_bytes().to_vec()
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_noop!(
			Server::update_api_url(
				Origin::signed(2),
				1,
				"server_id".as_bytes().to_vec(),
				"https://api.dev.myriad.social".as_bytes().to_vec(),
			),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Server::update_api_url(
				Origin::signed(2),
				3,
				"server_id".as_bytes().to_vec(),
				"https://api.dev.myriad.social".as_bytes().to_vec(),
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_web_url_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_noop!(
			Server::update_web_url(
				Origin::signed(2),
				1,
				"server_id".as_bytes().to_vec(),
				"https://app.dev.myriad.social".as_bytes().to_vec(),
			),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_change_web_url_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Server::update_web_url(
				Origin::signed(2),
				3,
				"server_id".as_bytes().to_vec(),
				"https://app.dev.myriad.social".as_bytes().to_vec(),
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_deregister_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_noop!(
			Server::unregister(Origin::signed(2), 1, "server_id".as_bytes().to_vec()),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_deregister_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Server::unregister(Origin::signed(2), 3, "server_id".as_bytes().to_vec()),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		AdminKey::<Test>::put(2);

		assert_ok!(Server::register(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		System::assert_last_event(Event::Server(crate::Event::Registered(pallet_server::Server {
			id: "server_id".as_bytes().to_vec(),
			owner: 1,
			name: "myriad".as_bytes().to_vec(),
			api_url: "https://api.dev.myriad.social".as_bytes().to_vec(),
			web_url: "https://app.dev.myriad.social".as_bytes().to_vec(),
		})));

		assert_ok!(Server::transfer_owner(
			Origin::signed(2),
			1,
			"server_id".as_bytes().to_vec(),
			3
		));

		System::assert_last_event(Event::Server(crate::Event::OwnerTransferred(
			3,
			"server_id".as_bytes().to_vec(),
		)));

		assert_ok!(Server::update_name(
			Origin::signed(2),
			3,
			"server_id".as_bytes().to_vec(),
			"local".as_bytes().to_vec()
		));

		System::assert_last_event(Event::Server(crate::Event::NameUpdated(
			"local".as_bytes().to_vec(),
			"server_id".as_bytes().to_vec(),
		)));

		assert_ok!(Server::update_api_url(
			Origin::signed(2),
			3,
			"server_id".as_bytes().to_vec(),
			"https://api.testnet.myriad.social".as_bytes().to_vec(),
		));

		System::assert_last_event(Event::Server(crate::Event::ApiUrlUpdated(
			"https://api.testnet.myriad.social".as_bytes().to_vec(),
			"server_id".as_bytes().to_vec(),
		)));

		assert_ok!(Server::update_web_url(
			Origin::signed(2),
			3,
			"server_id".as_bytes().to_vec(),
			"https://app.testnet.myriad.social".as_bytes().to_vec(),
		));

		System::assert_last_event(Event::Server(crate::Event::WebUrlUpdated(
			"https://app.testnet.myriad.social".as_bytes().to_vec(),
			"server_id".as_bytes().to_vec(),
		)));

		assert_ok!(Server::unregister(Origin::signed(2), 3, "server_id".as_bytes().to_vec()));

		System::assert_last_event(Event::Server(crate::Event::Unregistered(
			"server_id".as_bytes().to_vec(),
		)));
	})
}

#[test]
pub fn cant_transfer_admin_key_when_not_as_admin() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(1);

		assert_noop!(Server::transfer_admin_key(Origin::signed(3), 2), Error::<Test>::Unauthorized,);
	})
}
