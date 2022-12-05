use std::{fs::read_to_string};
use regex::Regex;

type Stack<T> = Vec<T>;

struct Command {
    amount: i32,
    from: i32,
    to: i32,
    version: String
}

fn parse_commands(input: &str, version: &str) -> Vec<Command> {
    input.lines()
    .map(|line| line.split(" ").collect::<Vec<&str>>())
    .map(|line| Command {
        amount: line[1].parse().unwrap(),
        from: line[3].parse::<i32>().unwrap() - 1,
        to: line[5].parse::<i32>().unwrap() - 1,
        version: String::from(version)
    })
    .collect()
}

fn parse_stacks(input: &str) -> Vec<Stack<char>> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let num_stacks = re.find_iter(input.lines().last().unwrap()).count();
    let mut stacks: Vec<Stack<char>> =  (0..num_stacks).map(|_| Vec::new()).collect();
    let max_size = input.lines().count() - 1;
    for line in input.lines().take(max_size) {
       let row: Vec<String> = line.chars().collect::<Vec<char>>()
       .chunks(4)
       .map(|c| c.iter().collect::<String>())
       .collect();
       for (i, elem) in row.iter().enumerate() {
        if !elem.replace(" ", "").is_empty() {
            let item = elem
            .replace("[", "")
            .replace("]", "")
            .replace(" ", "")
            .chars().next().unwrap();
            stacks[i].insert(0, item);
        }
       }
    }
    stacks
}

fn parse_input(path: &str, version: &str) -> (Vec<Stack<char>>, Vec<Command>) {
    let input = read_to_string(path).unwrap();
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks);
    let commands = parse_commands(moves, version);
    (stacks, commands)
}

fn apply(commands: &Vec<Command>, stacks: &mut Vec<Stack<char>>) -> Vec<Stack<char>> {
    for command in commands {
        let mut load: Vec<char> = Vec::new();
        for _ in 0..command.amount {
            let item = stacks[command.from as usize].pop().unwrap();
            match command.version.as_str() {
                "9000" => load.push(item),
                "9001" => load.insert(0, item),
                _ => panic!("Invalid version")
            }
        }
        stacks[command.to as usize].append(&mut load);
    }
    stacks.clone()
}

fn main() {
    let input = "./inputs/task_1.txt";

    let (mut stacks, commands) = parse_input(input, "9000");
    let task_1: String = apply(&commands, &mut stacks).iter()
    .map(|s| s.last().unwrap())
    .collect();
    println!("[Task 1] Top elements: {}", task_1);

    let (mut stacks, commands) = parse_input(input, "9001");
    let task_2: String = apply(&commands, &mut stacks).iter()
    .map(|s| s.last().unwrap())
    .collect();
    println!("[Task 2] Top elements: {}", task_2);
}
