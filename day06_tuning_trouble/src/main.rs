use std::{fs::read_to_string, collections::HashSet};

fn process_signal(signal: &str, window_size: usize) -> usize {
    let signals: Vec<char> = signal.chars().collect();
    signals.windows(window_size)
    .enumerate()
    .find(|(_, window)| window.iter().collect::<HashSet<&char>>().len() == window_size)
    .map(|(i, _)| i+window_size)
    .unwrap()
}

fn main() {
    let input = read_to_string("./input/task_1.txt").unwrap();
    let task_1_solution = process_signal(input.as_str(), 4);
    println!("[Task 1] Start of packet at: {}", task_1_solution);
    let task_2_solution = process_signal(input.as_str(), 14);
    println!("[Task 2] Start of message at: {}", task_2_solution);
}