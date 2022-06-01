use crate::{mock::*, Error};
use crate as pallet_simple_map;

use frame_support::{assert_err, assert_ok};

#[test]
fn set_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(SimpleMap::set_single_entry(Origin::signed(1), 19));

		let expected_event = Event::SimpleMap(pallet_simple_map::Event::EntrySet(1, 19));

		System::assert_last_event(expected_event);
	})
}

#[test]
fn get_throws() {
	new_test_ext().execute_with(|| {
		assert_err!(
			SimpleMap::get_single_entry(Origin::signed(2), 3),
			Error::<Test>::NoValueStored
		);
	})
}

#[test]
fn get_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::get_single_entry(Origin::signed(1), 2));

		let expected_event = Event::SimpleMap(pallet_simple_map::Event::EntryGot(1, 19));

		System::assert_last_event(expected_event);

		// Ensure storage is still set
		assert_eq!(SimpleMap::simple_map(2), 19);
	})
}

#[test]
fn take_throws() {
	new_test_ext().execute_with(|| {
		assert_err!(
			SimpleMap::take_single_entry(Origin::signed(2)),
			Error::<Test>::NoValueStored
		);
	})
}

#[test]
fn take_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::take_single_entry(Origin::signed(2)));

		let expected_event = Event::SimpleMap(pallet_simple_map::Event::EntryTaken(2, 19));

		System::assert_last_event(expected_event);

		// Assert storage has returned to default value (zero)
		assert_eq!(SimpleMap::simple_map(2), 0);
	})
}

#[test]
fn increase_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::increase_single_entry(Origin::signed(2), 2));

		let expected_event = Event::SimpleMap(pallet_simple_map::Event::EntryIncreased(2, 19, 21));

		System::assert_last_event(expected_event);

		// Assert storage map entry has been increased
		assert_eq!(SimpleMap::simple_map(2), 21);
	})
}
