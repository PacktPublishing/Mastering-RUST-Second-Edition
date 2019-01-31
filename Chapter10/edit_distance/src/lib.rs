// edit_distance/src/lib.rs

mod bindings;

use crate::bindings::levenshtein;
use std::ffi::CString;

pub fn levenshtein_safe(a: &str, b: &str) -> u32 {
    let a = CString::new(a).unwrap();
    let b = CString::new(b).unwrap();
    let distance = unsafe { levenshtein(a.as_ptr(), b.as_ptr()) };
    distance
}

#[cfg(test)]
mod tests {
    use levenshtein_safe;
    #[test]
    fn test_levenshtein_safe_wrapper() {
        let a = "foo";
        let b = "fooo";
        assert_eq!(1,levenshtein_safe(a, b));
    }
}
