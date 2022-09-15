use crate::{mock::*, Error, References, TipsBalance, TipsBalanceInfo};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError, sp_runtime::traits::Zero};

#[test]
fn send_tip_myria_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let server_id = account_key("alice");
		let tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"native");
		let tips_balance = TipsBalance::new(&tips_balance_info, &1);

		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info, 1));

		assert_eq!(
			Tipping::tips_balance_by_reference((
				&server_id,
				b"people".to_vec(),
				b"people_id".to_vec(),
				b"native".to_vec()
			)),
			Some(tips_balance)
		);

		assert_eq!(Balances::free_balance(account_key("bob")), 19);
	})
}

#[test]
fn send_tip_assets_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let server_id = account_key("alice");
		let tips_balance_info = TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"1");
		let tips_balance = TipsBalance::new(&tips_balance_info, &1);

		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info, 1));

		assert_eq!(
			Tipping::tips_balance_by_reference((
				&server_id,
				b"people".to_vec(),
				b"people_id".to_vec(),
				"1".as_bytes().to_vec()
			)),
			Some(tips_balance)
		);

		assert_eq!(Assets::balance(1, account_key("bob")), 19);
	})
}

#[test]
fn claim_reference_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let server_id = account_key("alice");
		let tips_balance_info_0 =
			TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"native");
		let tips_balance_info_1 = TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"1");
		let tips_balance_info_2 = TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"2");

		let mut tips_balance_0 = TipsBalance::new(&tips_balance_info_0, &1);
		let mut tips_balance_1 = TipsBalance::new(&tips_balance_info_1, &1);
		let mut tips_balance_2 = TipsBalance::new(&tips_balance_info_2, &2);

		let main_tips_balance_info_0 =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		let main_tips_balance_info_1 = TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"1");

		let main_tips_balance_info_2 = TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"2");

		let mut main_tips_balance_0 = TipsBalance::new(&main_tips_balance_info_0, &1);
		let mut main_tips_balance_1 = TipsBalance::new(&main_tips_balance_info_1, &1);
		let mut main_tips_balance_2 = TipsBalance::new(&main_tips_balance_info_2, &2);

		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info_0, 1));
		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info_1, 1));
		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info_2, 2));

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info_0,
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			server_id,
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec(), b"1".to_vec(), b"2".to_vec()],
			account_key("john"),
			1,
		));

		tips_balance_0.set_amount(Zero::zero());
		tips_balance_1.set_amount(Zero::zero());
		tips_balance_2.set_amount(Zero::zero());
		main_tips_balance_0.set_account_id(&account_key("john"));
		main_tips_balance_1.set_account_id(&account_key("john"));
		main_tips_balance_2.set_account_id(&account_key("john"));

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				b"people".to_vec(),
				b"people_id".to_vec(),
				b"native".to_vec()
			)),
			Some(tips_balance_0.clone())
		);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				b"people".to_vec(),
				b"people_id".to_vec(),
				b"1".to_vec()
			)),
			Some(tips_balance_1.clone())
		);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				b"people".to_vec(),
				b"people_id".to_vec(),
				b"2".to_vec()
			)),
			Some(tips_balance_2.clone())
		);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				b"user".to_vec(),
				b"user_id".to_vec(),
				b"native".to_vec()
			)),
			Some(main_tips_balance_0.clone())
		);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				b"user".to_vec(),
				b"user_id".to_vec(),
				b"1".to_vec()
			)),
			Some(main_tips_balance_1.clone())
		);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				b"user".to_vec(),
				b"user_id".to_vec(),
				b"2".to_vec()
			)),
			Some(main_tips_balance_2.clone())
		);

		assert_eq!(Balances::free_balance(account_key("alice")), 11);
	})
}

#[test]
pub fn claim_tip_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let server_id = account_key("alice");
		let tips_balance_info_0 =
			TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"native");
		let tips_balance_info_1 = TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"1");
		let tips_balance_info_2 = TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"2");

		let main_tips_balance_info_0 =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info_0, 1));
		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info_1, 1));
		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info_2, 2));
		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info_0,
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			server_id,
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec(), b"1".to_vec(), b"2".to_vec()],
			account_key("john"),
			1,
		));

		assert_ok!(Tipping::claim_tip(
			Origin::signed(account_key("john")),
			server_id,
			b"user".to_vec(),
			b"user_id".to_vec(),
			vec![b"native".to_vec(), b"1".to_vec(), b"2".to_vec()]
		));

		assert_eq!(Balances::free_balance(account_key("john")), 31);
		assert_eq!(Balances::free_balance(account_key("alice")), 11);
		assert_eq!(Assets::balance(1, account_key("john")), 31);
		assert_eq!(Assets::balance(2, account_key("john")), 32);
	})
}

#[test]
fn cant_claim_reference() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let server_id = account_key("alice");

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				account_key("bob"),
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::Unauthorized,
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				server_id,
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("alice"),
				1,
			),
			DispatchError::BadOrigin,
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				server_id,
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				0,
			),
			Error::<Test>::InsufficientBalance,
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				server_id,
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec(), b"user_idd".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::NotExists,
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				server_id,
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::NotExists,
		);

		let main_tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			0,
		));

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				server_id,
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				1,
			),
			Error::<Test>::InsufficientBalance,
		);

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info,
			1,
		));

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(account_key("alice")),
				server_id,
				References::new(b"people", &[b"people_id".to_vec()]),
				References::new(b"user", &[b"user_id".to_vec()]),
				vec![b"native".to_vec()],
				account_key("john"),
				2,
			),
			Error::<Test>::InsufficientBalance,
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let server_id = account_key("alice");
		let tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"native");
		let tips_balance_key = tips_balance_info.key();

		assert_ok!(Tipping::send_tip(Origin::signed(account_key("bob")), tips_balance_info, 1));

		let tipping_account_id = Tipping::tipping_account_id();

		System::assert_last_event(Event::Tipping(crate::Event::SendTip(
			account_key("bob"),
			tipping_account_id,
			(tips_balance_key, 1),
		)));

		let main_tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			1
		));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(account_key("alice")),
			server_id,
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec()],
			account_key("john"),
			1,
		));

		let mut main_tips_balance = TipsBalance::new(&main_tips_balance_info, &1);

		main_tips_balance.set_account_id(&account_key("john"));

		System::assert_last_event(Event::Tipping(crate::Event::ClaimReference(vec![
			main_tips_balance,
		])));

		let main_tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		assert_ok!(Tipping::send_tip(
			Origin::signed(account_key("bob")),
			main_tips_balance_info,
			1
		));

		assert_ok!(Tipping::claim_tip(
			Origin::signed(account_key("john")),
			server_id,
			b"user".to_vec(),
			b"user_id".to_vec(),
			vec![b"native".to_vec()],
		));

		System::assert_last_event(Event::Tipping(crate::Event::ClaimTip(
			tipping_account_id,
			(vec![(b"native".to_vec(), account_key("john"), 2)], None),
		)));
	})
}
