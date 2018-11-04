
extern crate edit_distance;
use edit_distance::levenshtein_safe;

fn main() {
    let a = "foo";
    let b = "fooo";
    assert_eq!(1, levenshtein_safe(a, b));
}
