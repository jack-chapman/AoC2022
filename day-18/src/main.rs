use std::{fs::read_to_string, ops::Add, str::FromStr};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Cube(i16, i16, i16);

impl FromStr for Cube {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_terminator(',').collect();
        let x = parts.get(0);
        let y = parts.get(1);
        let z = parts.get(2);
        if let (Some(x), Some(y), Some(z)) = (x, y, z) {
            let x = x.parse::<i16>();
            let y = y.parse::<i16>();
            let z = z.parse::<i16>();
            if let (Ok(x), Ok(y), Ok(z)) = (x, y, z) {
                Ok(Self(x, y, z))
            } else {
                Err("cannot parse input into numbers".to_string())
            }
        } else {
            Err("cannot split input correctly".to_string())
        }
    }
}

impl Add for Cube {
    type Output = Cube;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;
        Self(x, y, z)
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let cubes: Vec<Cube> = input
        .lines()
        .map(|line| Cube::from_str(line).unwrap())
        .collect();

    let all_exposed_surfaces = get_surface_area(&cubes);

    println!("surface area: {}", all_exposed_surfaces);

    let external_surfaces = get_external_surface_area(&cubes);

    println!("external surface area: {}", external_surfaces);
}

fn get_surface_area(cubes: &Vec<Cube>) -> i32 {
    let directions = vec![
        Cube(1, 0, 0),  // left
        Cube(-1, 0, 0), // right
        Cube(0, 1, 0),  // down
        Cube(0, -1, 0), // up
        Cube(0, 0, 1),  // forward
        Cube(0, 0, -1), // back
    ];

    let mut exposed_surfaces = 0;
    // loop over all cubes
    for cube in cubes {
        let cube = *cube;
        let mut blocked_sides = 0;
        // count how many cubes exist at cube position + directions
        for direction in &directions {
            let direction = *direction;
            let new_pos = cube + direction;
            if cubes.contains(&new_pos) {
                blocked_sides += 1;
            }
        }
        // add 6 - count to total
        exposed_surfaces += 6 - blocked_sides;
    }

    exposed_surfaces
}

fn get_external_surface_area(cubes: &Vec<Cube>) -> i32 {
    let x_values: Vec<i16> = cubes.iter().map(|c| c.0).collect();
    let max_x = x_values.iter().max().unwrap() + 1;
    let min_x = x_values.iter().min().unwrap() - 1;

    let y_values: Vec<i16> = cubes.iter().map(|c| c.1).collect();
    let max_y = y_values.iter().max().unwrap() + 1;
    let min_y = y_values.iter().min().unwrap() - 1;

    let z_values: Vec<i16> = cubes.iter().map(|c| c.2).collect();
    let max_z = z_values.iter().max().unwrap() + 1;
    let min_z = z_values.iter().min().unwrap() - 1;

    let mut space: Vec<(Cube, bool)> = vec![];

    for x in min_x..max_x {
        for y in min_y..max_y {
            for z in min_z..max_z {
                let cube = Cube(x, y, z);
                let in_rocks = cubes.contains(&cube);
                space.push((cube, in_rocks));
            }
        }
    }

    // space becomes solidified, meaning only internal space has false
    flood_fill(&mut space, Cube(0, 0, 0), false, true);

    let internal_cubes: Vec<Cube> = space.iter().filter(|(_, v)| !v).map(|(c, _)| *c).collect();

    let directions = vec![
        Cube(1, 0, 0),  // left
        Cube(-1, 0, 0), // right
        Cube(0, 1, 0),  // down
        Cube(0, -1, 0), // up
        Cube(0, 0, 1),  // forward
        Cube(0, 0, -1), // back
    ];

    // do same exposed surface calculation as before, but also check the space isnt internal too

    let mut exposed_surfaces = 0;

    for cube in cubes {
        let cube = *cube;
        let mut blocked_sides = 0;
        // count how many cubes exist at cube position + directions
        for direction in &directions {
            let direction = *direction;
            let new_pos = cube + direction;
            if cubes.contains(&new_pos) || internal_cubes.contains(&new_pos) {
                blocked_sides += 1;
            }
        }
        // add 6 - count to total
        exposed_surfaces += 6 - blocked_sides;
    }

    exposed_surfaces
}

fn flood_fill(cubes: &mut Vec<(Cube, bool)>, coords: Cube, target: bool, replacement: bool) {
    let directions = vec![
        Cube(1, 0, 0),  // left
        Cube(-1, 0, 0), // right
        Cube(0, 1, 0),  // down
        Cube(0, -1, 0), // up
        Cube(0, 0, 1),  // forward
        Cube(0, 0, -1), // back
    ];
    // get the cube at the coords
    let found = cubes.iter_mut().find(|(c, _)| *c == coords);

    if found.is_none() {
        return;
    }

    let found = found.unwrap();
    // if it has the target value already, return
    if found.1 != target {
        return;
    }
    // if not, set it to the replacement value
    found.1 = replacement;
    // run flood fill on all the directions
    for direction in directions {
        let new_coord = coords + direction;
        flood_fill(cubes, new_coord, target, replacement);
    }
}
