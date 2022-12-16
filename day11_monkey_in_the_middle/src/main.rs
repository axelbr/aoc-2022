use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
enum Operation {
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand)
}

#[derive(Clone)]
enum Operand {
    Constant(u64),
    Old
}

impl Operand {
    fn parse(string: &str) -> Operand {
        match string {
            "old" => Self::Old,
            a => Self::Constant(a.parse().unwrap())
        }
    }
}

impl Operation {
    fn get_result(&self, old: u64) -> u64 {
        let get_value = |op: &Operand| {
            match op {
                Operand::Old => old,
                Operand::Constant(v) => *v
            }
        };

        match self {
            Self::Add(a,b) => get_value(a) + get_value(b),
            Self::Sub(a,b) => get_value(a) - get_value(b),
            Self::Mul(a, b) => get_value(a) * get_value(b)
        }
    }

    fn parse(expression: &str) -> Operation {
        let equation: Vec<&str>  = expression.trim().split(" ").collect();
        assert!(equation.len() == 3);
        let op1 = Operand::parse(equation[0]);
        let op2 = Operand::parse(equation[2]);
        match equation[1] {
            "+" => Operation::Add(op1, op2),
            "-" => Operation::Sub(op1, op2),
            "*" => Operation::Mul(op1, op2),
            _ => panic!("invalid operation")
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    decision: HashMap<bool, usize>,
    inspections: u64
}

impl Monkey {

    fn parse(string: &str) -> Monkey {
        let lines: Vec<&str> = string.lines().map(|l| l.trim_start()).collect();
        let mut items: Vec<u64> = lines[1].split(":").last().unwrap()
        .split(",")
        .map(|item| item.trim().parse().unwrap())
        .collect();
        items.reverse();
        let operation = Operation::parse(lines[2].split("=").last().unwrap());
        let test: u64 = lines[3].split(" ").last().unwrap().parse().unwrap();
        let mut decision: HashMap<bool, usize> = HashMap::new();
        let if_true: usize = lines[4].split(" ").last().unwrap().parse().unwrap();
        let if_false: usize = lines[5].split(" ").last().unwrap().parse().unwrap();
        decision.insert(true, if_true);
        decision.insert(false, if_false);
        Monkey { items, operation, test, decision, inspections: 0}
    }
}

fn parse_monkeys(path: &str) -> Vec<Monkey> {
    let input = read_to_string(path).unwrap();
    let mut monkeys = Vec::new();
    for monkey_info in input.split("\n\n") {
        let monkey = Monkey::parse(monkey_info);
        monkeys.push( monkey);
    }
    monkeys

}


fn run_n_rounds(monkeys: Vec<Monkey>, n: usize, divide_by_three: bool) -> Vec<Monkey> {
    let mut monkeys = monkeys.clone();
    let common_multiple: u64 = monkeys.iter().map(|m| m.test).product();
    for _ in 0..n {
        for m in 0..monkeys.len() {
            let mut items: Vec<Vec<u64>> = (0..monkeys.len()).map(|_| Vec::new()).collect();
            let monkey = &mut monkeys[m];
            while let Some(item) = monkey.items.pop() {
                let mut level = monkey.operation.get_result(item);
                if divide_by_three {
                    level = level / 3;
                } else {
                    level = level % common_multiple;
                }
                let test = level % monkey.test == 0;
                let decision = monkey.decision.get(&test).unwrap();
                items[*decision].insert(0, level);
                monkey.inspections += 1;
            }
            for (i, list) in items.iter().enumerate() {
                let monkey = &mut monkeys[i];
                for elem in list {
                    monkey.items.insert(0, *elem);
                }
            }
        }
    }
    monkeys
}

fn main() {
    let mut monkeys = parse_monkeys("./input/task_1.txt");
    monkeys = run_n_rounds(monkeys, 20, true);
    monkeys.sort_by(|a,b| b.inspections.cmp(&a.inspections));
    let task_1_answer: u64 = monkeys.iter().take(2).map(|m| m.inspections).product();
    println!("[Task 1] Monkey business of top 2 monkeys after 20 rounds: {}", task_1_answer);
    let mut monkeys = parse_monkeys("./input/task_1.txt");
    monkeys = run_n_rounds(monkeys, 10000, false);
    monkeys.sort_by(|a,b| b.inspections.cmp(&a.inspections));
    let task_2_answer: u64 = monkeys.iter().take(2).map(|m| m.inspections).product();
    println!("[Task 2] Monkey business of top 2 monkeys after 10000 rounds: {}", task_2_answer);


}
