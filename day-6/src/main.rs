use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let input: Vec<&str> = input.split("").collect();

    for (index, window) in input.windows(4).skip(1).enumerate() {
        let set: HashSet<&str> = HashSet::from_iter(window.iter().map(|i| i.to_owned()));
        if set.len() == window.len() {
            println!("4 uniques - {:?} — {}", window, index + 4);
            break;
        }
    }

    for (index, window) in input.windows(14).skip(1).enumerate() {
        let set: HashSet<&str> = HashSet::from_iter(window.iter().map(|i| i.to_owned()));
        if set.len() == window.len() {
            println!("14 uniques - {:?} — {}", window, index + 14);
            break;
        }
    }
}
