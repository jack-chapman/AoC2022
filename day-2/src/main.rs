use std::fs::read_to_string;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(char: &str) -> Result<Self, Self::Error> {
        match char {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err("Cannot be converted to Shape"),
        }
    }
}

enum Strat {
    Win,
    Draw,
    Lose,
}

impl TryFrom<&str> for Strat {
    type Error = &'static str;

    fn try_from(char: &str) -> Result<Self, Self::Error> {
        match char {
            "X" => Ok(Strat::Lose),
            "Y" => Ok(Strat::Draw),
            "Z" => Ok(Strat::Win),
            _ => Err("Cannot be converted to Strat"),
        }
    }
}

fn main() {
    // load input
    let input = read_to_string("./input.txt").unwrap();

    let turns: Vec<&str> = input.split("\n").filter(|line| line.len() > 0).collect();

    let mut guessed_strat_score = 0;
    let mut correct_strat_score = 0;

    for turn in turns.iter() {
        let turn: Vec<&str> = turn.to_owned().split(" ").collect();
        let [first, second] = [
            turn.get(0).unwrap().to_owned(),
            turn.get(1).unwrap().to_owned(),
        ];
        let first_shape = Shape::try_from(first).unwrap();
        let guessed_second_shape = Shape::try_from(second).unwrap();

        guessed_strat_score += get_score(&first_shape, guessed_second_shape);

        let strat = Strat::try_from(second).unwrap();
        let correct_second_shape = get_correct_shape(&first_shape, strat);

        correct_strat_score += get_score(&first_shape, correct_second_shape);
    }

    println!("Guessed strat score: {}", guessed_strat_score);
    println!("Correct strat score: {}", correct_strat_score);
}

fn get_score(first: &Shape, second: Shape) -> u32 {
    return match (first, second) {
        (Shape::Rock, Shape::Rock) => 1 + 3,
        (Shape::Rock, Shape::Paper) => 2 + 6,
        (Shape::Rock, Shape::Scissors) => 3 + 0,
        (Shape::Paper, Shape::Rock) => 1 + 0,
        (Shape::Paper, Shape::Paper) => 2 + 3,
        (Shape::Paper, Shape::Scissors) => 3 + 6,
        (Shape::Scissors, Shape::Rock) => 1 + 6,
        (Shape::Scissors, Shape::Paper) => 2 + 0,
        (Shape::Scissors, Shape::Scissors) => 3 + 3,
    };
}

fn get_correct_shape(first_shape: &Shape, strat: Strat) -> Shape {
    return match (first_shape, strat) {
        (Shape::Rock, Strat::Win) => Shape::Paper,
        (Shape::Rock, Strat::Draw) => Shape::Rock,
        (Shape::Rock, Strat::Lose) => Shape::Scissors,
        (Shape::Paper, Strat::Win) => Shape::Scissors,
        (Shape::Paper, Strat::Draw) => Shape::Paper,
        (Shape::Paper, Strat::Lose) => Shape::Rock,
        (Shape::Scissors, Strat::Win) => Shape::Rock,
        (Shape::Scissors, Strat::Draw) => Shape::Scissors,
        (Shape::Scissors, Strat::Lose) => Shape::Paper,
    };
}
