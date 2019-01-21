// unit_test_demo/tests/sum.rs

extern crate unit_test_demo; 

mod common;
mod shared;

use unit_test_demo::sum;
use shared::teardown;
use common::setup;

#[test] 
fn test_sum_integration() {
    setup();
    assert_eq!(sum(6, 8), 14); 
    teardown();
}