// logic_gates/tests/half_adder.rs

use logic_gates::{and, xor};

pub type Sum = u8;
pub type Carry = u8;

pub fn half_adder_input_output() -> Vec<((u8, u8), (Sum, Carry))> { 
    vec![
        ((0, 0), (0, 0)), 
        ((0, 1), (1, 0)), 
        ((1, 0), (1, 0)), 
        ((1, 1), (0, 1)), 
    ] 
}

/// This function implements a half adder using primitive gates
fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
fn one_bit_adder() {
    for (inn, out) in half_adder_input_output() {
        let (a, b) = inn;
        assert_eq!(half_adder(a, b), out);
    }
}