use itertools::Itertools;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .flat_map(|f| (1..=limit).map(|i| *f * i).take_while(|m| m < &limit))
        .unique()
        .sum::<u32>()
}
