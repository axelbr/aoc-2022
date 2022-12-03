use std::{fs::File, io::{self, BufRead}, collections::HashSet};

fn read_input(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    return reader.lines().map(|l| l.unwrap()).collect();
}

fn get_common_item(rucksack: &String) -> Option<char> {
    let compartments = rucksack.split_at(rucksack.len() / 2);
    let c1: HashSet<char> = compartments.0.chars().collect();
    let c2: HashSet<char> = compartments.1.chars().collect();
    let mut common = c1.intersection(&c2);
    if let Some(elem) = common.next() {
        return Option::Some(elem.clone());
    }
    return Option::None;
    
}

fn get_badge(group: &Vec<String>) -> char {
    let intersection: HashSet<char> = group.iter()
    .map(|e| e.chars().collect::<HashSet<char>>()) // map to hashsets of chars
    .reduce(|a,b| {
        let int = a.intersection(&b); // compute intersection of all
        return int.map(|&c| char::from(c)).collect();
    }).unwrap(); // Get resulting hashset
    return *intersection.iter().next().unwrap(); // retrieve (hopefully) the single left item
}

fn score(item: &char) -> i32 {
    let mut priority_list: Vec<char> = ('a'..='z').collect();
    priority_list.append(&mut('A'..='Z').collect::<Vec<char>>());
    return (priority_list.iter().position(|e| e == item).unwrap() + 1) as i32;
}


fn main() {
    let rucksacks = read_input("./inputs/task_1.txt");
    let task1_sum: i32 = rucksacks.iter()
    .map(get_common_item)
    .map(|e| score(&e.unwrap()))
    .sum();
    println!("[Task 1] Priority sum: {}", task1_sum);

    let task2_sum: i32 = rucksacks
    .chunks(3) // Iterate over groups of 3
    .map(|c| get_badge(&c.to_vec())) // retrieve badges
    .map(|e| score(&e)) // compute score of each badge
    .sum(); // compute sum
    println!("[Task 2] Priority sum: {}", task2_sum);
}