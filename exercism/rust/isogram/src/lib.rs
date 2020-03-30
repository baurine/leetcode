use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    for c in candidate.to_ascii_lowercase().chars() {
        if c == ' ' || c == '-' {
            continue;
        }
        if letters.contains(&c) {
            return false;
        }
        letters.insert(c);
    }
    true
}
