use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();
    possible_anagrams
        .iter()
        .filter(|item| {
            let mut sorted_item = item.to_lowercase().chars().collect::<Vec<char>>();
            sorted_item.sort_unstable();
            sorted_word == sorted_item && word.to_lowercase() != item.to_lowercase()
        })
        .copied()
        .collect()
}
