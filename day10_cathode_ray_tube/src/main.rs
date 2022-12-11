use std::fs::read_to_string;

enum Instruction {
    Noop,
    Add(i32)
}

impl Instruction {
    fn parse(string: &str) -> Instruction {
        if string == "noop" {
            return Instruction::Noop;
        } else if string.starts_with("addx") {
            let val: i32 = string.split_once(" ").unwrap().1.parse().unwrap();
            return Instruction::Add(val);
        } else {
            panic!("Invalid instruction")
        }
    }
}

fn process(instructions: Vec<Instruction>, X: i32) -> Vec<i32>{
    let mut cycles: Vec<i32> = vec![X];
    let mut current_value = X;
    for instruction in instructions{
        if let Instruction::Noop = instruction {
            cycles.push(current_value)
        } else if let Instruction::Add(value) = instruction {
            cycles.push(current_value);
            cycles.push(current_value);
            current_value += value;
        }
    }
    return cycles;
}

fn draw(register_values: Vec<i32>) -> String {
    let mut screen = vec![["."; 40]; 6];
    for row in 0..6 {
        for col in 0..40 {
            let cur_pixel  = col as i32;
            let value = register_values[(cur_pixel+row*40+1) as usize];
            if value == cur_pixel || value + 1 == cur_pixel || value - 1 == cur_pixel {
                screen[row as usize][col] = "#";
            }
        }
    }

    let rows: Vec<String> = screen.iter().map(|s| s.to_vec().join("")).collect();
    rows.join("\n")
}

fn main() {
    let instructions: Vec<Instruction> = read_to_string("./input/task_1.txt").unwrap().lines()
    .map(|line| Instruction::parse(line))
    .collect();

    let register_values = process(instructions, 1);

    let task_1_answer: i32 = register_values.iter()
    .enumerate()
    .map(|(cycle,value)| cycle as i32 * value)
    .skip(20)
    .step_by(40)
    .sum();
    println!("[Task 1] Sum of signal strengths: {}", task_1_answer);

    println!("[Task 2]");
    let screen = draw(register_values);
    println!("{}", screen);
}