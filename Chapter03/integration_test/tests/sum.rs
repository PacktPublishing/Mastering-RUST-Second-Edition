// integration_test/tests/sum.rs

use integration_test::sum;

mod common;

use common::{setup, teardown};

#[test]
fn sum_test() { 
    assert_eq!(sum(6, 8), 14); 
}

#[test]
fn test_with_fixture() {
    setup();
    assert_eq!(sum(7, 14), 21);
    teardown();
}
