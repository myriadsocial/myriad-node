use crate::{mock::*, Error, References, TipsBalance, TipsBalanceInfo};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::{traits::Zero, SaturatedConversion},
};
use pallet_server::AdminKey;

#[test]
fn send_tip_myria_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);
		let tips_balance = TipsBalance::new(&tips_balance_info, &None, &1);

		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info, 1));

		assert_eq!(
			Tipping::tips_balance_by_reference((
				"myriad".as_bytes().to_vec(),
				"people".as_bytes().to_vec(),
				"people_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(tips_balance)
		);

		assert_eq!(Balances::free_balance(account_key("bob")), 19);
	})
}

#[test]
fn claim_reference_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			tips_balance_info.clone(),
			1
		));

		let main_tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(account_key("john")),
			1u128.saturated_into(),
		));

		let main_tips_balance =
			TipsBalance::new(&main_tips_balance_info, &Some(account_key("john")), &1);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				"myriad".as_bytes().to_vec(),
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(main_tips_balance)
		);

		let tips_balance = TipsBalance::new(&tips_balance_info, &None, &0);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				"myriad".as_bytes().to_vec(),
				"people".as_bytes().to_vec(),
				"people_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(tips_balance)
		);

		assert_eq!(Balances::free_balance(account_key("alice")), 11);
	})
}

#[test]
fn batch_claim_reference_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);
		let mut tips_balance = TipsBalance::new(&tips_balance_info, &None, &1);

		let main_tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		let mut main_tips_balance = TipsBalance::new(&main_tips_balance_info, &None, &1);

		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info, 1));

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info,
			1
		));

		assert_ok!(Tipping::batch_claim_reference(
			Origin::signed(account_key("alice")),
			"myriad".as_bytes().to_vec(),
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec()],
			account_key("john"),
			1,
		));

		tips_balance.set_amount(Zero::zero());
		main_tips_balance.set_account_id(&Some(account_key("john")));

		assert_eq!(
			Tipping::tips_balance_by_reference((
				"myriad".as_bytes().to_vec(),
				"people".as_bytes().to_vec(),
				"people_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(tips_balance.clone())
		);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				"myriad".as_bytes().to_vec(),
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(main_tips_balance.clone())
		);

		assert_eq!(Balances::free_balance(account_key("alice")), 11);
	})
}

#[test]
pub fn claim_tip_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			tips_balance_info.clone(),
			1
		));

		let main_tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			tips_balance_info,
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(account_key("john")),
			1u128.saturated_into(),
		));

		assert_ok!(Tipping::claim_tip(Origin::signed(account_key("john")), main_tips_balance_info));

		assert_eq!(Balances::free_balance(account_key("john")), 31);
		assert_eq!(Balances::free_balance(account_key("alice")), 11);
	})
}

#[test]
pub fn batch_claim_tip_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			tips_balance_info.clone(),
			1
		));

		let main_tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info,
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			tips_balance_info,
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(account_key("john")),
			1u128.saturated_into(),
		));

		assert_ok!(Tipping::batch_claim_tip(
			Origin::signed(account_key("john")),
			b"myriad".to_vec(),
			b"user".to_vec(),
			b"user_id".to_vec(),
			vec![b"native".to_vec()]
		));

		assert_eq!(Balances::free_balance(account_key("john")), 31);
		assert_eq!(Balances::free_balance(account_key("alice")), 11);
	})
}

#[test]
fn cant_send_tip_myria_when_insufficient_balance() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info, 21),
			Error::<Test>::InsufficientBalance
		);
	})
}

#[test]
fn cant_send_tip_myria_when_server_id_not_register() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::send_tip(Origin::signed(account_key("alice")), tips_balance_info, 1),
			Error::<Test>::ServerNotRegister
		);
	})
}

#[test]
fn cant_send_tip_myria_when_ft_identifier_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::send_tip(Origin::signed(account_key("alice")), tips_balance_info, 1,),
			Error::<Test>::ServerNotRegister
		);
	})
}

#[test]
fn cant_claim_reference_when_server_not_registered() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				tips_balance_info,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				None,
				1u128.saturated_into(),
			),
			Error::<Test>::ServerNotRegister,
		);
	})
}

