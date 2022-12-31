use crate::{mock::*, Error, Receipt, References, TipsBalance, TipsBalanceInfo};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError, sp_runtime::traits::Zero};

#[test]
fn pay_content_with_myria_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let server_id = account_key("alice");
		let sender = account_key("sender_1");
		let receiver = account_key("john");
		let tipping_account_id = Tipping::tipping_account_id();
		let amount = 100;

		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"unlockable_content",
			b"unlockable_content_id",
			b"native",
		);

		assert_ok!(Tipping::pay_content(
			RuntimeOrigin::signed(sender),
			receiver,
			tips_balance_info.clone(),
			amount
		));

		let receipt_ids = Tipping::receipt_ids();
		let receipt_id = receipt_ids[0];

		assert_eq!(Balances::free_balance(sender), 95);
		assert_eq!(Balances::free_balance(receiver), 130);
		assert_eq!(Balances::free_balance(tipping_account_id), 5);

		assert_eq!(Tipping::withdrawal_balance(b"native".to_vec()), 5);

		assert_eq!(
			Tipping::receipts(receipt_id),
			Some(Receipt::new(&receipt_id, &sender, &receiver, &tips_balance_info, &amount, &5, 0))
		);
	})
}

#[test]
fn pay_content_with_assets_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let server_id = account_key("alice");
		let sender = account_key("sender_1");
		let receiver = account_key("john");
		let tipping_account_id = Tipping::tipping_account_id();
		let amount = 100;

		let tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"unlockable_content", b"unlockable_content_id", b"1");

		assert_ok!(Tipping::pay_content(
			RuntimeOrigin::signed(sender),
			receiver,
			tips_balance_info.clone(),
			amount
		));

		let receipt_ids = Tipping::receipt_ids();
		let receipt_id = receipt_ids[0];

		assert_eq!(Assets::balance(1, sender), 95u128);
		assert_eq!(Assets::balance(1, receiver), 130u128);
		assert_eq!(Assets::balance(1, tipping_account_id), 5u128);

		assert_eq!(Tipping::withdrawal_balance(b"1".to_vec()), 5);

		assert_eq!(
			Tipping::receipts(receipt_id),
			Some(Receipt::new(&receipt_id, &sender, &receiver, &tips_balance_info, &amount, &5, 0))
		);
	})
}

#[test]
fn withdrawal_fee_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let tipping_account_id = Tipping::tipping_account_id();

		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(account_key("alice")),
			1,
			tipping_account_id,
			2
		));

		let server_id = account_key("alice");
		let sender_1 = account_key("sender_1");
		let sender_2 = account_key("sender_2");

		let receiver_1 = account_key("john");
		let receiver_2 = account_key("bob");

		let amount = 100;

		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"unlockable_content",
			b"unlockable_content_id",
			b"native",
		);

		assert_ok!(Tipping::pay_content(
			RuntimeOrigin::signed(sender_1),
			receiver_1,
			tips_balance_info,
			amount
		));

		let tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"unlockable_content", b"unlockable_content_id", b"1");

		assert_ok!(Tipping::pay_content(
			RuntimeOrigin::signed(sender_2),
			receiver_2,
			tips_balance_info,
			amount
		));

		assert_eq!(Tipping::withdrawal_balance(b"native".to_vec()), 5);
		assert_eq!(Tipping::withdrawal_balance(b"1".to_vec()), 5);

		let receiver = account_key("satoshi");

		assert_ok!(Tipping::withdraw_fee(RuntimeOrigin::root(), receiver));
		assert_eq!(Tipping::withdrawal_balance(b"native".to_vec()), 0);
		assert_eq!(Tipping::withdrawal_balance(b"1".to_vec()), 0);

		assert_eq!(Balances::free_balance(receiver), 45);
		assert_eq!(Assets::balance(1, receiver), 45);
	})
}

