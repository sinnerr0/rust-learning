mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_::add(2, 2));
}
