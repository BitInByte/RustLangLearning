// Each test in the tests directory is a separate crate,
// so, we need to bring our library into each test crat's scope
// We don't need to annotate any code in tesrts/integration_test.rs
// with #[cfg)tests]. Cargo trats the tests directory specially and
// compiles files in this directory only when we run cargo test
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
