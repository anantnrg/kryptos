pub mod substitution;

pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

use std::collections::HashSet;

/// Helpers
pub fn remove_duplicate_chars(s: &str) -> String {
    let mut seen = HashSet::new();
    let mut result = String::new();

    for c in s.chars() {
        if seen.insert(c) {
            result.push(c);
        }
    }
    result
}
