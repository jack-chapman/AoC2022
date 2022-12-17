use std::{collections::BTreeSet, fs::read_to_string, hash::Hash, ops::Add, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        Self(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
enum ShapeLayout {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Square,
}

#[derive(Debug, Clone)]
struct Shape {
    points: Vec<Point>,
}

impl Shape {
    fn new(layout: ShapeLayout) -> Self {
        let points = match layout {
            ShapeLayout::Horizontal => vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)],
            ShapeLayout::Vertical => vec![Point(0, 0), Point(0, -1), Point(0, -2), Point(0, -3)],
            ShapeLayout::Cross => vec![
                Point(1, -2),
                Point(0, -1),
                Point(1, -1),
                Point(2, -1),
                Point(1, 0),
            ],
            ShapeLayout::Angle => vec![
                Point(0, 0),
                Point(1, 0),
                Point(2, 0),
                Point(2, -1),
                Point(2, -2),
            ],
            ShapeLayout::Square => vec![Point(0, 0), Point(1, 0), Point(0, -1), Point(1, -1)],
        };
        Self { points }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn vector(dir: &Self) -> Point {
        match dir {
            Self::Left => Point(-1, 0),
            Self::Right => Point(1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == ">" {
            return Ok(Self::Right);
        }
        if s == "<" {
            return Ok(Self::Left);
        }
        Err(format!("must be < or >. {} given.", s))
    }
}

fn main() {
    let part_1 = solve_pt1("./input.txt");

    println!(
        "part 1: {:?}",
        part_1.iter().map(|p| p.1.abs()).max().unwrap()
    );

    let part_2 = solve_pt2("./input.txt");
    println!("part 2: {:?}", part_2);
}

fn solve_pt1(input_file: &str) -> BTreeSet<Point> {
    let input = read_to_string(input_file).unwrap();

    let shapes = vec![
        Shape::new(ShapeLayout::Horizontal),
        Shape::new(ShapeLayout::Cross),
        Shape::new(ShapeLayout::Angle),
        Shape::new(ShapeLayout::Vertical),
        Shape::new(ShapeLayout::Square),
    ];

    let mut shape_cycle = shapes.iter().cycle();

    let directions: Vec<Direction> = input
        .split("")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| Direction::from_str(s).unwrap())
        .collect();

    let mut direction_cycle = directions.iter().cycle();

    let mut used_points: BTreeSet<Point> = BTreeSet::new();

    const WIDTH: i32 = 7;

    for x in 0..=WIDTH {
        used_points.insert(Point(x, 0));
    }

    for _ in 0..2022 as i64 {
        let shape = shape_cycle.next().unwrap().clone();
        let highest_point = used_points.iter().map(|p| p.1).min().unwrap_or(0);
        let mut position = Point(2, highest_point - 4);
        loop {
            let direction = direction_cycle.next().unwrap();
            let move_in = Direction::vector(direction);
            let hits_rocks_on_slide = shape.points.iter().any(|p| {
                let pos = p.to_owned() + position + move_in;
                used_points.contains(&pos)
            });
            let hits_wall_on_slide = shape.points.iter().any(|p| {
                let pos = p.to_owned() + position + move_in;
                let blocked = pos.0 >= WIDTH || pos.0 < 0;
                blocked
            });
            if !hits_wall_on_slide && !hits_rocks_on_slide {
                position = position + move_in;
            }
            let hits_rocks_on_fall = shape.points.iter().any(|p| {
                let down = p.to_owned() + position + Point(0, 1);
                used_points.contains(&down)
            });
            if hits_rocks_on_fall {
                for point in shape.points.iter() {
                    let pos = point.to_owned() + position;
                    used_points.insert(pos);
                }
                break;
            } else {
                position = position + Point(0, 1);
            }
        }
    }

    used_points
}

fn solve_pt2(input_file: &str) -> u64 {
    let input = read_to_string(input_file).unwrap();

    let shapes = vec![
        Shape::new(ShapeLayout::Horizontal),
        Shape::new(ShapeLayout::Cross),
        Shape::new(ShapeLayout::Angle),
        Shape::new(ShapeLayout::Vertical),
        Shape::new(ShapeLayout::Square),
    ];

    let mut shape_cycle = shapes.iter().cycle();

    let directions: Vec<Direction> = input
        .split("")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| Direction::from_str(s).unwrap())
        .collect();

    let mut direction_cycle = directions.iter().cycle();

    let mut used_points: BTreeSet<Point> = BTreeSet::new();

    const WIDTH: i32 = 7;
    // literally guessed this number as enough to determine a pattern
    const AMOUNT: i64 = 6000;

    for x in 0..=WIDTH {
        used_points.insert(Point(x, 0));
    }

    let mut max_height_seen = 0;
    let mut prev_height = 0;
    let mut height_deltas: Vec<u64> = Vec::with_capacity(AMOUNT as usize);

    let mut i = 0;
    while i < AMOUNT {
        i += 1;
        let shape = shape_cycle.next().unwrap().clone();
        let highest_point = used_points.iter().map(|p| p.1).min().unwrap_or(0);
        let mut position = Point(2, highest_point - 4);
        loop {
            let direction = direction_cycle.next().unwrap();
            let move_in = Direction::vector(direction);
            let hits_rocks_on_slide = shape.points.iter().any(|p| {
                let pos = p.to_owned() + position + move_in;
                used_points.contains(&pos)
            });
            let hits_wall_on_slide = shape.points.iter().any(|p| {
                let pos = p.to_owned() + position + move_in;
                let blocked = pos.0 >= WIDTH || pos.0 < 0;
                blocked
            });
            if !hits_wall_on_slide && !hits_rocks_on_slide {
                position = position + move_in;
            }
            let hits_rocks_on_fall = shape.points.iter().any(|p| {
                let down = p.to_owned() + position + Point(0, 1);
                used_points.contains(&down)
            });
            if hits_rocks_on_fall {
                for point in shape.points.iter() {
                    let pos = point.to_owned() + position;
                    used_points.insert(pos);
                }
                let height = used_points.iter().map(|p| p.1.abs()).max().unwrap() as u64;
                height_deltas.push(height - prev_height);
                prev_height = height;
                max_height_seen = max_height_seen.max(height);
                break;
            } else {
                position = position + Point(0, 1);
            }
        }
    }

    // skip the first bunch of deltas
    const SKIP: usize = 500;
    let height_delta_for_pattern = &height_deltas[SKIP..];
    let mut found_pattern_len = 0;
    for pattern_len in 1..=height_delta_for_pattern.len() / 2 {
        let pattern = &height_delta_for_pattern[0..pattern_len];
        let mut found = true;
        for i in 0..height_delta_for_pattern.len() - pattern_len {
            if height_delta_for_pattern[i + pattern_len] != pattern[i % pattern_len] {
                found = false;
                break;
            }
        }
        if found {
            found_pattern_len = pattern_len;
            break;
        }
    }

    // find height of the pattern and pretend we iterated a trillion times
    const BIG_NUM: u64 = 1_000_000_000_000;
    let pattern = &height_delta_for_pattern[0..found_pattern_len];
    let pattern_sum = pattern.iter().sum::<u64>();
    let initial_deltas = &height_deltas[0..&height_deltas.len() / 4];
    let initial_sum = initial_deltas.iter().sum::<u64>();
    let num_patterns = (BIG_NUM - initial_deltas.len() as u64) / pattern.len() as u64;
    let num_leftover = ((BIG_NUM - initial_deltas.len() as u64) % pattern.len() as u64) as usize;
    let leftover_sum = pattern[0..num_leftover].iter().sum::<u64>();

    // add up all the heights
    initial_sum + pattern_sum * num_patterns + leftover_sum
}
