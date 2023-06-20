#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // handle an error
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    } else if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    } else if number.iter().any(|n| n >= &from_base) {
        return Err(Error::InvalidDigit(from_base));
    }

    let mut final_vec = Vec::new();

    let mut decimal: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(n, number)| number * from_base.pow(n as u32))
        .sum();

    loop {
        final_vec.push(decimal % to_base);
        decimal = decimal / to_base;
        if decimal == 0 {
            break;
        }
    }
    final_vec.reverse(); //sort MSD

    Ok(final_vec)
}
