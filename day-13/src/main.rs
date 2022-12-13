use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    fs::read_to_string,
    str::FromStr,
};

#[derive(Eq, Clone)]
enum Packet {
    Single(u8),
    Many(Vec<Packet>),
}

// use the nice print for debug prints too
impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

// print it out nicely
impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Single(x) => write!(f, "{}", *x),
            Packet::Many(packets) => {
                write!(f, "[")?;
                for (idx, packet) in packets.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", packet)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Err(format!("The packet should start with '[': {}", s).into());
        }
        if !s.ends_with(']') {
            return Err(format!("The packet should end with ']': {}", s).into());
        }
        // remove start and end brackets
        let s = &s[1..s.len() - 1];
        let mut stack = vec![];

        let mut many = vec![];
        let mut single = None;

        for ch in s.bytes() {
            match ch {
                // ch is 0-9
                b'0'..=b'9' => {
                    single = Some(match single.take() {
                        None => ch - b'0',
                        Some(val) => val * 10 + (ch - b'0'),
                    })
                }
                // ch is ,
                b',' => {
                    if let Some(value) = single.take() {
                        many.push(Packet::Single(value));
                    }
                }
                // start new packet
                b'[' => {
                    stack.push((many, single));
                    many = vec![];
                    single = None;
                }
                // end current packet
                b']' => {
                    if let Some(value) = single.take() {
                        many.push(Packet::Single(value));
                    }
                    let packet = Packet::Many(many);
                    (many, single) = stack.pop().unwrap();
                    many.push(packet);
                }
                _ => panic!("unexpected character :{}", ch as char),
            }
        }

        if let Some(value) = single.take() {
            many.push(Packet::Single(value));
        }

        // outermost packet is always many
        Ok(Packet::Many(many))
    }
}

// impl equality and ordering for easy sorting and comparisons
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Single(x), Packet::Single(y)) => x.cmp(y),
            (Packet::Many(x), Packet::Many(y)) => x.cmp(y),
            (x @ Packet::Single(_), Packet::Many(y)) => std::slice::from_ref(x).cmp(y.as_slice()),
            (Packet::Many(x), y @ Packet::Single(_)) => x.as_slice().cmp(std::slice::from_ref(y)),
        }
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    // turn the input into Packets
    let mut packets: Vec<Packet> = input
        .lines()
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse::<Packet>().unwrap())
        .collect();

    // part 1
    // chunk packets into sets of two
    let results: Vec<usize> = packets
        .chunks(2)
        .enumerate()
        .filter_map(|(idx, chunk)| {
            if chunk[0].cmp(&chunk[1]) == Ordering::Less {
                return Some(idx + 1);
            }
            None
        })
        .collect();

    let total: usize = results.iter().sum();

    println!("{}", total);

    // part 2
    packets.push(Packet::Single(2));
    packets.push(Packet::Single(6));
    // thanks to ordering trait implementation!
    packets.sort_unstable();

    let mut answer = 1;
    let mut idx = 0;
    while idx < packets.len() {
        idx += 1;
        if let Packet::Single(2) = packets[idx - 1] {
            answer *= idx;
            break;
        }
    }
    while idx < packets.len() {
        idx += 1;
        if let Packet::Single(6) = packets[idx - 1] {
            answer *= idx;
            break;
        }
    }

    println!("{}", answer);
}
