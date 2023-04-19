/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.replace(" ", "").chars().any(|c| !c.is_digit(10))
        || code.replace(" ", "").chars().collect::<Vec<char>>().len() < 2
    {
        return false;
    }

    let numbers: Vec<(usize, u32)> = code
        .replace(" ", "")
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .enumerate()
        .collect();

    let sum: u32 = numbers
        .into_iter()
        .map(|(n, x)| match n {
            a if a % 2 == 0 => x,
            _ if x > 4 => 2 * x - 9,
            _ => 2 * x,
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    match sum % 10 {
        0 => true,
        _ => false,
    }
}
