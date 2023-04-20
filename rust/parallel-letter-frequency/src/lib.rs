//TODO!
//not multiworkers
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut char_map = HashMap::new();

    input.iter().for_each(|line| {
        line.chars().for_each(|c| {
            if c.is_alphabetic() {
                let lower_c = c.to_lowercase().to_string().chars().nth(0).unwrap();
                *char_map.entry(lower_c).or_insert(0) += 1;
            }
        });
    });

    char_map

    //unimplemented!(
    //"Count the frequency of letters in the given input '{input:?}'. Ensure that you are using {} to process the input.",
    //match worker_count {
    //1 => "1 worker".to_string(),
    //_ => format!("{worker_count} workers"),
    //}
    //);
}
//TODO!
