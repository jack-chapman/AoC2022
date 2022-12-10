use std::{collections::HashMap, fs::read_to_string};

const MAX_SIZE: u32 = 100_000;

#[derive(Debug)]
enum Move<'a> {
    Into(&'a str),
    ToParent,
    ToRoot,
}

impl<'a> From<&'a str> for Move<'a> {
    fn from(value: &'a str) -> Self {
        if value == ".." {
            return Self::ToParent;
        }
        if value == "/" {
            return Self::ToRoot;
        }
        Self::Into(value)
    }
}

#[derive(Debug)]
enum InputLine<'a> {
    Move(Move<'a>),
    List,
    Dir,
    File(u32),
}

impl<'a> TryFrom<&'a str> for InputLine<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.starts_with("$ cd") {
            let split: Vec<&str> = value.split(" ").collect();
            if let Some(dir) = split.get(2) {
                return Ok(Self::Move(Move::from(dir.to_owned())));
            } else {
                return Err("cannot parse 'cd' command");
            }
        }
        if value.starts_with("$ ls") {
            return Ok(Self::List);
        }
        if value.starts_with("dir") {
            return Ok(Self::Dir);
        }
        let split: Vec<&str> = value.split(" ").collect();
        let size = split.get(0);
        if let Ok(size) = size.unwrap().parse() {
            return Ok(Self::File(size));
        } else {
            return Err("cannot parse size of file line");
        }
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    let inputs: Vec<InputLine> = input
        .lines()
        .map(|line| {
            return InputLine::try_from(line).unwrap();
        })
        .collect();

    let mut current_path: Vec<&str> = vec![];
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    // use instructions to build filesystem
    for input in inputs {
        match input {
            InputLine::Move(dir) => match dir {
                Move::Into(dir) => {
                    current_path.push(dir);
                    let path = current_path.join("/");
                    if !dir_sizes.contains_key(&path) {
                        dir_sizes.insert(path, 0);
                    }
                }
                Move::ToParent => {
                    current_path.pop();
                }
                Move::ToRoot => {
                    current_path.push("<root>");
                    let path = current_path.join("/");
                    if !dir_sizes.contains_key(&path) {
                        dir_sizes.insert(path, 0);
                    }
                }
            },
            InputLine::Dir => {}
            InputLine::File(filesize) => {
                let mut update_list: Vec<String> = vec![];
                for dir in &current_path {
                    update_list.push(dir.to_string());
                    let update_path = update_list.join("/");
                    dir_sizes
                        .entry(update_path)
                        .and_modify(|size| *size += filesize);
                }
            }
            _ => {}
        }
    }

    // part 1
    let total: u32 = dir_sizes.values().filter(|&size| *size < MAX_SIZE).sum();
    println!("total size: {}", total);

    // part 2
    let root = String::from("<root>");
    let space_needed: u32 = 30_000_000 - (70_000_000 - dir_sizes.get(&root).unwrap());
    let candidate = dir_sizes
        .iter()
        .filter(|(_, &size)| size >= space_needed)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    let candidate_size = candidate.1;
    println!("candidate size: {}", candidate_size);
}
