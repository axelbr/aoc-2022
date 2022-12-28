use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Debug)]
enum Expr {
    Constant(String, i64),
    Sum(String, Box<Expr>, Box<Expr>),
    Sub(String, Box<Expr>, Box<Expr>),
    Mul(String, Box<Expr>, Box<Expr>),
    Div(String, Box<Expr>, Box<Expr>)
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Self::Constant(_, n) => *n,
            Self::Sum(_, a, b) => a.eval() + b.eval(),
            Self::Sub(_, a, b) => a.eval() - b.eval(),
            Self::Mul(_, a, b) => a.eval() * b.eval(),
            Self::Div(_, a, b) => a.eval() / b.eval()
        }
    }

    fn contains(&self, name: &str) -> bool {
        match self {
            Self::Constant(n, _) => n == name,
            Self::Sum(n, a, b) => n == name || a.contains(name) || b.contains(name),
            Self::Sub(n, a, b) => n == name || a.contains(name) || b.contains(name),
            Self::Mul(n, a, b) => n == name || a.contains(name) || b.contains(name),
            Self::Div(n, a, b) => n == name || a.contains(name) || b.contains(name)
        }
    }
}

fn build_ast(name: &str, operations: &HashMap<String, String>) -> Expr {
    let expr = &operations[name];
    if expr.trim().contains(" ") {
        let items: Vec<&str> = expr.trim().split(" ").collect();
        let a = Box::new(build_ast(items[0], operations));
        let b = Box::new(build_ast(items[2], operations));
        match items[1].trim() {
            "+" => Expr::Sum(name.to_string(), a, b),
            "-" => Expr::Sub(name.to_string(), a, b),
            "*" => Expr::Mul(name.to_string(), a, b),
            "/" => Expr::Div(name.to_string(), a, b),
            _ => panic!("invalid operation"),
        }
    } else {
        Expr::Constant(name.to_string(), expr.trim().parse().unwrap())
    }
}

fn parse(path: &str) -> HashMap<String, String> {
    let mut expressions = HashMap::new();
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .for_each(|(k, v)| {
            expressions.insert(k.to_string(), v.to_string());
        });
    expressions
}

fn solve_for(name: &str, lhs: &Expr, rhs: &Expr) -> Expr {
    let (lhs, rhs) = match lhs {
        Expr::Constant(_, _) => {
            return rhs.clone();
        },
        Expr::Sum(e, a, b) => {
            if a.contains(name) {
                (a.as_ref().clone(), Expr::Sub(format!("inv_{}", e), Box::new(rhs.clone()), b.clone()))
            } else {
                (b.as_ref().clone(), Expr::Sub(format!("inv_{}", e), Box::new(rhs.clone()), a.clone()))
            }
        },
        Expr::Sub(e, a, b) => {
            if a.contains(name) {
                (a.as_ref().clone(), Expr::Sum(format!("inv_{}", e), Box::new(rhs.clone()), b.clone()))
            } else {
                (b.as_ref().clone(), Expr::Sub(format!("inv_{}", e), a.clone(), Box::new(rhs.clone())))
            }
        },
        Expr::Mul(e, a, b) => {
            if a.contains(name) {
                (a.as_ref().clone(), Expr::Div(format!("inv_{}", e), Box::new(rhs.clone()), b.clone()))
            } else {
                (b.as_ref().clone(), Expr::Div(format!("inv_{}", e), Box::new(rhs.clone()),  a.clone()))
            }
        },
        Expr::Div(e, a, b) => {
            if a.contains(name) {
                (a.as_ref().clone(), Expr::Mul(format!("inv_{}", e), Box::new(rhs.clone()), b.clone()))
            } else {
                (b.as_ref().clone(), Expr::Div(format!("inv_{}", e), a.clone(), Box::new(rhs.clone())))
            }
        }
    };
    solve_for(name, &lhs, &rhs)   
}

fn main() {
    let operations = parse("./input/task_1.txt");
    let ast = build_ast("root", &operations);
    let task_1_answer = ast.eval();
    println!("[Task 1] Root yells: {}", task_1_answer);

    let (a,b) = operations["root"].split_once(" + ").unwrap();
    let lhs = build_ast(a, &operations);
    let rhs = build_ast(b, &operations);
    let solution = solve_for("humn", &lhs, &rhs);
    let task_2_answer = solution.eval();
    println!("[Task 2] I have to yell {}", task_2_answer);
}