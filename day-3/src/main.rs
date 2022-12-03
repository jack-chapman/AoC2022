use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Backpack {
    combined: String,
    first_compartment: String,
    second_compartment: String,
}

impl Backpack {
    fn new(first: String, last: String) -> Self {
        let mut combined = first.clone();
        combined.push_str(last.as_str());
        Self {
            combined,
            first_compartment: first,
            second_compartment: last,
        }
    }
    fn find_common_item(&self) -> Option<char> {
        let mut first_uniques: HashSet<char> = HashSet::from_iter(self.first_compartment.chars());
        let second_uniques: HashSet<char> = HashSet::from_iter(self.second_compartment.chars());
        for c in second_uniques.into_iter() {
            let result = first_uniques.insert(c);
            if !result {
                return Some(c);
            }
        }
        None
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let backpack_lines: Vec<&str> = input.split("\n").filter(|line| line.len() > 0).collect();
    let backpacks: Vec<Backpack> = backpack_lines
        .iter()
        .map(|line| {
            let line = line.to_owned();
            let (left, right) = line.split_at(line.len() / 2);
            Backpack::new(left.to_string(), right.to_string())
        })
        .collect();

    // part 1
    let common_items: Vec<char> = backpacks
        .iter()
        .map(|backpack| backpack.find_common_item().unwrap())
        .collect();

    let score = calculate_score(common_items);

    println!("Total score: {}", score);

    // part 2
    // chunk backpacks into groups of three strings
    let chunks = Vec::from_iter(backpacks.chunks(3));
    let chunks: Vec<Vec<String>> = chunks
        .into_iter()
        .map(|chunk| chunk.into_iter().map(|backpack| backpack.combined.clone()))
        .map(|a| Vec::from_iter(a))
        .collect();

    // find common char in each group of three strings
    let mut common_items: Vec<char> = Vec::new();
    for chunk in chunks {
        // vec of 3 hashsets containing unique chars
        let sets: Vec<HashSet<char>> = chunk
            .iter()
            .map(|c| HashSet::from_iter(c.chars()))
            .collect();

        let intersected: Vec<char> = sets
            .into_iter()
            .reduce(|a, b| HashSet::from_iter(a.intersection(&b).into_iter().map(|i| *i)))
            .unwrap()
            .into_iter()
            .collect();

        let result = intersected.get(0).unwrap().to_owned();

        common_items.push(result);
    }

    // call calculate_score on common chars from groups
    let score = calculate_score(common_items);
    println!("Grouped score: {}", score);
}

fn calculate_score(chars: Vec<char>) -> u32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    for c in chars.iter() {
        let score: u32 = alphabet
            .chars()
            .position(|a| a == c.to_owned())
            .unwrap()
            .saturating_add(1) as u32;
        sum += score;
    }
    sum
}
