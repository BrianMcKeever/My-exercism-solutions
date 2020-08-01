pub fn map<T, U, F: FnMut(T) -> U>(mut input: Vec<T>, mut function: F) -> Vec<U> {
    let mut result = Vec::with_capacity(input.len());
    for i in (0..input.len()/2) {
        let a = input.swap_remove(i);
        result.push(function(a));
    }
    while input.len() > 0 {
        let a = input.pop().unwrap();
        result.push(function(a));
    }
    result
}
