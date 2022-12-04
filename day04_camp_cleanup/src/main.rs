use std::{fs::{File}, io::{self, BufRead}};

type Assignment = (i32, i32);

fn parse_line(line: &str) -> (Assignment, Assignment) {
    let assignments: Vec<Assignment> = line.split(",")
    .map(|s| s.split_once("-").unwrap())
    .map(|shift| (shift.0.parse().unwrap(), shift.1.parse().unwrap()))
    .collect();
    return (assignments[0], assignments[1]);
}

fn read_shifts(path: &str) -> Vec<(Assignment, Assignment)> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    return reader.lines()
    .map(|l| l.unwrap())
    .map(|l| parse_line(&l))
    .collect();

}

fn overlaps_completely(a: &Assignment, b: &Assignment) -> bool {
    let (a1, a2) = a;
    let (b1, b2) = b;
    a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2
}

fn overlaps(a: &Assignment, b: &Assignment) -> bool {
    let (a1, a2) = a;
    let (b1, b2) = b;
    a1 <= b1 && b1 <= a2 || b1 <= a1  && a1 <= b2
}

fn main() {
    let input = "./inputs/task_1.txt";
    let assignments = read_shifts(input);

    let overlap_count = assignments.iter()
    .filter(|(a,b)| overlaps_completely(a,b))
    .count();
    println!("[Task 1] Number of overlapping assignments: {}", overlap_count);

    let overlap_count = assignments.iter()
    .filter(|(a,b)| overlaps(a,b))
    .count();
    println!("[Task 2] Number of partially overlapping assignments: {}", overlap_count);


}
