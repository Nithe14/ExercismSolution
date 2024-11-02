use std::{collections::BTreeMap, sync::LazyLock, sync::Mutex};

//With memoization
static MEMORY: LazyLock<Mutex<BTreeMap<u64, u64>>> = LazyLock::new(|| Mutex::new(BTreeMap::new()));

/*
 * if n % 2 == 0 -> n = n/2
 * else n = 3n + 1
 * till n == 1
 */
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut ret = n;
    let mut count = 0;
    let mut mem = MEMORY.lock().unwrap();

    while ret > 1 {
        if let Some(value) = mem.get(&ret) {
            count += *value;
            break;
        }

        ret = if ret % 2 == 0 { ret / 2 } else { 3 * ret + 1 };
        count += 1;
    }
    Some(*mem.entry(n).or_insert(count))
}
