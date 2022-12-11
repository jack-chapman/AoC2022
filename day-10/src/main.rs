use std::fs::read_to_string;

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl TryFrom<String> for Command {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("noop") {
            Ok(Self::Noop)
        } else {
            let value: i32 = value
                .split("addx ")
                .filter(|i| i.len() > 0)
                .map(|i| {
                    let result: i32 = i.parse().unwrap();
                    result
                })
                .collect::<Vec<i32>>()
                .get(0)
                .unwrap()
                .to_owned();
            Ok(Self::Addx(value))
        }
    }
}

#[derive(Debug)]
struct Frame {
    command: Option<Command>,
}

impl Frame {
    fn new(command: Option<Command>) -> Self {
        Self { command }
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let mut elapsed_cycles: u32 = 0;
    let mut register_x = 1;

    let mut frames = Vec::new();

    for line in input.lines() {
        if let Ok(command) = Command::try_from(line.to_string()) {
            match command {
                Command::Noop => {
                    frames.push(Frame::new(None));
                }
                Command::Addx(value) => {
                    frames.push(Frame::new(None));
                    frames.push(Frame::new(Some(Command::Addx(value))));
                }
            }
        }
    }

    const CYCLES_OF_INTEREST: [u32; 6] = [20, 60, 100, 140, 180, 220];

    let mut total_signal = 0;

    let mut crt_position = 0;
    let mut screen: Vec<char> = vec![];

    for frame in frames {
        elapsed_cycles += 1;

        if CYCLES_OF_INTEREST.contains(&elapsed_cycles) {
            let signal_strength = elapsed_cycles as i32 * register_x;
            total_signal += signal_strength;
        }
        // crt here
        let register_range = (register_x - 1)..=(register_x + 1);
        if register_range.contains(&crt_position) {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if let Some(command) = frame.command {
            match command {
                Command::Addx(value) => {
                    register_x += value;
                }
                _ => {}
            }
        }

        if crt_position == 39 {
            crt_position = 0;
        } else {
            crt_position += 1;
        }
    }

    // part 1
    println!("total signal strength: {}", total_signal);

    // part 2
    screen.chunks(40).for_each(|chunk| {
        let s = String::from_iter(chunk.iter());
        println!("{}", s);
    });
}
