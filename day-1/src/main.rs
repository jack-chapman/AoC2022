use std::fs::read_to_string;

fn main() {
    // load input
    let input = read_to_string("./input.txt").unwrap();

    // 1.
    let mut sums: Vec<u32> = Vec::new();
    let mut nums: Vec<u32> = Vec::new();
    input.split("\n").for_each(|line| {
        if let Ok(res) = line.parse::<u32>() {
            nums.push(res);
        } else {
            let sum: u32 = nums.iter().sum();
            sums.push(sum);
            nums.clear();
        }
    });
    sums.sort();
    sums.reverse();
    println!("largest: {:?}", sums.first().unwrap());

    // 2.
    let (top_three, _) = sums.split_at(3);
    println!("sum of top three: {:?}", top_three.iter().sum::<u32>());
}
