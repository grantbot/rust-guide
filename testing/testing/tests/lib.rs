//Tests here are an entirely new create, so we need to import our library. We
//consume our library like any other would; this is a logical place for
//integration tests

extern crate testing; //name of project
use testing::add_three_times_four;

#[test]
fn math_checks_out() {
    let result = add_three_times_four(5i);

    assert_eq!(32i, result);
}
