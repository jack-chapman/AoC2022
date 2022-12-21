use std::{collections::HashMap, fs::read_to_string, str::FromStr};

#[derive(Debug, Clone)]
enum Calculation {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug, Clone)]
enum Monkey {
    Plain(f64),
    Depends(Calculation),
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        if split.len() == 1 {
            let value = split.get(0).unwrap().parse::<f64>().unwrap();
            return Ok(Self::Plain(value));
        } else {
            let a = split.get(0).unwrap().to_string();
            let b = split.get(2).unwrap().to_string();
            let op = split.get(1).unwrap().to_owned();
            let calc = match op {
                "+" => Calculation::Add(a, b),
                "-" => Calculation::Sub(a, b),
                "*" => Calculation::Mul(a, b),
                "/" => Calculation::Div(a, b),
                _ => panic!("invalid operator"),
            };
            Ok(Self::Depends(calc))
        }
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let mut monkies: HashMap<String, Monkey> = HashMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(":").map(|spl| spl.trim()).collect();
        let name = split.get(0).unwrap().to_string();
        let monkey = Monkey::from_str(split.get(1).unwrap()).unwrap();
        monkies.insert(name, monkey);
    }

    let root = monkies.get(&"root".to_string()).unwrap();
    // part 1
    let result = solve(root, &monkies);
    println!("root = {}", result);

    // part 2
    let (left, right) = match root {
        Monkey::Depends(a) => match a {
            Calculation::Add(a, b) => (a, b),
            Calculation::Sub(a, b) => (a, b),
            Calculation::Mul(a, b) => (a, b),
            Calculation::Div(a, b) => (a, b),
        },
        _ => panic!("root is wrong type"),
    };

    let left = monkies[left].clone();
    let right = monkies[right].clone();

    // binary search for the correct result
    // this only works with floating point nums for some reason..
    let mut matches = false;
    let mut min = 0.;
    let mut max = 10000000000000.;
    let mut test_val = 0.;
    while !matches {
        test_val = (min + max) / 2.;
        monkies
            .get_mut(&"humn".to_string())
            .map(|m| *m = Monkey::Plain(test_val));

        let a = solve(&left, &monkies);
        let b = solve(&right, &monkies);
        if a == b {
            matches = true;
        } else {
            if a > b {
                min = test_val;
            } else {
                max = test_val;
            }
        }
    }

    println!("required num = {}", test_val);
}

fn solve(monkey: &Monkey, monkies: &HashMap<String, Monkey>) -> f64 {
    match monkey {
        Monkey::Plain(v) => *v,
        Monkey::Depends(calc) => match calc.to_owned() {
            Calculation::Add(a, b) => {
                let a = monkies.get(&a).unwrap();
                let b = monkies.get(&b).unwrap();
                let a = solve(a, monkies);
                let b = solve(b, monkies);
                a + b
            }
            Calculation::Sub(a, b) => {
                let a = monkies.get(&a).unwrap();
                let b = monkies.get(&b).unwrap();
                let a = solve(a, monkies);
                let b = solve(b, monkies);
                a - b
            }
            Calculation::Mul(a, b) => {
                let a = monkies.get(&a).unwrap();
                let b = monkies.get(&b).unwrap();
                let a = solve(a, monkies);
                let b = solve(b, monkies);
                a * b
            }
            Calculation::Div(a, b) => {
                let a = monkies.get(&a).unwrap();
                let b = monkies.get(&b).unwrap();
                let a = solve(a, monkies);
                let b = solve(b, monkies);
                a / b
            }
        },
    }
}
