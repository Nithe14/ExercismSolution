pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut prime: u64 = 2;

    while n > 1 {
        if n % prime == 0 {
            prime_factors.push(prime);
            n /= prime;
        } else {
            match prime {
                //skiping even numbers
                2 => prime += 1,
                _ => prime += 2,
            }
        }
    }

    prime_factors
}
