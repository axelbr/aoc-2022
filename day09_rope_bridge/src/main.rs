use std::{fs::read_to_string, collections::HashSet};

type Position = (i32, i32);

enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn parse(dir: &str) -> Direction {
        match dir {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Invalid direction")
        }
    }
}

type State = Vec<Position>;

fn apply(state: &State, direction: &Direction) -> State {
    let (hx, hy) = state[0];
    let new_head = match direction {
        Direction::Up => (hx+1, hy),
        Direction::Right => (hx, hy+1),
        Direction::Down => (hx-1, hy),
        Direction::Left => (hx, hy-1)
    };

    let mut new_state = vec![new_head];

    for knot in state[1..state.len()].iter() {
        let prev_knot = new_state.last().unwrap();
        let dx = prev_knot.0 - knot.0;
        let dy = prev_knot.1 - knot.1;
        let dknot = match (dx.abs(), dy.abs()) {
            (0,0) | (1,0) | (0,1) | (1,1) => (0, 0),
            _ => (dx.signum(), dy.signum())
        };
        new_state.push((knot.0 + dknot.0, knot.1 + dknot.1));
    }
    new_state
}

fn run_system(x0: State, commands: &Vec<(Direction, usize)>) -> Vec<State> {
    let mut trajectory: Vec<State> = vec![x0];
    for (direction, n) in commands {
        for _ in 0..*n {
            let x = apply(trajectory.last().unwrap(), direction);
            trajectory.push(x);
        }
    }
    trajectory
}

fn parse_input(path: &str) -> Vec<(Direction, usize)> {
    let input = read_to_string(path).unwrap();
    input.lines()
    .map(|l| l.split_once(" ").unwrap())
    .map(|(d, n)| (Direction::parse(d), n.parse().unwrap()))
    .collect()
}

fn main() {
    let commands = parse_input("./input/task_1.txt");
    let initial_state = vec![(0,0); 2];
    let trajectory = run_system(initial_state, &commands);
    let task_1_answer = trajectory.iter()
    .map(|s| s[1])
    .collect::<HashSet<Position>>()
    .len();
    println!("[Task 1] Number of unique tail positions: {}", task_1_answer);

    let trajectory = run_system(vec![(0,0); 10], &commands);
    let task_2_answer = trajectory.iter()
    .map(|s| *s.last().unwrap())
    .collect::<HashSet<Position>>()
    .len();
    println!("[Task 2] Number of unique tail positions: {}", task_2_answer);
    //visualize(&trajectory)
}
