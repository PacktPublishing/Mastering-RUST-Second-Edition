// unit_test_demo/tests/sum.rs

extern crate unit_test_demo;

use unit_test_demo::sum;

#[test]
fn test_sum_integration() {
    assert_eq!(sum(6, 8), 14);
}
