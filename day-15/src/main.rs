use std::{collections::HashSet, fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, Hash, Eq)]
struct Point(i32, i32);

impl Point {
    fn distance_to(a: Point, b: Point) -> u32 {
        let dx = i32::abs_diff(a.0, b.0);
        let dy = i32::abs_diff(a.1, b.1);
        dx + dy
    }
}

impl FromStr for Point {
    type Err = String;
    // s must be "x=2, y=18"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(",").map(|s| s.trim()).collect();
        let (x, y) = (split.get(0), split.get(1));
        if let (Some(x), Some(y)) = (x, y) {
            let x = x.split("=").map(|s| s.trim()).collect::<Vec<&str>>();
            let y = y.split("=").map(|s| s.trim()).collect::<Vec<&str>>();
            if let (Some(x), Some(y)) = (x.get(1), y.get(1)) {
                let x = x.parse::<i32>();
                let y = y.parse::<i32>();
                if let (Ok(x), Ok(y)) = (x, y) {
                    Ok(Self(x, y))
                } else {
                    Err("could not parse input into numbers".to_string())
                }
            } else {
                Err("input in wrong format".to_string())
            }
        } else {
            Err("input in wrong format".to_string())
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug)]
struct Pair {
    sensor: Point,
    beacon: Point,
    distance: u32,
}

impl Pair {
    fn can_reach(&self, point: Point) -> bool {
        Point::distance_to(self.sensor, point) <= self.distance
    }
}

impl FromStr for Pair {
    type Err = String;
    // s must be "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(":").map(|s| s.trim()).collect();
        let sensor = split.get(0);
        let beacon = split.get(1);
        if let (Some(sensor), Some(beacon)) = (sensor, beacon) {
            let sensor: Vec<&str> = sensor.split("at").map(|s| s.trim()).collect();
            let beacon: Vec<&str> = beacon.split("at").map(|s| s.trim()).collect();
            let sensor = sensor.get(1);
            let beacon = beacon.get(1);
            if let (Some(sensor), Some(beacon)) = (sensor, beacon) {
                let sensor = sensor.parse::<Point>();
                let beacon = beacon.parse::<Point>();
                if let (Ok(sensor), Ok(beacon)) = (sensor, beacon) {
                    let distance = Point::distance_to(sensor, beacon);
                    Ok(Self {
                        sensor,
                        beacon,
                        distance,
                    })
                } else {
                    Err("cannot create Points".to_string())
                }
            } else {
                Err("input in wrong format".to_string())
            }
        } else {
            Err("input in wrong format".to_string())
        }
    }
}

fn main() {
    // parse input into sensor and beacon pairs
    let pairs: Vec<Pair> = read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let l = l.trim();
            // assign distances to each pair
            return Pair::from_str(l).unwrap();
        })
        .collect();

    // part 1
    let largest_x = pairs
        .iter()
        .map(|p| {
            let (a, b) = (p.beacon.0, p.beacon.0);
            let max = i32::max(a, b);
            max + p.distance as i32
        })
        .max()
        .unwrap();

    let smallest_x = pairs
        .iter()
        .map(|p| {
            let (a, b) = (p.beacon.0, p.beacon.0);
            let max = i32::min(a, b);
            max - p.distance as i32
        })
        .min()
        .unwrap();

    // count how many unique positions have a distance to any beacon less than their own distance
    let mut overlap_points: HashSet<Point> = HashSet::new();
    // loop over all positions between smallest and largets x positions
    for x in smallest_x..largest_x {
        for pair in &pairs {
            // y is 2_000_000
            let p = Point(x, 2_000_000);
            if p == pair.beacon {
                continue;
            }
            let s_to_p = Point::distance_to(p, pair.sensor);
            if s_to_p <= pair.distance {
                overlap_points.insert(p);
            }
        }
    }

    println!("total positions: {}", overlap_points.len());

    // part 2
    let mut point: Option<Point> = None;

    let max_coord = 4_000_000;

    'outer: for pair in &pairs {
        let distance: i32 = pair.distance as i32 + 1;
        let min = i32::min(pair.sensor.0 - distance, 0);
        let max = i32::max(pair.sensor.0 + distance, max_coord);

        for x in min..pair.sensor.0 {
            let y1 = pair.sensor.1 + (distance - (x - min));
            if y1 >= 0 && y1 <= max_coord && !in_range(&pairs, Point(x, y1)) {
                point = Some(Point(x, y1));
                break 'outer;
            }

            let y2 = pair.sensor.1 - (distance - (x - min));
            if y2 >= 0 && y2 <= max_coord && !in_range(&pairs, Point(x, y2)) {
                point = Some(Point(x, y2));
                break 'outer;
            }
        }

        let x = pair.sensor.0;
        let y = pair.sensor.1 + distance;
        if y >= 0 && y <= max_coord && !in_range(&pairs, Point(x, y)) {
            point = Some(Point(x, y));
            break 'outer;
        }
        let x = pair.sensor.0;
        let y = pair.sensor.1 - distance;
        if y >= 0 && y <= max_coord && !in_range(&pairs, Point(x, y)) {
            point = Some(Point(x, y));
            break 'outer;
        }

        for x in (pair.sensor.0 + 1)..max {
            let y1 = pair.sensor.1 + (distance - (x - pair.sensor.0));
            if y1 >= 0 && y1 <= max_coord && !in_range(&pairs, Point(x, y1)) {
                point = Some(Point(x, y1));
                break 'outer;
            }

            let y2 = pair.sensor.1 - (distance - (x - pair.sensor.0));
            if y2 >= 0 && y2 <= max_coord && !in_range(&pairs, Point(x, y2)) {
                point = Some(Point(x, y2));
                break 'outer;
            }
        }
    }

    if point.is_some() {
        let point = point.unwrap();
        let freq: i64 = point.0 as i64 * 4_000_000 + point.1 as i64;
        println!("tuning frequency: {}", freq);
    }
}

fn in_range(pairs: &Vec<Pair>, point: Point) -> bool {
    for pair in pairs {
        if pair.can_reach(point) {
            return true;
        }
    }
    false
}
