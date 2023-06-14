fn oposite(c: char) -> Option<&'static char> {
    match c {
        ']' => Some(&'['),
        '}' => Some(&'{'),
        ')' => Some(&'('),
        _ => None,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let supporting_vec = vec!['[', ']', '{', '}', '(', ')'];
    let filtered = string
        .chars()
        .filter(|c| supporting_vec.contains(c))
        .collect::<String>();

    let mut stack: Vec<char> = Vec::new();
    for c in filtered.chars() {
        if stack.last() != oposite(c) || stack.is_empty() {
            stack.push(c);
        }
        if stack.last() == oposite(c) {
            stack.pop();
        }
    }
    stack.is_empty()
}
