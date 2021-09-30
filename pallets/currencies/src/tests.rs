use crate::{
	self as pallet_currency,
	mock::{Event, *},
	Error,
};
use frame_support::{assert_noop, assert_ok};

#[test]
fn update_balance_work() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		assert_ok!(Currencies::update_balance(
			Origin::root(),
			1,
			String::from("ACA").into_bytes(),
			21000000
		));

		assert_eq!(
			Currencies::accounts(1, String::from("ACA").into_bytes()),
			Some(pallet_currency::CurrencyBalance { free: 21000000 })
		);
	})
}

#[test]
fn transfer_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		assert_ok!(Currencies::update_balance(
			Origin::root(),
			1,
			String::from("ACA").into_bytes(),
			21000000
		));
		assert_ok!(Currencies::transfer(
			Origin::signed(1),
			2,
			String::from("ACA").into_bytes(),
			100
		));

		assert_eq!(
			Currencies::accounts(1, String::from("ACA").into_bytes()),
			Some(pallet_currency::CurrencyBalance { free: 20999900 })
		);
		assert_eq!(
			Currencies::accounts(2, String::from("ACA").into_bytes()),
			Some(pallet_currency::CurrencyBalance { free: 100 })
		);
	})
}

#[test]
fn add_currency_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));

		assert_eq!(
			Currencies::currency(String::from("ACA").into_bytes()),
			Some(pallet_currency::CurrencyInfo {
				decimal: 12,
				rpc_url: String::from("wss://rpc.myriad.systems").into_bytes(),
				native: true
			})
		);
	})
}

#[test]
fn cant_add_existing_currency() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));

		assert_noop!(
			Currencies::add_currency(
				Origin::root(),
				String::from("ACA").into_bytes(),
				12,
				String::from("wss://rpc.myriad.systems").into_bytes(),
				true
			),
			Error::<Test>::CurrencyExist
		);
	})
}

#[test]
fn cant_transfer_when_currency_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Currencies::transfer(Origin::signed(1), 2, String::from("AUSD").into_bytes(), 21000000),
			Error::<Test>::CurrencyNotExist
		);
	})
}

#[test]
fn cant_spend_more_than_you_have() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		assert_ok!(Currencies::update_balance(
			Origin::root(),
			1,
			String::from("ACA").into_bytes(),
			21000000
		));

		assert_noop!(
			Currencies::transfer(Origin::signed(1), 2, String::from("ACA").into_bytes(), 21000001),
			Error::<Test>::InsufficientFunds
		);
	})
}

#[test]
fn cant_transfer_to_same_account() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		assert_ok!(Currencies::update_balance(
			Origin::root(),
			1,
			String::from("ACA").into_bytes(),
			21000000
		));

		assert_noop!(
			Currencies::transfer(Origin::signed(1), 1, String::from("ACA").into_bytes(), 21000000),
			Error::<Test>::BadOrigin
		);
	})
}

#[test]
fn cant_set_transfer_amount_to_zero() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		assert_ok!(Currencies::update_balance(
			Origin::root(),
			1,
			String::from("ACA").into_bytes(),
			21000000
		));

		assert_noop!(
			Currencies::transfer(Origin::signed(1), 2, String::from("ACA").into_bytes(), 0),
			Error::<Test>::InsufficientAmount
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Currencies::add_currency(
			Origin::root(),
			String::from("ACA").into_bytes(),
			12,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		System::assert_last_event(Event::Currencies(crate::Event::NewCurrencyAdded(
			String::from("ACA").into_bytes(),
		)));

		assert_ok!(Currencies::update_balance(
			Origin::root(),
			1,
			String::from("ACA").into_bytes(),
			21000000
		));
		System::assert_last_event(Event::Currencies(crate::Event::BalanceUpdated(
			String::from("ACA").into_bytes(),
			1,
			21000000,
		)));

		assert_ok!(Currencies::transfer(
			Origin::signed(1),
			2,
			String::from("ACA").into_bytes(),
			100
		));
		System::assert_last_event(Event::Currencies(crate::Event::Transferred(
			String::from("ACA").into(),
			1,
			2,
			100,
		)));
	})
}
