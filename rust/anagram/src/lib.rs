//TODO!
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = unstable_sort_string(word.to_lowercase().chars().collect());
    possible_anagrams
        .iter()
        .filter(|item| unstable_sort_string(item.to_lowercase().chars().collect()) == sorted_word)
        .filter(|item| item.to_lowercase() != word.to_lowercase())
        .copied()
        .collect()
}

fn unstable_sort_string(mut vec: Vec<char>) -> Vec<char> {
    vec.sort_unstable();
    vec
}

//TODO!
