pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        n => digits
            .chars()
            .collect::<Vec<char>>() //split to chars
            .windows(n) //create windows
            .map(|w| w.iter().collect::<String>()) //collect eery window to string
            .collect::<Vec<String>>(),
    }
}
