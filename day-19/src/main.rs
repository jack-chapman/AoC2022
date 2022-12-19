use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

type RecipePart = (u32, Material);

#[derive(Debug)]
struct Blueprint {
    id: u32,
    robot_recipes: [Vec<RecipePart>; 4],
}

impl Blueprint {
    fn from_line(line: &str) -> Self {
        let mut id = 0;
        let mut ore_robot_ore_cost = 0;
        let mut clay_robot_ore_cost = 0;
        let (mut obs_robot_ore_cost, mut obs_robot_clay_cost) = (0, 0);
        let (mut geo_robot_ore_cost, mut geo_robot_obs_cost) = (0, 0);

        scanf::sscanf!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", id, ore_robot_ore_cost, clay_robot_ore_cost, obs_robot_ore_cost, obs_robot_clay_cost, geo_robot_ore_cost, geo_robot_obs_cost).unwrap();

        let ore_robot = vec![(ore_robot_ore_cost, Material::Ore)];
        let clay_robot = vec![(clay_robot_ore_cost, Material::Ore)];
        let obsidian_robot = vec![
            (obs_robot_ore_cost, Material::Ore),
            (obs_robot_clay_cost, Material::Clay),
        ];
        let geode_robot = vec![
            (geo_robot_ore_cost, Material::Ore),
            (geo_robot_obs_cost, Material::Obsidian),
        ];

        Self {
            id,
            robot_recipes: [ore_robot, clay_robot, obsidian_robot, geode_robot],
        }
    }
}

#[derive(Clone, Copy)]
struct SearchState {
    time_remaining: u32,
    robots: [u32; 4],
    materials: [u32; 4],
}

impl SearchState {
    fn can_build_robot(
        &self,
        robot_type: usize,
        blueprint: &Blueprint,
        max_materials: &[u32],
    ) -> bool {
        let recipe = &blueprint.robot_recipes[robot_type];
        let maxed_out = self.robots[robot_type] >= max_materials[robot_type];
        !maxed_out
            && recipe
                .iter()
                .all(|&(amount, material)| self.materials[material as usize] >= amount)
    }

    fn build_robot(&mut self, robot_type: usize, blueprint: &Blueprint) {
        self.robots[robot_type] += 1;
        for &(amount, material) in &blueprint.robot_recipes[robot_type] {
            self.materials[material as usize] -= amount;
        }
    }

    fn unbuild_robot(&mut self, robot_type: usize, blueprint: &Blueprint) {
        self.robots[robot_type] -= 1;
        for &(amount, material) in &blueprint.robot_recipes[robot_type] {
            self.materials[material as usize] += amount;
        }
    }
}

fn main() {
    let blueprints: Vec<Blueprint> = read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(Blueprint::from_line)
        .collect();

    let part_1 = blueprints
        .iter()
        .map(|bp| (bp.id * get_blueprint_score(bp, 24)))
        .sum::<u32>();

    println!("best out of blueprints: {}", part_1);

    let part_2 = blueprints[0..3]
        .iter()
        .map(|bp| get_blueprint_score(bp, 32))
        .product::<u32>();

    println!("product of first 3: {}", part_2);
}

fn get_blueprint_score(blueprint: &Blueprint, time_remaining: u32) -> u32 {
    let state = SearchState {
        time_remaining,
        robots: [1, 0, 0, 0],
        materials: [0, 0, 0, 0],
    };
    let max_materials = get_max_materials(blueprint);
    run_for_blueprint(&state, blueprint, &max_materials, None, 0)
}

fn get_max_materials(blueprint: &Blueprint) -> [u32; 4] {
    let mut maxs = [0, 0, 0, u32::MAX];

    for recipe in &blueprint.robot_recipes {
        for &(amount, material) in recipe {
            let i = material as usize;
            maxs[i] = std::cmp::max(maxs[i], amount);
        }
    }

    maxs
}

fn run_for_blueprint(
    state: &SearchState,
    blueprint: &Blueprint,
    max_materials: &[u32],
    prev_skipped: Option<&Vec<usize>>,
    best_so_far: u32,
) -> u32 {
    if state.time_remaining == 1 {
        return state.materials[3] + state.robots[3];
    }

    if optimistic_best(state, Material::Geode) < best_so_far {
        return 0;
    }

    let min_obs = max_materials[2];
    if optimistic_best(state, Material::Obsidian) < min_obs {
        return state.materials[3] + state.robots[3] * state.time_remaining;
    }

    let mut new_state = *state;
    new_state.time_remaining -= 1;
    (0..4).for_each(|i| new_state.materials[i] += new_state.robots[i]);

    if state.can_build_robot(Material::Geode as usize, blueprint, max_materials) {
        new_state.build_robot(Material::Geode as usize, blueprint);
        return run_for_blueprint(&new_state, blueprint, max_materials, None, best_so_far);
    }

    let robots_available: Vec<usize> = (0..3)
        .filter(|i| state.can_build_robot(*i, blueprint, max_materials))
        .collect();
    let mut best = best_so_far;

    for &robot_type in &robots_available {
        if prev_skipped
            .map(|ls| ls.contains(&robot_type))
            .unwrap_or(false)
        {
            continue;
        }

        new_state.build_robot(robot_type, blueprint);
        let score = run_for_blueprint(&new_state, blueprint, max_materials, None, best);
        best = std::cmp::max(score, best);
        new_state.unbuild_robot(robot_type, blueprint);
    }

    let score = run_for_blueprint(
        &new_state,
        blueprint,
        max_materials,
        Some(&robots_available),
        best,
    );
    best = std::cmp::max(score, best);

    best
}

fn optimistic_best(state: &SearchState, material: Material) -> u32 {
    let mat = material as usize;
    let i = state.time_remaining;
    state.materials[mat] + state.robots[mat] * i + i * (i - 1) / 2
}
