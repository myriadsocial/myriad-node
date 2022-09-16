use crate::{self as pallet_server, mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn register_works() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0_u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let server = pallet_server::Server::new(server_id, &owner, &api_url);

		assert_ok!(Server::register(owner_origin, api_url.clone()));

		assert_eq!(Server::server_by_id(server_id), Some(server.clone()));
		assert_eq!(Server::server_by_owner(owner, server_id), Some(server));
		assert_eq!(Server::server_by_api_url(api_url), Some(server_id));
		assert_eq!(Server::server_count(), 1);
		assert_eq!(Server::server_index(), 1);
	})
}

#[test]
pub fn transfer_owner_works() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0_u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let new_owner = 2;
		let server = pallet_server::Server::new(server_id, &new_owner, &api_url);

		assert_ok!(Server::register(Origin::signed(owner), api_url));
		assert_ok!(Server::transfer_owner(owner_origin, 0, new_owner));

		assert_eq!(Server::server_by_id(server_id), Some(server.clone()));
		assert_eq!(Server::server_by_owner(owner, server_id), None);
		assert_eq!(Server::server_by_owner(new_owner, server_id), Some(server));
	})
}

#[test]
pub fn change_api_url_works() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0_u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
		let server = pallet_server::Server::new(server_id, &owner, &new_api_url);

		assert_ok!(Server::register(Origin::signed(owner), api_url.clone()));
		assert_ok!(Server::update_api_url(owner_origin, 0, new_api_url.clone()));

		assert_eq!(Server::server_by_api_url(api_url), None);
		assert_eq!(Server::server_by_id(server_id), Some(server));
		assert_eq!(Server::server_by_api_url(new_api_url), Some(server_id));
	})
}

#[test]
pub fn deregister_works() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0_u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::register(Origin::signed(owner), api_url.clone()));
		assert_ok!(Server::unregister(owner_origin, 0));

		assert_eq!(Server::server_by_id(server_id), None);
		assert_eq!(Server::server_by_owner(owner, server_id), None);
		assert_eq!(Server::server_by_api_url(api_url), None);
		assert_eq!(Server::server_count(), 0);
		assert_eq!(Server::server_index(), 1);
	})
}

#[test]
pub fn cant_register_when_api_url_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);
		let api_url = b"https://api.dev.myriad.social".to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let other_owner = 2;
		let other_owner_origin = Origin::signed(other_owner);
		let other_api_url = b"https://api.dev.myriad.social".to_vec();

		assert_noop!(
			Server::register(other_owner_origin, other_api_url),
			Error::<Test>::AlreadyExists,
		);
	})
}

#[test]
pub fn cant_transfer_owner_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let fake_id = 0;
		let new_owner = 2;

		assert_noop!(
			Server::transfer_owner(owner_origin, fake_id, new_owner),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_transfer_owner_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let fake_owner = 3;
		let fake_owner_origin = Origin::signed(fake_owner);
		let new_owner = 4;

		assert_noop!(
			Server::transfer_owner(fake_owner_origin, server_id, new_owner),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let fake_id = 0;
		let new_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_noop!(
			Server::update_api_url(owner_origin, fake_id, new_api_url),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let fake_owner = 3;
		let fake_owner_origin = Origin::signed(fake_owner);
		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();

		assert_noop!(
			Server::update_api_url(fake_owner_origin, server_id, new_api_url),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_api_url_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0;
		let api_url = b"https://api.dev.myriad.social".to_vec();

		assert_ok!(Server::register(owner_origin.clone(), api_url));

		let other_owner = 2;
		let other_owner_origin = Origin::signed(other_owner);
		let other_api_url = b"https://api.testnet.myriad.social".to_vec();

		assert_ok!(Server::register(other_owner_origin, other_api_url));

		let new_api_url = b"https://api.testnet.myriad.social".to_vec();

		assert_noop!(
			Server::update_api_url(owner_origin, server_id, new_api_url),
			Error::<Test>::AlreadyExists,
		);
	})
}

#[test]
pub fn cant_deregister_when_server_id_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let fake_id = 0;

		assert_noop!(Server::unregister(owner_origin, fake_id), Error::<Test>::NotExists);
	})
}

#[test]
pub fn cant_deregister_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		let owner = 1;
		let owner_origin = Origin::signed(owner);

		let server_id = 0;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let fake_owner = 3;
		let fake_owner_origin = Origin::signed(fake_owner);

		assert_noop!(Server::unregister(fake_owner_origin, server_id), Error::<Test>::Unauthorized,);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let owner = 1;

		let server_id = 0;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let server = pallet_server::Server::new(server_id, &owner, &api_url);

		assert_ok!(Server::register(Origin::signed(owner), api_url));

		System::assert_last_event(Event::Server(crate::Event::Registered(server)));

		let new_owner = 3;

		assert_ok!(Server::transfer_owner(Origin::signed(owner), server_id, new_owner));

		System::assert_last_event(Event::Server(crate::Event::OwnerTransferred(
			new_owner, server_id,
		)));

		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::update_api_url(
			Origin::signed(new_owner),
			server_id,
			new_api_url.clone()
		));

		System::assert_last_event(Event::Server(crate::Event::ApiUrlUpdated(
			new_api_url,
			server_id,
		)));

		assert_ok!(Server::unregister(Origin::signed(new_owner), server_id));

		System::assert_last_event(Event::Server(crate::Event::Unregistered(server_id)));
	})
}
