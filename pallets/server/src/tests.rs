use crate::{self as pallet_server, mock::*, Action, Error, Status};
use frame_support::{assert_noop, assert_ok, traits::OnInitialize};

#[test]
fn register_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let server = pallet_server::Server::new(server_id, &owner, &api_url, 3);
		let server_account_id = Server::server_account_id(server_id);

		assert_ok!(Server::register(owner_origin, api_url.clone()));

		assert_eq!(Server::server_by_id(server_id), Some(server.clone()));
		assert_eq!(Server::server_by_owner(owner, server_id), Some(server));
		assert_eq!(Server::server_by_api_url(api_url), Some(server_id));
		assert_eq!(Server::server_count(), 1);
		assert_eq!(Server::server_index(), 1);
		assert_eq!(Balances::free_balance(owner), 7);
		assert_eq!(Balances::free_balance(server_account_id), 3);
	})
}

#[test]
pub fn transfer_owner_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let new_owner = account_key("bob");
		let server = pallet_server::Server::new(server_id, &new_owner, &api_url, 3);

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url));
		assert_ok!(Server::transfer_owner(owner_origin, 0, new_owner));

		assert_eq!(Server::server_by_id(server_id), Some(server.clone()));
		assert_eq!(Server::server_by_owner(owner, server_id), None);
		assert_eq!(Server::server_by_owner(new_owner, server_id), Some(server));
	})
}

#[test]
pub fn change_api_url_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
		let server = pallet_server::Server::new(server_id, &owner, &new_api_url, 3);

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url.clone()));
		assert_ok!(Server::update_api_url(owner_origin, 0, new_api_url.clone()));

		assert_eq!(Server::server_by_api_url(api_url), None);
		assert_eq!(Server::server_by_id(server_id), Some(server));
		assert_eq!(Server::server_by_api_url(new_api_url), Some(server_id));
	})
}

#[test]
pub fn deregister_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		System::set_block_number(10);

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url));
		assert_ok!(Server::unregister(owner_origin, server_id));

		assert_eq!(Server::tasks(20), vec![0]);
	})
}

#[test]
pub fn increase_stake_amount_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_id = 0u64;
		let amount = 3;

		let server = pallet_server::Server::new(server_id, &owner, &api_url, 6);
		let server_account_id = Server::server_account_id(server_id);

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url));
		assert_ok!(Server::update_stake_amount(
			RuntimeOrigin::signed(owner),
			server_id,
			Action::Stake(amount)
		));

		assert_eq!(Server::server_by_id(server_id), Some(server.clone()));
		assert_eq!(Server::server_by_owner(owner, server_id), Some(server));
		assert_eq!(Balances::free_balance(owner), 4);
		assert_eq!(Balances::free_balance(server_account_id), 6);
	})
}

#[test]
pub fn decrease_stake_amount_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_id = 0u64;
		let amount = 3;

		let server = pallet_server::Server::new(server_id, &owner, &api_url, 3);
		let server_account_id = Server::server_account_id(server_id);

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url));
		assert_ok!(Server::update_stake_amount(
			RuntimeOrigin::signed(owner),
			server_id,
			Action::Stake(amount)
		));
		assert_ok!(Server::update_stake_amount(
			RuntimeOrigin::signed(owner),
			server_id,
			Action::Unstake(amount)
		));

		assert_eq!(Server::server_by_id(server_id), Some(server.clone()));
		assert_eq!(Server::server_by_owner(owner, server_id), Some(server));
		assert_eq!(Balances::free_balance(owner), 7);
		assert_eq!(Balances::free_balance(server_account_id), 3);
	})
}

#[test]
pub fn unstake_server_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server = pallet_server::Server::new(server_id, &owner, &api_url, 0);

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url.clone()));
		assert_ok!(Server::unregister(RuntimeOrigin::signed(owner), server_id));

		let other_owner = account_key("bob");
		let other_server_id = 1u64;
		let other_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();

		System::set_block_number(2);

		assert_ok!(Server::register(RuntimeOrigin::signed(other_owner), other_api_url));
		assert_ok!(Server::unregister(RuntimeOrigin::signed(other_owner), other_server_id));

		System::set_block_number(11);

		<Server as OnInitialize<u64>>::on_initialize(11);

		let server_account_id = Server::server_account_id(server_id);

		assert_eq!(Server::server_by_id(server_id), None);
		assert_eq!(Server::server_by_owner(owner, server_id), Some(server));
		assert_eq!(Server::server_by_api_url(api_url), None);
		assert_eq!(Server::server_count(), 1);
		assert_eq!(Server::server_index(), 2);
		assert_eq!(Server::tasks(11), Vec::<u64>::new());
		assert_eq!(Balances::free_balance(server_account_id), 0);
		assert_eq!(Balances::free_balance(owner), 10);
	})
}

#[test]
pub fn cant_register_when_api_url_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);
		let api_url = b"https://api.dev.myriad.social".to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let other_owner = account_key("bob");
		let other_owner_origin = RuntimeOrigin::signed(other_owner);
		let other_api_url = b"https://api.dev.myriad.social".to_vec();

		assert_noop!(
			Server::register(other_owner_origin, other_api_url),
			Error::<Test>::AlreadyExists,
		);
	})
}

#[test]
pub fn cant_register_when_balance_insufficient() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("satoshi");
		let owner_origin = RuntimeOrigin::signed(owner);
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_noop!(Server::register(owner_origin, api_url), Error::<Test>::InsufficientBalance,);
	})
}

