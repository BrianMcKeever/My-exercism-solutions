#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }
    let first;
    let second;
    if first_list.len() >= second_list.len() {
        first = first_list;
        second = second_list;
    } else {
        second = first_list;
        first = second_list;
    }
    for i in 0..first.len() {
        let mut potential_match = true;
        for (j, m) in second.iter().enumerate() {
            match first.get(i + j) {
                Some(t) if t == m => (),
                _ => {
                    potential_match = false;
                    break;
                }
            }
        }
        if potential_match {
            if first.len() == second.len() {
                return Comparison::Equal;
            }
            if first == first_list {
                return Comparison::Superlist;
            } else {
                return Comparison::Sublist;
            }
        }
    }
    Comparison::Unequal
}
