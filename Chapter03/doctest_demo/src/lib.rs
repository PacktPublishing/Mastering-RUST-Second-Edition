// doctest_demo/src/lib.rs

//! This crate provides functionality for adding things
//!
//! # Examples
//! ```
//! use doctest_demo::sum;
//!
//! let work_a = 4;
//! let work_b = 34;
//! let total_work = sum(work_a, work_b);
//! ```

/// Sum two arguments
///
/// # Examples
///
/// ```
/// assert_eq!(doctest_demo::sum(1, 1), 2);
/// ```
pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}
