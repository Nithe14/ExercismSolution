/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    if isbn.len() != 10 {
        return false;
    }

    let sum: Option<usize> = isbn.chars().enumerate().try_fold(0, |acc, (i, c)| {
        let value = match c {
            'X' if i == 9 => 10,
            _ => c.to_digit(10)? as usize, // Returns None if parsing fails
        };
        Some(acc + value * (10 - i))
    });

    sum.is_some_and(|s| s % 11 == 0)
}
