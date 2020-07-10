use std::collections::HashMap;
use std::collections::HashSet;

/*
struct PermutationIterator {
    elements: Vec<char>,
    index: Vec<u8>,
}

impl PermutationIterator {
    fn new(elements: Vec<char>) -> PermutationIterator {
        PermutationIterator {index: vec![0; elements.len()], elements: elements }
    }
    fn next(&mut self) -> Option<Vec<char>> {
        unimplemented!();
    }
}
*/

fn factorial(n: usize) -> usize {
    if n == 0 { return 1; }
    (1..=n).fold(1, |a, x| a * x)
}

pub fn permutations(elements: Vec<u8>) -> Vec<Vec<u8>> {
    let len = elements.len();
    if len == 0 {
        return Vec::new();
    }
    if len == 1 {
        return vec![
            vec![elements[0]]
        ];
    }
    if len == 2 {
        return vec![
            vec![elements[0], elements[1]],
            vec![elements[1], elements[0]]
        ];
    }
    let num_permutations = factorial(len);
    let mut result: Vec<Vec<u8>> = Vec::with_capacity(num_permutations);
    let mut mutable_elements = elements.clone();
    mutable_elements.shrink_to_fit();
    inner_permutations(&mut result, &mut mutable_elements, len - 1);
    result
}

fn inner_permutations(result: &mut Vec<Vec<u8>>, elements: &mut Vec<u8>, index: usize) {
    if index == 2 {
        result.push(elements.clone());
        elements.swap(0,1);
        result.push(elements.clone());
        elements.swap(1,2);
        result.push(elements.clone());
        elements.swap(0,1);
        result.push(elements.clone());
        elements.swap(1,2);
        result.push(elements.clone());
        elements.swap(0,1);
        result.push(elements.clone());
        elements.swap(1,2);
        return;
    }
    inner_permutations(result, elements, index - 1);
    for i in 0..index {
        elements.swap(i, index);
        inner_permutations(result, elements, index - 1);
        elements.swap(i, index);
    }
}

fn translate(decoder: &HashMap<char, u8>, word: &str) -> u64 {
    let mut s = String::new();
    for letter in word.chars() {
        s.push_str(&decoder.get(&letter).unwrap().to_string());
    }
    s.parse().unwrap()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let chars = input.chars().filter(|x| !vec![' ', '+', '='].contains(x)).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
    let perms = permutations((0..=9 as u8).collect());

    let mut split =  input.split(" == ");
    let words = split.next()?;
    let result = split.next()?;
    let parsed_words = words.split(" + ").collect::<Vec<_>>();
    dbg!(&parsed_words, result);
    for perm in perms {
        let decoder: HashMap<char, u8> = chars.iter().cloned().zip(perm).collect();
        let mut skip = false;
        for word in parsed_words.iter() {
            if *decoder.get(&word.chars().collect::<Vec<_>>()[0])? == 0 {
                skip = true;
            }
        }
        if skip {
            continue;
        }
        if *decoder.get(&result.chars().collect::<Vec<_>>()[0])? == 0 {
            continue;
        }
        let mut total: u64 = 0;
        for word in parsed_words.clone() {
            total += translate(&decoder, word);
        }
        if total == translate(&decoder, result){
            return Some(decoder);
        }
    }
    None
}
