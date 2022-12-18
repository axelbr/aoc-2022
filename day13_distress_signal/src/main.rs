use std::{fs::read_to_string, cmp::Ordering};

#[derive(Clone, Debug, PartialEq)]
enum Signal {
    List(Vec<Signal>),
    Number(i32),
}

impl Signal {
    fn parse(line: &str) -> Signal {
        let mut stack: Vec<Signal> = Vec::new();
        let mut current_list = Vec::new();
        let mut number = String::new();
        for c in line.chars() {
            if c == '[' {
                stack.push(Signal::List(current_list.clone()));
                current_list.clear();
            } else if c == ']' {
                if !number.is_empty() {
                    let value: i32 = number.parse().unwrap();
                    current_list.push(Signal::Number(value));
                    number.clear();
                }
                if let Signal::List(list) = stack.pop().unwrap() {
                    let mut new_list = list.clone();
                    let complete_list = Signal::List(current_list.clone());
                    new_list.push(complete_list);
                    current_list = new_list.clone();
                }
            } else if c == ',' {
                if !number.is_empty() {
                    let value: i32 = number.parse().unwrap();
                    number.clear();
                    current_list.push(Signal::Number(value));
                }
                
            } else {
                number.push(c);
            }
        }
       current_list[0].clone()
    }
}

fn parse_input(path: &str) -> Vec<(Signal, Signal)> {
    let input = read_to_string(path).unwrap();
    let mut signals = Vec::new();
    for pair in input.split("\n\n") {
        let (a,b) = pair.split_once("\n").unwrap();
        signals.push((Signal::parse(a), Signal::parse(b)));
    }
    signals
}

fn check_order(a: &Signal, b: &Signal) -> Option<bool> {
    match (a,b) {
        (Signal::List(l1), Signal::List(l2)) => {
            let mut result = Option::None;
            let mut i = 0;
            while result.is_none() {
                if i < l1.len() && i < l2.len(){
                    result = check_order(&l1[i], &l2[i]);
                } else if i == l1.len() && i < l2.len() {
                    return Some(true);
                } else if i < l1.len() && i == l2.len() {
                    return Some(false);
                } else {
                    return None;
                }
                i += 1
            }
            result
        },
        (Signal::Number(n1), Signal::Number(n2)) => {
            if n1 != n2 {
                Option::Some(n1 < n2)
            } else {
                Option::None
            }
        },
        (Signal::Number(_), Signal::List(_)) => {
            let list = [a.clone()].to_vec();
            check_order(&Signal::List(list), b)
        },
        (Signal::List(_), Signal::Number(_)) => {
            let list = [b.clone()].to_vec();
            check_order(a, &Signal::List(list))
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let result = check_order(&self, &other);
        if let Some(v) = result {
            if v {
                Some(Ordering::Less) 
            } else {
                Some(Ordering::Greater)
            }
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn main() {
    let input = parse_input("./input/task_1.txt");
    let task_1_answer: usize = input.iter()
    .enumerate()
    .filter(|&(_, pair)| check_order(&pair.0, &pair.1).unwrap())
    .map(|(i, _)| i + 1)
    .sum();

    println!("[Task 1] Sum of indices: {:?}", task_1_answer);

    let mut signals: Vec<Signal> = input.iter()
    .flat_map(|t| [t.0.clone(), t.1.clone()].to_vec())
    .collect();

    signals.push(Signal::parse("[[2]]"));
    signals.push(Signal::parse("[[6]]"));
    signals.sort_by(|a,b| a.partial_cmp(b).unwrap());

    let idx_1: usize = signals.iter()
    .enumerate()
    .find(|&(_, s)| *s == Signal::parse("[[2]]"))
    .unwrap().0 + 1;

    let idx_2 = signals.iter()
    .enumerate()
    .find(|&(_, s)| *s == Signal::parse("[[6]]"))
    .unwrap().0 + 1;

    println!("[Task 2] Decoder key: {}", idx_1 * idx_2);

}
