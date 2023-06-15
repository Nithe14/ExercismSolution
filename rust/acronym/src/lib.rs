fn is_word_uppercase(word: &str) -> bool {
    word.chars().all(|c| c.is_ascii_uppercase())
}
fn is_word_lowercase(word: &str) -> bool {
    word.chars().all(|c| c.is_ascii_lowercase())
}

fn title_formatting(word: &str) -> String {
    let mut string = String::new();
    if let Some(c) = word.chars().next() {
        string.push_str(&c.to_ascii_uppercase().to_string());
        string.push_str(&word[1..].to_ascii_lowercase());
    }
    string
}

pub fn abbreviate(phrase: &str) -> String {
    let words: Vec<&str> = phrase
        .split(|c: char| c == ' ' || !c.is_alphabetic() && c != '\'')
        .collect();
    let mut acronym = String::new();

    for word in words {
        let mut corrected = String::from(word);
        if is_word_uppercase(word) || is_word_lowercase(word) {
            corrected = title_formatting(&corrected);
        }
        acronym.extend(corrected.chars().filter(|c| c.is_ascii_uppercase()));
    }

    acronym
}
