use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Movement {
    steps: u32,
    direction: Direction,
}

impl TryFrom<String> for Movement {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(" ").collect();
        if let (Some(first), Some(last)) = (parts.get(0), parts.get(1)) {
            let steps: u32 = last.parse().unwrap();
            if let Some(direction) = match first.to_owned() {
                "U" => Some(Direction::Up),
                "D" => Some(Direction::Down),
                "R" => Some(Direction::Right),
                "L" => Some(Direction::Left),
                _ => None,
            } {
                Ok(Movement { steps, direction })
            } else {
                Err("cannot create Movemet".to_string())
            }
        } else {
            Err("could not parse line".to_string())
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn difference(a: &Position, b: &Position) -> Position {
        Self::new(a.x - b.x, a.y - b.y)
    }

    fn move_into(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn follow(&mut self, target: (i32, i32)) -> (i32, i32) {
        let target = &Position {
            x: target.0,
            y: target.1,
        };
        let difference = Self::difference(target, &self);
        let x_abs = difference.x.abs();
        let y_abs = difference.y.abs();
        match (x_abs, y_abs) {
            (0, 0) => self.coords(),
            (1, 0) => self.coords(),
            (0, 1) => self.coords(),
            (1, 1) => self.coords(),
            (_, 0) => {
                // need to move horizontally
                if difference.x > 0 {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
                self.coords()
            }
            (0, _) => {
                // need to move vertically
                if difference.y > 0 {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
                self.coords()
            }
            (_, _) => {
                // need to move diagonally
                if difference.x > 0 {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
                if difference.y > 0 {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
                self.coords()
            }
        }
    }
}

struct Rope {
    positions: Vec<Position>,
}

impl Rope {
    fn new(size: u32, start: (i32, i32)) -> Self {
        let mut positions: Vec<Position> = vec![];
        for _ in 0..size {
            positions.push(Position::new(start.0, start.1));
        }
        Self { positions }
    }

    fn process_movement(&mut self, direction: &Direction) -> (i32, i32) {
        let mut last_pos = (0, 0);
        if let Some(first) = self.positions.first_mut() {
            first.move_into(direction);
            last_pos = first.coords();
        }
        for position in self.positions.iter_mut().skip(1) {
            position.follow(last_pos);
            last_pos = position.coords();
        }
        self.positions.last().unwrap().coords()
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    // set size to 2 for part 1
    let mut rope = Rope::new(10, (0, 0));

    let mut tail_positions = HashSet::new();

    for line in input.lines() {
        if let Ok(movement) = Movement::try_from(line.to_string()) {
            for _ in 0..movement.steps {
                let tail_position = rope.process_movement(&movement.direction);
                tail_positions.insert(tail_position);
            }
        }
    }

    println!("tail moved into {} positions", tail_positions.len());
}
