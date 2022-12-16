use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter::Chain,
    slice::Iter,
};

#[derive(Debug)]
struct Room<'a>(u16, Vec<&'a str>);

fn main() {
    const TIME_1: usize = 30;

    let input = read_to_string("./input.txt").unwrap();
    let mut rooms = HashMap::<&str, Room>::new();
    for room in input.lines().map(|l| {
        let sides: Vec<&str> = l.split_terminator("valve").collect();
        let terms: Vec<&str> = sides[0].split_terminator(' ').collect();
        let name = terms[1];
        let pressure = terms[4].split_terminator(['=', ';']).collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let connections = sides[1]
            .trim_start_matches("s ")
            .trim()
            .split_terminator(", ")
            .collect();
        (name, Room(pressure, connections))
    }) {
        rooms.insert(room.0, room.1);
    }
    type ChoiceA<'a> = (u32, u16, &'a str, HashMap<&'a str, bool>);
    let mut choices = HashMap::<(u16, &str), ChoiceA>::new();
    let first_choice: ChoiceA = (0, 0, "AA", HashMap::new());
    choices.insert((0, "AA"), first_choice);

    for min in 0..TIME_1 {
        println!("{} - choices {}", min, choices.len());
        let mut new_choices = HashMap::new();
        for (_, choice) in &choices {
            let valid_rooms = &rooms.get(choice.2).unwrap().1;
            let new_time = choice.0 + 1;
            let mut new_release = choice.1;
            for v in valid_rooms {
                let v = *v;
                let new_state = choice.3.clone();
                let new_choice: ChoiceA = (new_time, new_release, v, new_state);
                new_choices.insert((new_release, v), new_choice);
            }
            let new_room = choice.2;
            if !choice.3.contains_key(&new_room) {
                new_release += &rooms.get(&choice.2).unwrap().0 * (TIME_1 - min - 1) as u16;
                let mut new_state = choice.3.clone();
                new_state.insert(new_room, true);
                let new_choice: ChoiceA = (new_time, new_release, new_room, new_state);
                new_choices.insert((new_release, new_room), new_choice);
            }
        }
        choices = new_choices;
    }

    let mut m = 0;
    for (_, choice) in choices {
        if choice.1 > m {
            m = choice.1;
        }
    }

    println!("part 1: {}", m);

    type ChoiceB<'a> = (u16, &'a str, HashSet<&'a str>, &'a str);
    let mut choices = HashMap::<(u16, &str, &str), ChoiceB>::new();
    let first_choice: ChoiceB = (0, "AA", HashSet::new(), "AA");
    choices.insert((0, "AA", "AA"), first_choice);

    const TIME_2: usize = 26;
    const TIME_DELTA: usize = 1;

    'outer: for min in 0..TIME_2 {
        println!("{} - choices {}", min, choices.len());
        let mut new_choices = HashMap::new();
        for (_, choice) in &choices {
            let i_stay = [choice.1];
            let e_stay = [choice.3];
            let valid: Chain<Iter<_>, Iter<_>> =
                rooms.get(choice.1).unwrap().1.iter().chain(i_stay.iter());
            let valid_e: Chain<Iter<_>, Iter<_>> =
                rooms.get(choice.3).unwrap().1.iter().chain(e_stay.iter());

            if choice.2.len() == rooms.len() {
                break 'outer;
            }

            for v in valid {
                let this_valid_e = valid_e.clone();
                for e in this_valid_e {
                    let mut new_release = choice.0;
                    let new_room = *v;
                    let new_e_room = *e;
                    let mut new_state = choice.2.clone();
                    let new_choice;

                    if new_e_room == choice.3 && new_room == choice.1 {
                        if !new_state.contains(new_room) || !choice.2.contains(new_e_room) {
                            new_release = choice.0;
                            if !new_state.contains(new_room) {
                                new_release += &rooms.get(new_room).unwrap().0
                                    * (TIME_2 - min - TIME_DELTA) as u16;
                                new_state.insert(new_room);
                            }
                            if !new_state.contains(new_e_room) {
                                new_release += &rooms.get(new_e_room).unwrap().0
                                    * (TIME_2 - min - TIME_DELTA) as u16;
                                new_state.insert(new_e_room);
                            }
                        }
                    } else if new_room == choice.1 {
                        new_release = choice.0;
                        if !new_state.contains(new_room) {
                            new_release += &rooms.get(new_room).unwrap().0
                                * (TIME_2 - min - TIME_DELTA) as u16;
                            new_state.insert(new_room);
                        }
                    } else if new_e_room == choice.3 {
                        new_release = choice.0;
                        if !new_state.contains(new_e_room) {
                            new_release += &rooms.get(new_e_room).unwrap().0
                                * (TIME_2 - min - TIME_DELTA) as u16;
                            new_state.insert(new_e_room);
                        }
                    }
                    new_choice = (new_release, new_room, new_state, new_e_room);
                    if !new_choices.contains_key(&(new_release, new_room, new_e_room))
                        && !new_choices.contains_key(&(new_release, new_e_room, new_room))
                    {
                        new_choices.insert((new_release, new_room, new_e_room), new_choice);
                    }
                }
            }
        }
        choices = new_choices;
    }

    let mut m = 0;
    for (_, choice) in choices {
        if choice.0 > m {
            m = choice.0;
        }
    }

    println!("part 2: {}", m);
}
