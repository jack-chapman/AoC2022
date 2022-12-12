use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum OperationTarget {
    Value(usize),
    Itself,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    MultiplyBy(OperationTarget),
    AddTo(OperationTarget),
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divide_by: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    inspection_count: usize,
    starting_items: VecDeque<usize>,
    operation: Operation,
    test: Test,
}

impl From<&[&str]> for Monkey {
    fn from(f: &[&str]) -> Self {
        let items: Vec<&str> = f[1].split(":").filter(|i| i.len() > 0).collect();
        let items: Vec<&str> = items.get(1).unwrap().split(",").collect();
        let items: Vec<usize> = items.iter().map(|i| i.trim().parse().unwrap()).collect();

        let operation_parts: Vec<&str> =
            f[2].split("=").last().unwrap().split_whitespace().collect();
        let op = operation_parts.get(1).unwrap().to_owned();
        let target = operation_parts.get(2).unwrap().to_owned();

        let test: Vec<&str> = f[3].split("by").collect();
        let divide_by: usize = test.last().unwrap().trim().parse().unwrap();

        let if_true: Vec<&str> = f[4].split("monkey").collect();
        let if_true: usize = if_true.last().unwrap().trim().parse().unwrap();

        let if_false: Vec<&str> = f[5].split("monkey").collect();
        let if_false: usize = if_false.last().unwrap().trim().parse().unwrap();

        let test = Test {
            divide_by,
            if_true,
            if_false,
        };

        let operation_target = match target {
            "old" => OperationTarget::Itself,
            n => OperationTarget::Value(n.parse().unwrap()),
        };

        let operation = match op {
            "+" => Operation::AddTo(operation_target),
            "*" => Operation::MultiplyBy(operation_target),
            _ => panic!("unexpected operator"),
        };

        Self {
            inspection_count: 0,
            starting_items: VecDeque::from(items),
            operation,
            test,
        }
    }
}

fn main() {
    let mut monkies: Vec<Monkey> = vec![];

    let input = read_to_string("./input.txt").unwrap();
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .for_each(|f| {
            let monkey: Monkey = f.into();

            monkies.push(monkey);
        });

    let lcm: usize = monkies.iter().map(|m| m.test.divide_by).product();
    // do 0..20 for part 1
    for _ in 0..10_000 {
        for i in 0..monkies.len() {
            for _ in 0..monkies[i].starting_items.len() {
                let mut value = monkies[i].starting_items.pop_front().unwrap();
                monkies[i].inspection_count += 1;
                match monkies[i].operation {
                    Operation::AddTo(target) => match target {
                        OperationTarget::Value(v) => value += v,
                        OperationTarget::Itself => value += value,
                    },
                    Operation::MultiplyBy(target) => match target {
                        OperationTarget::Value(v) => value *= v,
                        OperationTarget::Itself => value *= value,
                    },
                };

                // do value /= 3 for part 1
                value %= lcm;

                let is_divisible = value % monkies[i].test.divide_by == 0;
                let true_i = monkies[i].test.if_true as usize;
                let false_i = monkies[i].test.if_false as usize;
                if is_divisible {
                    monkies[true_i].starting_items.push_back(value);
                } else {
                    monkies[false_i].starting_items.push_back(value);
                }
            }
        }
    }

    let mut scores: Vec<usize> = monkies.iter().map(|m| m.inspection_count).collect();
    scores.sort();

    let top_two = scores.get((scores.len() - 2)..).unwrap();

    let monkey_business_score: usize = top_two.iter().product();

    println!("total monkey business: {}", monkey_business_score);
}
