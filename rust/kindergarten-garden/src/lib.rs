fn match_user(user: &str) -> Option<usize> {
    match user {
        "Alice" => Some(0),
        "Bob" => Some(2),
        "Charlie" => Some(4),
        "David" => Some(6),
        "Eve" => Some(8),
        "Fred" => Some(10),
        "Ginny" => Some(12),
        "Harriet" => Some(14),
        "Ileana" => Some(16),
        "Joseph" => Some(18),
        "Kincaid" => Some(20),
        "Larry" => Some(22),
        _ => None,
    }
}

fn match_char(ch: char) -> Option<&'static str> {
    match ch {
        'G' => Some("grass"),
        'C' => Some("clover"),
        'R' => Some("radishes"),
        'V' => Some("violets"),
        _ => None,
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let user = match_user(student).unwrap_or_default();
    diagram
        .lines()
        .flat_map(|line| {
            line.chars()
                .skip(user)
                .take(2)
                .map(|c| match_char(c).unwrap_or_default())
        })
        .collect()
}
