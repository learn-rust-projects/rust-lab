mod common;
use basic_test::basic_01::add_two;

#[test]
fn test_assert_eq() {
    common::setup();
    assert_eq!(4, add_two(2));
}
