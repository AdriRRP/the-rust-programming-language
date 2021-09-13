use tests;

mod common;

#[test]
fn it_adds_two_ext() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}
