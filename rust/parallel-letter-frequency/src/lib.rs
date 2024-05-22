use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

//test bench_large_parallel   ... bench:     138,913 ns/iter (+/- 13,421)
//test bench_large_sequential ... bench:     214,249 ns/iter (+/- 9,443)
//I couldn't manage to make it faster for small and tiny inputs.
//I guess that sequential is just better for small texts.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let chunk_size = (input.len() / worker_count).max(1);
    let chunks = input.chunks(chunk_size).map(|c| c.join(""));
    let char_map = Arc::new(Mutex::new(HashMap::with_capacity(chunk_size)));

    thread::scope(|s| {
        for chunk in chunks {
            let char_map = Arc::clone(&char_map);
            s.spawn(move || {
                let mut map = HashMap::new();
                chunk
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .map(|c| c.to_ascii_lowercase())
                    .for_each(|c| *map.entry(c).or_insert(0) += 1);

                //lock after counting the current chunk
                let mut final_map = char_map.lock().unwrap();
                for (key, value) in map {
                    *final_map.entry(key).or_insert(0) += value;
                }
            });
        }
    });

    Arc::try_unwrap(char_map).unwrap().into_inner().unwrap()
}
