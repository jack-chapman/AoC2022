use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let split: Vec<Vec<&str>> = lines
        .split(|line| line.to_owned() == "")
        .map(|line| line.to_owned())
        .collect();
    let initial_state = split.get(0).unwrap().to_owned();
    let instructions = split.get(1).unwrap().to_owned();

    let mut state: BTreeMap<usize, Vec<char>> = BTreeMap::new();

    // transform crates from diagram into hashmap
    for i in initial_state {
        for (idx, c) in i.char_indices() {
            let scaled_idx = (idx / 4).saturating_add(1);
            if c.is_alphabetic() {
                if let Some(vec) = state.get_mut(&scaled_idx) {
                    vec.push(c);
                } else {
                    let mut new_vec = Vec::new();
                    new_vec.push(c);
                    state.insert(scaled_idx, new_vec);
                }
            }
        }
    }

    // reverse order of crates
    for (_, v) in state.iter_mut() {
        v.reverse();
    }

    // make copy of state for part 2
    let mut state_2 = state.clone();

    // process instructions
    for instruction in instructions {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let amount: usize = parts.get(1).unwrap().to_owned().parse().unwrap();
        let from: usize = parts.get(3).unwrap().to_owned().parse().unwrap();
        let to: usize = parts.get(5).unwrap().to_owned().parse().unwrap();

        // part 1
        let mut from_stack = Vec::from_iter(state.get(&from).unwrap().iter().map(|i| i.to_owned()));
        let mut to_stack = Vec::from_iter(state.get(&to).unwrap().iter().map(|i| i.to_owned()));

        for _ in 0..amount {
            // pop from end of from
            let c = from_stack.pop().unwrap().to_owned();
            // push to end of to
            to_stack.push(c);
        }

        // replace vecs in state
        state.insert(from, from_stack);
        state.insert(to, to_stack);

        // part 2
        let mut from_stack =
            Vec::from_iter(state_2.get(&from).unwrap().iter().map(|i| i.to_owned()));
        let mut to_stack = Vec::from_iter(state_2.get(&to).unwrap().iter().map(|i| i.to_owned()));

        // pop amount from end of from
        let range = (from_stack.len() - amount)..(from_stack.len());
        let c: Vec<char> = from_stack.drain(range).collect();
        to_stack.extend(c);

        // replace vecs in state
        state_2.insert(from, from_stack);
        state_2.insert(to, to_stack);
    }

    // log top of each stack
    let result: String = state
        .iter()
        .map(|(_, stack)| stack.last().unwrap())
        .collect();

    let result_2: String = state_2
        .iter()
        .map(|(_, stack)| stack.last().unwrap())
        .collect();

    println!("{}", result);
    println!("{}", result_2);
}