#[test]
pub fn cant_transfer_owner_when_server_id_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let fake_id = 0u64;
		let new_owner = account_key("bob");

		assert_noop!(
			Server::transfer_owner(owner_origin, fake_id, new_owner),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_transfer_owner_when_not_owner() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let fake_owner = account_key("bob");
		let fake_owner_origin = RuntimeOrigin::signed(fake_owner);
		let new_owner = account_key("john");

		assert_noop!(
			Server::transfer_owner(fake_owner_origin, server_id, new_owner),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_server_id_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let fake_id = 0u64;
		let new_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_noop!(
			Server::update_api_url(owner_origin, fake_id, new_api_url),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_not_owner() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let fake_owner = account_key("bob");
		let fake_owner_origin = RuntimeOrigin::signed(fake_owner);
		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();

		assert_noop!(
			Server::update_api_url(fake_owner_origin, server_id, new_api_url),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_change_api_url_when_api_url_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = b"https://api.dev.myriad.social".to_vec();

		assert_ok!(Server::register(owner_origin.clone(), api_url));

		let other_owner = account_key("bob");
		let other_owner_origin = RuntimeOrigin::signed(other_owner);
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
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let fake_id = 0u64;

		assert_noop!(Server::unregister(owner_origin, fake_id), Error::<Test>::NotExists);
	})
}

#[test]
pub fn cant_deregister_when_not_owner() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::register(owner_origin, api_url));

		let fake_owner = account_key("bob");
		let fake_owner_origin = RuntimeOrigin::signed(fake_owner);

		assert_noop!(Server::unregister(fake_owner_origin, server_id), Error::<Test>::Unauthorized,);
	})
}

#[test]
pub fn cant_deregister_when_max_scheduled_per_block_over_limit() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("john");
		let owner_origin = RuntimeOrigin::signed(owner);

		System::set_block_number(1);

		for n in 0..6 {
			assert_ok!(Server::register(owner_origin.clone(), vec![n]));
		}

		for n in 0..5 {
			assert_ok!(Server::unregister(owner_origin.clone(), n));
		}

		assert_noop!(Server::unregister(owner_origin, 5), Error::<Test>::FailedToSchedule);
	})
}

#[test]
pub fn cant_increase_stake_amount_when_server_id_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let server_id = 0u64;
		let amount = 10;

		assert_noop!(
			Server::update_stake_amount(
				RuntimeOrigin::signed(owner),
				server_id,
				Action::Stake(amount)
			),
			Error::<Test>::NotExists,
		);
	})
}

#[test]
pub fn cant_increase_stake_amount_when_not_owner() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_id = 0u64;
		let amount = 10;

		let other_owner = account_key("bob");

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url));
		assert_noop!(
			Server::update_stake_amount(
				RuntimeOrigin::signed(other_owner),
				server_id,
				Action::Stake(amount)
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
pub fn cant_increase_stake_amount_when_balance_insufficient() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
		let server_id = 0u64;
		let amount = 13;

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url));
		assert_noop!(
			Server::update_stake_amount(
				RuntimeOrigin::signed(owner),
				server_id,
				Action::Stake(amount)
			),
			Error::<Test>::BadSignature,
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		System::set_block_number(1);

		let owner = account_key("alice");

		let server_id = 0u64;
		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

		let server = pallet_server::Server::new(server_id, &owner, &api_url, 3);

		assert_ok!(Server::register(RuntimeOrigin::signed(owner), api_url));

		System::assert_has_event(RuntimeEvent::Server(crate::Event::Registered(server)));
		System::assert_has_event(RuntimeEvent::Server(crate::Event::Staked(owner, server_id, 3)));

		assert_ok!(Server::update_stake_amount(
			RuntimeOrigin::signed(owner),
			server_id,
			Action::Stake(3)
		));

		System::assert_last_event(RuntimeEvent::Server(crate::Event::StakedAmountUpdated(
			owner,
			server_id,
			Action::Stake(3),
		)));

		let new_owner = account_key("bob");

		assert_ok!(Server::transfer_owner(RuntimeOrigin::signed(owner), server_id, new_owner));

		System::assert_last_event(RuntimeEvent::Server(crate::Event::OwnerTransferred(
			new_owner, server_id,
		)));

		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();

		assert_ok!(Server::update_api_url(
			RuntimeOrigin::signed(new_owner),
			server_id,
			new_api_url.clone()
		));

		System::assert_last_event(RuntimeEvent::Server(crate::Event::ApiUrlUpdated(
			new_api_url,
			server_id,
		)));

		assert_ok!(Server::unregister(RuntimeOrigin::signed(new_owner), server_id));

		System::assert_last_event(RuntimeEvent::Server(crate::Event::Scheduled {
			server_id,
			when: 11,
			task: b"Unstaked".to_vec(),
			status: Status::InProgress,
		}));

		System::set_block_number(11);

		<Server as OnInitialize<u64>>::on_initialize(11);

		System::assert_has_event(RuntimeEvent::Server(crate::Event::Unstaked(
			new_owner, server_id, 6,
		)));
		System::assert_has_event(RuntimeEvent::Server(crate::Event::Unregistered(server_id)));
		System::assert_last_event(RuntimeEvent::Server(crate::Event::Scheduled {
			server_id,
			when: 11,
			task: b"Unstaked".to_vec(),
			status: Status::Success,
		}));
	})
}
