//TODO!
use kmp::kmp_find;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if first_list.is_empty() && !second_list.is_empty() {
        return Comparison::Sublist;
    } else if second_list.is_empty() && !first_list.is_empty() {
        return Comparison::Superlist;
    } else if first_list.len() < second_list.len() {
        let result = kmp_find(first_list, second_list);
        match result {
            None => return Comparison::Unequal,
            Some(_) => return Comparison::Sublist,
        }
    } else if first_list.len() > second_list.len() {
        let result = kmp_find(second_list, first_list);
        match result {
            None => return Comparison::Unequal,
            Some(_) => return Comparison::Superlist,
        }
    }

    Comparison::Unequal
}
//TODO!
