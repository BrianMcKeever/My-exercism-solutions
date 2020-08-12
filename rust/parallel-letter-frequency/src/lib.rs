use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel();
    let num_per_input = 1 + input.len() / worker_count;
    let mut input: Vec<String> = input.iter().map(|x| x.to_string()).collect();
    while !input.is_empty() {
        let partial_input = if input.len() > num_per_input {
            input.split_off(input.len() - num_per_input)
        } else {
            input.split_off(0)
        };
        let sender = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let result = frequency_list(partial_input);
            sender.send(result).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    }
    drop(tx);

    let mut result = HashMap::new();
    for received in rx {
        for (key, value) in received {
            let count = result.get(&key).copied().unwrap_or(0);
            result.insert(key, count + value);
        }
    }
    result
}

fn frequency_list(input: Vec<String>) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for s in input {
        for letter in s.to_lowercase().chars() {
            if !letter.is_alphabetic() {
                continue;
            }
            let count = result.get(&letter).copied().unwrap_or(0);
            result.insert(letter, count + 1);
        }
    }
    result
}
