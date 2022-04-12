use crate::{mock::*, Error, TipsBalance, TipsBalanceInfo};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{BlakeTwo256, Hash, Zero},
};
use pallet_server::AdminKey;

#[test]
fn send_tip_myria_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);
		let tips_balance = TipsBalance::new(&tips_balance_info, &None, &1);

		assert_ok!(Tipping::send_tip(Origin::signed(2), tips_balance_info, 1));

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				"people".as_bytes().to_vec(),
				"people_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(tips_balance)
		);

		assert_eq!(Balances::free_balance(2), 19);
	})
}

#[test]
fn claim_reference_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let mut tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);
		let mut tips_balance = TipsBalance::new(&tips_balance_info, &None, &1);

		assert_ok!(Tipping::send_tip(Origin::signed(2), tips_balance_info.clone(), 1));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(1),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			None,
		));

		tips_balance.set_amount(Zero::zero());

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				"people".as_bytes().to_vec(),
				"people_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(tips_balance.clone())
		);

		tips_balance_info.set_reference_type("user".as_bytes());
		tips_balance_info.set_reference_id("user_id".as_bytes());
		tips_balance.set_tips_balance_info(&tips_balance_info);
		tips_balance.set_amount(1);

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(tips_balance.clone())
		);

		assert_ok!(Tipping::claim_reference(
			Origin::signed(1),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(3),
		));

		tips_balance.set_account_id(&Some(3));

		assert_eq!(
			Tipping::tips_balance_by_reference((
				server_id,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				"native".as_bytes().to_vec()
			)),
			Some(tips_balance)
		);
	})
}

#[test]
pub fn claim_tip_myria_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let mut tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(Origin::signed(2), tips_balance_info.clone(), 1));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(1),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			None,
		));

		tips_balance_info.set_reference_type("user".as_bytes());
		tips_balance_info.set_reference_id("user_id".as_bytes());

		assert_ok!(Tipping::claim_reference(
			Origin::signed(1),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(3),
		));

		assert_ok!(Tipping::claim_tip(Origin::signed(3), tips_balance_info,));

		assert_eq!(Balances::free_balance(3), 31);
	})
}

#[test]
fn cant_send_tip_myria_when_insufficient_balance() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::send_tip(Origin::signed(2), tips_balance_info, 21,),
			Error::<Test>::InsufficientBalance
		);
	})
}

#[test]
fn cant_send_tip_myria_when_server_id_not_register() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let tips_balance_info = TipsBalanceInfo::new(
			&BlakeTwo256::hash("server_id".as_bytes()),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::send_tip(Origin::signed(2), tips_balance_info, 1,),
			Error::<Test>::ServerNotRegister
		);
	})
}

#[test]
fn cant_send_tip_myria_when_ft_identifier_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let tips_balance_info = TipsBalanceInfo::new(
			&BlakeTwo256::hash("server_id".as_bytes()),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::send_tip(Origin::signed(2), tips_balance_info, 1,),
			Error::<Test>::ServerNotRegister
		);
	})
}

#[test]
fn cant_claim_reference_when_server_not_registered() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let tips_balance_info = TipsBalanceInfo::new(
			&BlakeTwo256::hash("server_id".as_bytes()),
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(1),
				tips_balance_info,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::ServerNotRegister,
		);
	})
}

#[test]
fn cant_claim_reference_when_not_as_server_owner() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(2),
				tips_balance_info,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
fn cant_claim_reference_when_receiver_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let mut tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(Origin::signed(2), tips_balance_info.clone(), 1));

		tips_balance_info.set_reference_type("user".as_bytes());
		tips_balance_info.set_reference_id("user_id".as_bytes());

		assert_noop!(
			Tipping::claim_reference(
				Origin::signed(1),
				tips_balance_info,
				"user".as_bytes().to_vec(),
				"user_id".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::ReceiverNotExists,
		);
	})
}

#[test]
fn cant_claim_tip_balance_when_nothing_to_claimed() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_noop!(
			Tipping::claim_tip(
				Origin::signed(1),
				TipsBalanceInfo {
					reference_id: "user_id".as_bytes().to_vec(),
					reference_type: "user".as_bytes().to_vec(),
					server_id: BlakeTwo256::hash("server_id".as_bytes()),
					ft_identifier: "native".as_bytes().to_vec(),
				},
			),
			Error::<Test>::NotExists,
		);

		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(Origin::signed(2), tips_balance_info.clone(), 0));

		assert_noop!(
			Tipping::claim_tip(Origin::signed(1), tips_balance_info,),
			Error::<Test>::NothingToClaimed,
		);
	})
}

#[test]
fn cant_claim_tip_balance_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"user".as_bytes(),
			"user_id".as_bytes(),
			"native".as_bytes(),
		);

		assert_ok!(Tipping::send_tip(Origin::signed(2), tips_balance_info.clone(), 1));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(1),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(3),
		));

		assert_noop!(
			Tipping::claim_tip(Origin::signed(4), tips_balance_info,),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		AdminKey::<Test>::put(5);

		assert_ok!(Server::register(Origin::signed(5), 1, "myriad".as_bytes().to_vec()));

		let seed =
			[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 109, 121, 114, 105, 97, 100];
		let server_id = BlakeTwo256::hash(&seed);
		let mut tips_balance_info = TipsBalanceInfo::new(
			&server_id,
			"people".as_bytes(),
			"people_id".as_bytes(),
			"native".as_bytes(),
		);
		let mut tips_balance = TipsBalance::new(&tips_balance_info, &None, &1);

		assert_ok!(Tipping::send_tip(Origin::signed(2), tips_balance_info.clone(), 1));

		let tipping_account_id = Tipping::tipping_account_id();

		System::assert_last_event(Event::Tipping(crate::Event::SendTip(
			2,
			tipping_account_id,
			tips_balance.clone(),
		)));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(1),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			None,
		));

		tips_balance.set_amount(Zero::zero());

		let mut tips_balance_1 = tips_balance.clone();
		let tips_balance_2 = tips_balance.clone();

		tips_balance_info.set_reference_type("user".as_bytes());
		tips_balance_info.set_reference_id("user_id".as_bytes());
		tips_balance_1.set_tips_balance_info(&tips_balance_info.clone());
		tips_balance_1.set_amount(1);

		System::assert_last_event(Event::Tipping(crate::Event::ClaimReference(
			tips_balance_1.clone(),
			Some(tips_balance_2),
		)));

		assert_ok!(Tipping::claim_reference(
			Origin::signed(1),
			tips_balance_info.clone(),
			"user".as_bytes().to_vec(),
			"user_id".as_bytes().to_vec(),
			Some(3),
		));

		tips_balance_1.set_account_id(&Some(3));

		System::assert_last_event(Event::Tipping(crate::Event::ClaimReference(
			tips_balance_1,
			None,
		)));

		assert_ok!(Tipping::claim_tip(Origin::signed(3), tips_balance_info,));

		System::assert_last_event(Event::Tipping(crate::Event::ClaimTip(
			tipping_account_id,
			3,
			1,
			"native".as_bytes().to_vec(),
		)));
	})
}
