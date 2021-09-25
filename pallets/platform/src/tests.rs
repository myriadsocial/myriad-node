use crate::{
	mock::{Event, *},
	Error,
};
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_platform_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Platform::add_platform(Origin::root(), String::from("twitter").into_bytes()));
		assert_eq!(Platform::platforms(), Some(vec![String::from("twitter").into_bytes()]))
	})
}

#[test]
fn cant_add_platform_when_already_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Platform::add_platform(Origin::root(), String::from("twitter").into_bytes()));
		assert_noop!(
			Platform::add_platform(Origin::root(), String::from("twitter").into_bytes()),
			Error::<Test>::PlatformExist
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Platform::add_platform(Origin::root(), String::from("twitter").into_bytes()));
		System::assert_last_event(Event::Platform(crate::Event::PlatformAdded(
			String::from("twitter").into_bytes(),
		)))
	})
}