#[test]
fn send_tip_myria_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let server_id = account_key("alice");
		let tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"native");
		let tips_balance = TipsBalance::new(&tips_balance_info, &1);

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info,
			1
		));

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

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info,
			1
		));

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

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info_0,
			1
		));
		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info_1,
			1
		));
		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info_2,
			2
		));

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			main_tips_balance_info_0,
			1
		));

		assert_ok!(Tipping::claim_reference(
			RuntimeOrigin::signed(account_key("alice")),
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
		let tipping_account_id = Tipping::tipping_account_id();

		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(account_key("alice")),
			1,
			tipping_account_id,
			2
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(account_key("alice")),
			2,
			tipping_account_id,
			2
		));

		let server_id = account_key("alice");
		let tips_balance_info_0 =
			TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"native");
		let tips_balance_info_1 = TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"1");
		let tips_balance_info_2 = TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"2");

		let main_tips_balance_info_0 =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info_0,
			1
		));
		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info_1,
			1
		));
		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info_2,
			2
		));
		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			main_tips_balance_info_0,
			1
		));

		assert_ok!(Tipping::claim_reference(
			RuntimeOrigin::signed(account_key("alice")),
			server_id,
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec(), b"1".to_vec(), b"2".to_vec()],
			account_key("john"),
			1,
		));

		assert_ok!(Tipping::claim_tip(
			RuntimeOrigin::signed(account_key("john")),
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
				RuntimeOrigin::signed(account_key("alice")),
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
				RuntimeOrigin::signed(account_key("alice")),
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
				RuntimeOrigin::signed(account_key("alice")),
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
				RuntimeOrigin::signed(account_key("alice")),
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
				RuntimeOrigin::signed(account_key("alice")),
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
			RuntimeOrigin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			0,
		));

		assert_noop!(
			Tipping::claim_reference(
				RuntimeOrigin::signed(account_key("alice")),
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
			RuntimeOrigin::signed(account_key("bob")),
			main_tips_balance_info,
			1,
		));

		assert_noop!(
			Tipping::claim_reference(
				RuntimeOrigin::signed(account_key("alice")),
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
		// PayContent Event
		let server_id = account_key("alice");
		let sender = account_key("sender_1");
		let receiver = account_key("satoshi");
		let amount = 100;

		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			b"unlockable_content",
			b"unlockable_content_id",
			b"native",
		);

		assert_ok!(Tipping::pay_content(
			RuntimeOrigin::signed(sender),
			receiver,
			tips_balance_info.clone(),
			amount
		));

		let receipt_ids = Tipping::receipt_ids();
		let receipt_id = receipt_ids[0];

		System::assert_last_event(RuntimeEvent::Tipping(crate::Event::PayUnlockableContent(
			Receipt::new(&receipt_id, &sender, &receiver, &tips_balance_info, &amount, &5, 0),
		)));

		// Withdrawal Event
		assert_ok!(Tipping::withdraw_fee(RuntimeOrigin::root(), receiver));

		let sender = Tipping::tipping_account_id();

		System::assert_last_event(RuntimeEvent::Tipping(crate::Event::Withdrawal(
			sender,
			receiver,
			vec![(b"native".to_vec(), 5)],
		)));

		// SendTip Event
		let server_id = account_key("alice");
		let tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"people", b"people_id", b"native");
		let tips_balance_key = tips_balance_info.key();

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			tips_balance_info,
			1
		));

		let tipping_account_id = Tipping::tipping_account_id();

		System::assert_last_event(RuntimeEvent::Tipping(crate::Event::SendTip(
			account_key("bob"),
			tipping_account_id,
			(tips_balance_key, 1),
		)));

		// ClaimReference Event
		let main_tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			main_tips_balance_info.clone(),
			1
		));

		assert_ok!(Tipping::claim_reference(
			RuntimeOrigin::signed(account_key("alice")),
			server_id,
			References::new(b"people", &[b"people_id".to_vec()]),
			References::new(b"user", &[b"user_id".to_vec()]),
			vec![b"native".to_vec()],
			account_key("john"),
			1,
		));

		let mut main_tips_balance = TipsBalance::new(&main_tips_balance_info, &1);

		main_tips_balance.set_account_id(&account_key("john"));

		System::assert_last_event(RuntimeEvent::Tipping(crate::Event::ClaimReference(vec![
			main_tips_balance,
		])));

		// ClaimTip Event
		let main_tips_balance_info =
			TipsBalanceInfo::new(&server_id, b"user", b"user_id", b"native");

		assert_ok!(Tipping::send_tip(
			RuntimeOrigin::signed(account_key("bob")),
			main_tips_balance_info,
			1
		));

		assert_ok!(Tipping::claim_tip(
			RuntimeOrigin::signed(account_key("john")),
			server_id,
			b"user".to_vec(),
			b"user_id".to_vec(),
			vec![b"native".to_vec()],
		));

		System::assert_last_event(RuntimeEvent::Tipping(crate::Event::ClaimTip(
			tipping_account_id,
			(vec![(b"native".to_vec(), account_key("john"), 2)], None),
		)));
	})
}
