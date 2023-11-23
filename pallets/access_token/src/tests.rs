use crate::{self as pallet_access_token, mock::*, Error, Scopes, TimelineId};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
	traits::OnInitialize,
};

#[test]
fn create_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin,
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);
	})
}

#[test]
pub fn cant_create_when_hash_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_noop!(
			AccessToken::create(
				owner_origin,
				Keccak256::hash("hash".as_bytes()),
				Scopes::<TimelineId>::default()
			),
			Error::<Test>::AlreadyExists,
		);

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);
	})
}

#[test]
fn revoke_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);

		assert_ok!(AccessToken::revoke(owner_origin, Keccak256::hash("hash".as_bytes())));

		assert_eq!(AccessToken::all_access_tokens_by_owner(owner), Some(vec![]));
		assert_eq!(AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())), None);
		assert_eq!(AccessToken::access_token_count(), 0);
		assert_eq!(AccessToken::access_token_index(), 1);
	})
}

#[test]
fn cant_revoke_when_hash_does_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);

		assert_noop!(
			AccessToken::revoke(owner_origin, Keccak256::hash("hash2".as_bytes())),
			Error::<Test>::NotExists,
		);

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);
	})
}

#[test]
fn cant_revoke_when_not_owner() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin,
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);

		let owner_2 = account_key("bob");
		let owner_origin_2 = RuntimeOrigin::signed(owner_2);

		assert_noop!(
			AccessToken::revoke(owner_origin_2, Keccak256::hash("hash".as_bytes())),
			Error::<Test>::Unauthorized,
		);

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);
	})
}

#[test]
fn revoke_all_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);

		let access_token_2 = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone(), access_token_2.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())),
			Some(access_token_2)
		);
		assert_eq!(AccessToken::access_token_count(), 2);
		assert_eq!(AccessToken::access_token_index(), 2);

		assert_ok!(AccessToken::revoke_all(owner_origin));

		assert_eq!(AccessToken::all_access_tokens_by_owner(owner), None);
		assert_eq!(AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())), None);
		assert_eq!(AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())), None);
		assert_eq!(AccessToken::access_token_count(), 0);
		assert_eq!(AccessToken::access_token_index(), 2);
	})
}

#[test]
fn cant_revoke_all_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);

		let access_token_2 = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone(), access_token_2.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())),
			Some(access_token_2.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 2);
		assert_eq!(AccessToken::access_token_index(), 2);

		let owner_2 = account_key("bob");
		let owner_origin_2 = RuntimeOrigin::signed(owner_2);

		assert_noop!(AccessToken::revoke_all(owner_origin_2), Error::<Test>::NotExists,);

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone(), access_token_2.clone()])
		);

		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);

		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())),
			Some(access_token_2)
		);

		assert_eq!(AccessToken::access_token_count(), 2);
		assert_eq!(AccessToken::access_token_index(), 2);
	})
}

#[test]
fn revoke_all_by_scopes_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);

		let access_token_2 = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone(), access_token_2.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())),
			Some(access_token_2)
		);
		assert_eq!(AccessToken::access_token_count(), 2);
		assert_eq!(AccessToken::access_token_index(), 2);

		assert_ok!(AccessToken::revoke_all_by_scopes(
			owner_origin,
			Scopes::<TimelineId>::default()
		));

		assert_eq!(AccessToken::all_access_tokens_by_owner(owner), Some(vec![]));
		assert_eq!(AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())), None);
		assert_eq!(AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())), None);
		assert_eq!(AccessToken::access_token_count(), 0);
		assert_eq!(AccessToken::access_token_index(), 2);
	})
}

#[test]
fn cant_revoke_all_by_scopes_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);

		let access_token_2 = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin.clone(),
			Keccak256::hash("hash2".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone(), access_token_2.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())),
			Some(access_token_2.clone())
		);
		assert_eq!(AccessToken::access_token_count(), 2);
		assert_eq!(AccessToken::access_token_index(), 2);

		let owner_2 = account_key("bob");
		let owner_origin_2 = RuntimeOrigin::signed(owner_2);

		assert_noop!(
			AccessToken::revoke_all_by_scopes(owner_origin_2, Scopes::<TimelineId>::default()),
			Error::<Test>::NotExists,
		);

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone(), access_token_2.clone()])
		);

		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);

		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash2".as_bytes())),
			Some(access_token_2)
		);

		assert_eq!(AccessToken::access_token_count(), 2);
		assert_eq!(AccessToken::access_token_index(), 2);
	})
}
