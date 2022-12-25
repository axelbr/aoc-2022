use std::{fs::read_to_string};

fn parse_input(path: &str) -> Vec<i64> {
    let input = read_to_string(path).unwrap();
    input.lines().map(|n| n.parse().unwrap()).collect()
}

fn solve(numbers: &Vec<i64>, n_times: usize, decryption_key: i64) -> Vec<i64> {
    let mut positions: Vec<usize> = (0..numbers.len()).collect();
    let numbers: Vec<i64> = numbers.iter().map(|&n| n * decryption_key).collect();
    for _ in 0..n_times {
        for i in 0..numbers.len() {
            let val = numbers[i];
            let cur_pos = positions.iter().position(|&j| j == i).unwrap();
            let mut new_pos = (cur_pos as i64 + val) % (positions.len() as i64 - 1);
            if new_pos < 0 {
                new_pos = (positions.len() as i64 - 1) + new_pos;
            }
            positions.remove(cur_pos);
            positions.insert(new_pos as usize, i);
        }
    }
    positions.iter().map(|&i| numbers[i as usize]).collect()
}

fn compute_grove_coordinates(mixed: &Vec<i64>) -> i64 {
    let zero_index = mixed.iter().position(|&i| i == 0).unwrap();
    mixed[(zero_index + 1000) % mixed.len()]
        + mixed[(zero_index + 2000) % mixed.len()]
        + mixed[(zero_index + 3000) % mixed.len()]
}

fn main() {
    let input = parse_input("./input/task_1.txt");
    let decrypted = solve(&input, 1, 1);
    let task_1_answer = compute_grove_coordinates(&decrypted);
    println!("[Task 1] Sum of grove coordinates: {}", task_1_answer);

    let decrypted = solve(&input, 10, 811589153);
    let task_1_answer = compute_grove_coordinates(&decrypted);
    println!("[Task 2] Sum of grove coordinates: {}", task_1_answer);
}