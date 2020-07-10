use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    } else if array.len() == 1 {
        if array[0] == key {
            return Some(0);
        } else {
            return None;
        }
    }
    let midpoint = array.len() / 2 as usize;
    match array[midpoint].cmp(&key) {
        Ordering::Greater => find(&array[..midpoint], key),
        Ordering::Equal => Some(midpoint),
        Ordering::Less => find(&array[midpoint..], key).map(|x| x + midpoint),
    }
}
