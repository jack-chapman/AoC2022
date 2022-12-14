use std::{collections::HashSet, fs::read_to_string, str::FromStr};

#[derive(Debug, Eq, Clone, Copy, Hash)]
struct Point(i16, i16);

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(",").collect::<Vec<&str>>();
        let first = split.get(0);
        let last = split.get(1);
        match (first, last) {
            (Some(x), Some(y)) => {
                let x = x.parse::<i16>();
                let y = y.parse::<i16>();
                if let (Ok(x), Ok(y)) = (x, y) {
                    Ok(Self(x, y))
                } else {
                    Err("Cannot convert x and y to i16s".to_string())
                }
            }
            _ => Err("Unable to parse input into Point".to_string()),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Point {
    fn range(start: Point, end: Point) -> Vec<Point> {
        let mut range: Vec<Point> = vec![];
        if start.0 > end.0 {
            for i in end.0..start.0 {
                range.push(Point(i, start.1));
            }
        } else if end.0 > start.0 {
            for i in start.0..end.0 {
                range.push(Point(i, start.1));
            }
        }
        if start.1 > end.1 {
            for i in end.1..start.1 {
                range.push(Point(start.0, i));
            }
        } else if end.1 > start.1 {
            for i in start.1..end.1 {
                range.push(Point(start.0, i));
            }
        }
        range
    }
}

#[derive(Debug)]
struct Wall(HashSet<Point>);

impl FromStr for Wall {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("->").map(|s| s.trim()).collect::<Vec<&str>>();
        let points: Vec<Point> = split.iter().map(|p| p.parse::<Point>().unwrap()).collect();
        let mut wall: HashSet<Point> = HashSet::new();
        let mut start: Option<Point> = None;
        for point in points {
            wall.insert(point);
            if let Some(s) = start {
                let range = Point::range(s, point);
                wall.extend(range);
            }
            start = Some(point);
        }
        Ok(Self(wall))
    }
}

struct Sand(HashSet<Point>);

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let mut rocks: HashSet<Point> = HashSet::new();

    input
        .lines()
        .map(|line| line.parse::<Wall>().unwrap())
        .for_each(|wall| {
            rocks.extend(wall.0);
        });

    let highest_point = rocks.iter().map(|p| p.1).max().unwrap();
    let floor_level = highest_point + 2;

    let mut sand_pile = Sand(HashSet::new());
    let mut out_of_space = false;

    let mut particle = Point(500, 0);

    while !out_of_space {
        if particle.1 == i16::MAX {
            break;
        }
        let below = Point(particle.0, particle.1 + 1);
        let below_left = Point(below.0 - 1, below.1);
        let below_right = Point(below.0 + 1, below.1);
        if !rocks.contains(&below) && !sand_pile.0.contains(&below) && below.1 < floor_level {
            // empty space below -> fall down
            particle = below;
            continue;
        } else {
            // there is something below — do we fall off?
            if !rocks.contains(&below_left)
                && !sand_pile.0.contains(&below_left)
                && below_left.1 < floor_level
            {
                // empty space below and left -> fall that way
                particle = below_left;
                continue;
            }
            if !rocks.contains(&below_right)
                && !sand_pile.0.contains(&below_right)
                && below_left.1 < floor_level
            {
                // empty space below and right -> fall that way
                particle = below_right;
                continue;
            }
        }
        // cant move
        if sand_pile.0.contains(&particle) || particle.1 == i16::MAX {
            // The pile of sand contains the space we're in
            // so there is nowhere else to go
            out_of_space = true;
        } else {
            // add it to the pile
            sand_pile.0.insert(particle);
            // 'respawn' particle
            particle = Point(500, 0);
        }
    }

    println!("{}", sand_pile.0.len());
}
