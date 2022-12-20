use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let list: Vec<(usize, i32)> = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .enumerate()
        .collect();

    println!("total part 1: {}", part_1(&list));

    println!(
        "total part 2: {}",
        part_2(&list.iter().map(|(i, v)| (*i, *v as i64)).collect())
    );
}

fn mix(list: &mut Vec<(usize, i32)>) {
    let len = list.len();
    for i in 0..len {
        let index = list.iter().position(|(idx, _)| i == *idx).unwrap();
        let (_, value) = list[index];
        let new_index = index as i32 + value;
        let new_index = new_index.rem_euclid((len - 1) as i32);
        let val = list.remove(index);
        list.insert(new_index as usize, val);
    }
}

// need to understand generics better to remove this
fn mix_i64(list: &mut Vec<(usize, i64)>) {
    let len = list.len();
    for i in 0..len {
        let index = list.iter().position(|(idx, _)| i == *idx).unwrap();
        let (_, value) = list[index];
        let new_index = index as i64 + value;
        let new_index = new_index.rem_euclid((len - 1) as i64);
        let val = list.remove(index);
        list.insert(new_index as usize, val);
    }
}

fn part_1(list: &Vec<(usize, i32)>) -> i32 {
    let mut list = list.clone();

    mix(&mut list);

    let values: Vec<i32> = list.iter().map(|(_, v)| *v).collect();

    let zero_position = values.iter().position(|n| *n == 0).unwrap();
    let len = list.len();
    let results = [
        values[(zero_position + 1000) % len],
        values[(zero_position + 2000) % len],
        values[(zero_position + 3000) % len],
    ];

    results.iter().sum::<i32>()
}

fn part_2(list: &Vec<(usize, i64)>) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;
    let mut list = list.clone();
    let zero_index = list.iter().position(|(_, v)| *v == 0).unwrap();
    list = list.iter().map(|(i, v)| (*i, v * DECRYPTION_KEY)).collect();
    let zero_index_value = list[zero_index].1;
    for _ in 0..10 {
        mix_i64(&mut list);
    }

    let values: Vec<i64> = list.iter().map(|(_, v)| *v).collect();
    let zero_position = values.iter().position(|n| *n == zero_index_value).unwrap();
    let len = list.len();

    let results = [
        values[(zero_position + 1000) % len],
        values[(zero_position + 2000) % len],
        values[(zero_position + 3000) % len],
    ];

    results.iter().sum::<i64>()
}
