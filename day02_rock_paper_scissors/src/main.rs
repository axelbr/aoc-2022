use std::{fs::File, io::{self, BufRead}};


enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn from(code: &str) -> Move {
        match code {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid code.")
        }
    }

    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }

    fn play(&self, other: &Move) -> Outcome {
        match (&self, other) {
                (Move::Rock, Move::Paper) => Outcome::Loose,
                (Move::Rock, Move::Scissors) => Outcome::Win,
                (Move::Paper, Move::Rock) => Outcome::Win,
                (Move::Paper, Move::Scissors) => Outcome::Loose,
                (Move::Scissors, Move::Rock) => Outcome::Loose,
                (Move::Scissors, Move::Paper) => Outcome::Win,
                _ => Outcome::Draw
        }   
    }

    fn get_move_for(&self, outcome: &Outcome) -> Move {
        match &self {
            Move::Rock => match outcome {
                Outcome::Loose => Move::Scissors,
                Outcome::Draw => Move::Rock,
                Outcome::Win => Move::Paper
            },
            Move::Paper => match outcome {
                Outcome::Loose => Move::Rock,
                Outcome::Draw => Move::Paper,
                Outcome::Win => Move::Scissors
            },
            Move::Scissors => match outcome {
                Outcome::Loose => Move::Paper,
                Outcome::Draw => Move::Scissors,
                Outcome::Win => Move::Rock
            },
        }
    }
}



enum Outcome {
    Win,
    Draw,
    Loose
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loose => 0
        }
    }

    fn from(str: &str) -> Outcome {
        match str {
            "X" => Outcome::Loose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid character.")
        }
    }
}


fn read_input(path: &str) -> Vec<(String, String)> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    return reader.lines()
    .map(|result| result.unwrap())
    .map(|line: String| {
        let mut split = line.split_whitespace();
        (String::from(split.next().unwrap()), String::from(split.next().unwrap()))
    })
    .collect();
}

fn main() {
    let input = "inputs/task_1.txt";
    let inputs = read_input(input);
    // Task 1
    let task1_score : i32 = inputs.iter()
    .map(|cols| (Move::from(&cols.0), Move::from(&cols.1)))
    .map(|moves| moves.1.score() + moves.1.play(&moves.0).score())
    .sum();

    println!("[Task 1] Total score: {}", task1_score);

    // Task 2
    let task2_score: i32 = inputs.iter()
    .map(|cols| {
        let opp_move = Move::from(&cols.0);
        let desired_outcome = Outcome::from(&cols.1);
        let own_move = opp_move.get_move_for(&desired_outcome);
        return own_move.score() + desired_outcome.score();
    })
    .sum();

    println!("[Task 2] Total score: {}", task2_score);

}
