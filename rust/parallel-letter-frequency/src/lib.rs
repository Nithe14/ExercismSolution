//TODO!
//parallel but slower then seq xD
use std::cmp::max;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut char_map = HashMap::new();
    if input.is_empty() {
        return char_map;
    }

    let chars: Vec<char> = input.join("").chars().collect::<Vec<char>>();
    let chunk_size = max(chars.len() / worker_count, 1);
    let chunks: Vec<Vec<char>> = chars
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut handles = Vec::with_capacity(chunks.len());

    for chunk in chunks {
        let handle = thread::spawn(move || {
            let map = count_in_chunk(chunk);
            map
        });
        handles.push(handle);
    }

    for handle in handles {
        let map = handle.join().unwrap();
        for (k, v) in map {
            *char_map.entry(k).or_insert(0) += v
        }
    }

    char_map
}

fn count_in_chunk(input: Vec<char>) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    input.iter().for_each(|c| {
        if c.is_alphabetic() {
            let lower_c = c.to_lowercase().to_string().chars().nth(0).unwrap();
            *map.entry(lower_c).or_insert(0) += 1;
        }
    });
    map
}

//TODO!
