pub fn egg_count(display_value: u32) -> usize {
    let mut binary_count = 0;
    let mut tmp = display_value;

    while tmp != 0 {
        binary_count += tmp % 2;
        tmp /= 2;
    }

    binary_count as usize
}
