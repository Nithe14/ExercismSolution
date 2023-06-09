pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64");
    }
    u64::pow(2, s - 1) //nth square formula
}

pub fn total() -> u64 {
    u64::max_value() //the correct formula is (2^number_of_squares - 1) but this is the same for a chessboard
}
