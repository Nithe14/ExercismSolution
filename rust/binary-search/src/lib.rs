pub fn find<V: AsRef<[I]>, I: Ord>(array: V, key: I) -> Option<usize> {
    let array: &[I] = array.as_ref();
    let middle = match array.len() {
        0 => 0,
        2 => 1,
        n if n % 2 == 0 => n / 2 - 1,
        _ => array.len() / 2,
    };
    if !array.contains(&key) {
        return None;
    }
    match array.get(middle) {
        Some(n) if n == &key => Some(middle),
        Some(n) if n > &key => find(&array[0..middle], key),
        Some(n) if n < &key => Some(middle + find(&array[middle..], key).unwrap()),
        _ => None,
    }
}
