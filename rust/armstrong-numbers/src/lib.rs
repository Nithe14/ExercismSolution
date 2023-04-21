pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u64> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let digits_len = digits.len();
    let sum: u64 = digits.iter().map(|d| d.pow(digits_len as u32)).sum();

    sum == num as u64
}
