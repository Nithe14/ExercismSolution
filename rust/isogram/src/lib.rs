use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let candidate = candidate
        .replace("-", "")
        .replace(" ", "")
        .to_ascii_lowercase();

    candidate.chars().collect::<HashSet<_>>().len() == candidate.len()
}
