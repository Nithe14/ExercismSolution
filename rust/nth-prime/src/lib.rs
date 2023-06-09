fn is_prime(n: u32) -> bool {
    for x in 2..=(n as f64).sqrt() as u32 {
        if n % x == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut counter = 1;
    let mut prime = 3;

    loop {
        if counter == n {
            return prime;
        }
        prime += 2; //skiping even numbers
        if is_prime(prime) {
            counter += 1;
        }
    }
}
