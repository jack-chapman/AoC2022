use std::{collections::HashSet, fs::read_to_string};

fn find_marker(input: &Vec<&str>, size: usize) {
    for (index, window) in input.windows(size).skip(1).enumerate() {
        let set: HashSet<&str> = HashSet::from_iter(window.iter().map(|i| i.to_owned()));
        if set.len() == window.len() {
            println!("{} uniques - {:?} — {}", size, window, index + size);
            break;
        }
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let input: Vec<&str> = input.split("").collect();

    // part 1
    find_marker(&input, 4);

    // part 2
    find_marker(&input, 14);
}
