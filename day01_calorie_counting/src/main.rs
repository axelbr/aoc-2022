use std::env;
use std::fs::File;
use std::io::{self, BufRead};


fn read_backpacks(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut backpack: Vec<i32> = Vec::new();
    let mut calories = 0;
    for line in reader.lines() {
        if let Ok(string) = line {
            if string.is_empty() {
                backpack.push(calories);
                calories = 0;
            } else {
                calories += string.parse::<i32>().unwrap();
            }
        } else {
            panic!("Could not read lines.")
        }
    }
    return backpack;
}

fn main() {
    let input = &env::args().collect::<Vec<String>> ()[1];
    let mut backpacks = read_backpacks(&input);
    
    backpacks.sort_by(|a, b| b.cmp(a));
    println!("[Task 1] Sum of most calories: {}", backpacks[0]);

    let top_3_sum: i32 = backpacks.iter()
    .take(3)
    .sum();
    
    println!("[Task 2] Sum of top 3 elves: {}", top_3_sum);


}

