use crate::{self as pallet_escrow, mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn send_tip_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::signed(1),
			String::from("MYRIA").into_bytes(),
			18,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		assert_ok!(Platform::add_platform(Origin::signed(1), String::from("twitter").into_bytes()));
		assert_ok!(Escrow::send_tip(
			Origin::signed(1),
			pallet_escrow::Post {
				post_id: String::from("60efac8c565ab8004ed28bb3").into_bytes(),
				people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
				platform: String::from("twitter").into_bytes()
			},
			String::from("MYRIA").into_bytes(),
			100
		));
		assert_ok!(Escrow::send_tip(
			Origin::signed(1),
			pallet_escrow::Post {
				post_id: String::from("60efac8c565ab8004ed28bb5").into_bytes(),
				people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
				platform: String::from("twitter").into_bytes()
			},
			String::from("MYRIA").into_bytes(),
			100
		));

		assert_eq!(
			Escrow::people_balance((
				String::from("MYRIA").into_bytes(),
				String::from("60efac8c565ab8004ed28ba6").into_bytes(),
				String::from("twitter").into_bytes()
			)),
			200
		);
		assert_eq!(
			Escrow::post_balance((
				String::from("MYRIA").into_bytes(),
				String::from("60efac8c565ab8004ed28bb3").into_bytes(),
				String::from("60efac8c565ab8004ed28ba6").into_bytes(),
				String::from("twitter").into_bytes()
			)),
			100
		);
		assert_eq!(
			Escrow::post_balance((
				String::from("MYRIA").into_bytes(),
				String::from("60efac8c565ab8004ed28bb5").into_bytes(),
				String::from("60efac8c565ab8004ed28ba6").into_bytes(),
				String::from("twitter").into_bytes()
			)),
			100
		);
	});
}

#[test]
fn cant_send_tip_when_platform_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::signed(1),
			String::from("MYRIA").into_bytes(),
			18,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));

		assert_noop!(
			Escrow::send_tip(
				Origin::signed(1),
				pallet_escrow::Post {
					post_id: String::from("60efac8c565ab8004ed28bb3").into_bytes(),
					people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
					platform: String::from("twitter").into_bytes()
				},
				String::from("MYRIA").into_bytes(),
				100
			),
			Error::<Test>::PlatformNotExist
		);
	})
}

#[test]
fn cant_send_tip_when_currency_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Platform::add_platform(Origin::signed(1), String::from("twitter").into_bytes()));

		assert_noop!(
			Escrow::send_tip(
				Origin::signed(1),
				pallet_escrow::Post {
					post_id: String::from("60efac8c565ab8004ed28bb3").into_bytes(),
					people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
					platform: String::from("twitter").into_bytes()
				},
				String::from("MYRIA").into_bytes(),
				100
			),
			Error::<Test>::CurrencyNotExist
		);
	})
}

#[test]
fn cant_send_tip_when_amount_is_zero() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Platform::add_platform(Origin::signed(1), String::from("twitter").into_bytes()));
		assert_ok!(Currencies::add_currency(
			Origin::signed(1),
			String::from("MYRIA").into_bytes(),
			18,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));

		assert_noop!(
			Escrow::send_tip(
				Origin::signed(1),
				pallet_escrow::Post {
					post_id: String::from("60efac8c565ab8004ed28bb3").into_bytes(),
					people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
					platform: String::from("twitter").into_bytes()
				},
				String::from("MYRIA").into_bytes(),
				0
			),
			Error::<Test>::InsufficientAmount
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Currencies::add_currency(
			Origin::signed(1),
			String::from("MYRIA").into_bytes(),
			18,
			String::from("wss://rpc.myriad.systems").into_bytes(),
			true
		));
		assert_ok!(Currencies::update_balance(
			Origin::signed(1),
			1,
			String::from("MYRIA").into_bytes(),
			21000000
		));
		assert_ok!(Platform::add_platform(Origin::signed(1), String::from("twitter").into_bytes()));
		assert_ok!(Escrow::send_tip(
			Origin::signed(1),
			pallet_escrow::Post {
				post_id: String::from("60efac8c565ab8004ed28bb3").into_bytes(),
				people_id: String::from("60efac8c565ab8004ed28ba6").into_bytes(),
				platform: String::from("twitter").into_bytes()
			},
			String::from("MYRIA").into_bytes(),
			100
		));
		System::assert_last_event(Event::Escrow(crate::Event::TipReceived(
			String::from("MYRIA").into_bytes(),
			100,
			pallet_currency::CurrencyBalance { free: 100 },
			Escrow::account_id(),
			1,
		)));
	})
}