#[test]
fn cant_claim_reference_when_not_as_server_owner() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("bob")),
				tips_balance_info,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				None,
				1u128.saturated_into(),
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
fn cant_claim_reference_when_receiver_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let mut tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			tips_balance_info.clone(),
			1
		));

		tips_balance_info.set_reference_type("user".as_bytes());
		tips_balance_info.set_reference_id("user_id".as_bytes());

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				tips_balance_info,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				None,
				1u128.saturated_into(),
			),
			Error::<Test>::ReceiverNotExists,
		);
	})
}

#[test]
fn cant_batch_claim_reference_when_not_server_not_register() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::ServerNotRegister,
		);
	})
}

#[test]
fn cant_batch_claim_reference_when_not_server_owner() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("satoshi"),
			"myriad_1".as_bytes().to_vec(),
			"myriad_1".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad_1".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("alice"),
				1,
			),
			Error::<Test>::FailedToVerify,
		);
	})
}

#[test]
fn cant_batch_claim_reference() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("alice"),
				1,
			),
			Error::<Test>::FailedToVerify,
		);

		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				0,
			),
			Error::<Test>::FailedToVerify,
		);

		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec(), b"user_idd".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::FailedToVerify,
		);

		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::FailedToVerify,
		);

		let main_tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			0,
		));

		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::FailedToVerify,
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info,
			1,
		));

		assert_noop!(
			Tipping::batch_claim_reference(
				Origin::signed(account_key("alice")),
				"myriad".as_bytes().to_vec(),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				2,
			),
			Error::<Test>::FailedToVerify,
		);
	})
}

#[test]
fn cant_claim_tip_balance_when_nothing_to_claimed() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_noop!(
			Tipping::claim_tip(
				Origin::signed(account_key("alice")),
				TipsBalanceInfo::new(
					"myriad".as_bytes(),
					"user".as_bytes(),
					"user_id".as_bytes(),
					"native".as_bytes()
				),
			),
			Error::<Test>::NotExists,
		);

		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			tips_balance_info.clone(),
			0
		));

		assert_noop!(
			Tipping::claim_tip(Origin::signed(account_key("alice")), tips_balance_info,),
			Error::<Test>::NothingToClaimed,
		);
	})
}

#[test]
fn cant_claim_tip_balance_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			tips_balance_info.clone(),
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(account_key("john")),
			1u128.saturated_into(),
		));

		assert_noop!(
			Tipping::claim_tip(Origin::signed(account_key("satoshi")), tips_balance_info,),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(account_key("admin"));

		assert_ok!(Server::register(
			Origin::signed(account_key("admin")),
			account_key("alice"),
			"myriad".as_bytes().to_vec(),
			"myriad".as_bytes().to_vec(),
			"https://api.dev.myriad.social".as_bytes().to_vec(),
			"https://app.dev.myriad.social".as_bytes().to_vec(),
		));

		let tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);
		let tips_balance = TipsBalance::new(&tips_balance_info, &None, &1);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			tips_balance_info.clone(),
			1
		));

		let tipping_account_id = Tipping::tipping_account_id();

		System::assert_last_event(Event::Tipping(crate::Event::SendTip(
			account_key("bob"),
			tipping_account_id,
			tips_balance,
		)));

		let main_tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			tips_balance_info,
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(account_key("john")),
			1u128.saturated_into(),
		));

		let main_tips_balance =
			TipsBalance::new(&main_tips_balance_info, &Some(account_key("john")), &1);

		System::assert_last_event(Event::Tipping(crate::Event::ClaimReference(vec![
			main_tips_balance,
		])));

		assert_ok!(Tipping::batch_claim_reference(
			Origin::signed(account_key("alice")),
			"myriad".as_bytes().to_vec(),
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec()],
			account_key("john"),
			1,
		));

		let main_tips_balance =
			TipsBalance::new(&main_tips_balance_info, &Some(account_key("john")), &0);

		System::assert_last_event(Event::Tipping(crate::Event::ClaimReference(vec![
			main_tips_balance,
		])));

		let main_tips_balance_info = TipsBalanceInfo::new(
			"myriad".as_bytes(),
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			1
		));

		assert_ok!(
			Tipping::claim_tip(Origin::signed(account_key("john")), main_tips_balance_info,)
		);

		System::assert_last_event(Event::Tipping(crate::Event::ClaimTip(
			tipping_account_id,
			account_key("john"),
			1,
			"native".as_bytes().to_vec(),
		)));
	})
}
