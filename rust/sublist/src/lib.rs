use kmp::kmp_find;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (a, b) if a == b => Comparison::Equal,
        (a, _) if a.is_empty() => Comparison::Sublist,
        (_, b) if b.is_empty() => Comparison::Superlist,
        (a, b) if a.len() < b.len() && kmp_find(a, b).is_some() => Comparison::Sublist,
        (a, b) if a.len() > b.len() && kmp_find(b, a).is_some() => Comparison::Superlist,
        (_, _) => Comparison::Unequal,
    }
}
