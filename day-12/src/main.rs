use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let mut grid: HashMap<(i16, i16), char> = HashMap::new();
    let mut start: (i16, i16) = (255, 255);
    let mut end: (i16, i16) = (255, 255);
    let mut possible_starts: Vec<(i16, i16)> = Vec::new();

    let input = read_to_string("./input.txt").unwrap();

    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut height = c;
            if c == 'S' {
                start = (x as i16, y as i16);
                height = 'a';
            } else if c == 'E' {
                end = (x as i16, y as i16);
                height = 'z';
            }
            if height == 'a' {
                possible_starts.push((x as i16, y as i16));
            }
            grid.insert((x as i16, y as i16), height);
        }
    }

    let distance = get_distance(&grid, start, end).unwrap();

    // part 1
    println!("{}", distance);

    // part 2 brute force lmao
    let shortest_distance_from_any_start = possible_starts
        .iter()
        .map(|start| get_distance(&grid, *start, end))
        .filter(|distance| distance.is_some())
        .min()
        .unwrap()
        .unwrap();

    println!("{}", shortest_distance_from_any_start);
}

fn get_distance(
    grid: &HashMap<(i16, i16), char>,
    start: (i16, i16),
    end: (i16, i16),
) -> Option<u16> {
    let mut visited: HashMap<(i16, i16), u16> = HashMap::new();
    let mut to_visit: VecDeque<(i16, i16)> = VecDeque::new();

    visited.insert(start, 0);
    to_visit.push_back(start);

    while !to_visit.is_empty() {
        let (cx, cy) = to_visit.pop_front().unwrap();
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = ((cx + dx), (cy + dy));

            if grid.contains_key(&(nx, ny)) && !visited.contains_key(&(nx, ny)) {
                if *grid.get(&(nx, ny)).unwrap() as i16 - *grid.get(&(cx, cy)).unwrap() as i16 <= 1
                {
                    to_visit.push_back((nx, ny));
                    visited.insert((nx, ny), visited.get(&(cx, cy)).unwrap() + 1);

                    if (nx, ny) == end {
                        return Some(visited.get(&(cx, cy)).unwrap() + 1);
                    }
                }
            }
        }
    }
    None
}
