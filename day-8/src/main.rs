use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let forest: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            let parts: Vec<u8> = line
                .split("")
                .filter_map(|i| {
                    if i.len() > 0 {
                        Some(i.to_string())
                    } else {
                        None
                    }
                })
                .map(|i| i.parse().unwrap())
                .collect();
            parts
        })
        .collect();

    // part 1
    let mut visible_count = 0;

    for (x, row) in forest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            let trees_to_left = &row[..y];
            let visible_from_left = is_largest(trees_to_left, tree);
            if visible_from_left {
                visible_count += 1;
                continue;
            }
            let trees_to_right = &row[(y + 1)..];
            let visible_from_right = is_largest(trees_to_right, tree);
            if visible_from_right {
                visible_count += 1;
                continue;
            }

            let column: Vec<u8> = forest.iter().map(|row| row[y]).collect();
            let trees_above = &column[..x];
            let visible_from_above = is_largest(trees_above, tree);
            if visible_from_above {
                visible_count += 1;
                continue;
            }
            let trees_below = &column[(x + 1)..];
            let visible_from_below = is_largest(trees_below, tree);
            if visible_from_below {
                visible_count += 1;
                continue;
            }
        }
    }

    println!("visble trees from outside of forest: {}", visible_count);

    // part 2
    let mut best_score = 0;

    for (x, row) in forest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            let trees_to_left = &row[..y];
            let visible_from_left = count_visible(trees_to_left, tree, true);

            let trees_to_right = &row[(y + 1)..];
            let visible_from_right = count_visible(trees_to_right, tree, false);

            let column: Vec<u8> = forest.iter().map(|row| row[y]).collect();

            let trees_above = &column[..x];
            let visible_from_above = count_visible(trees_above, tree, true);

            let trees_below = &column[(x + 1)..];
            let visible_from_below = count_visible(trees_below, tree, false);

            let score: u32 =
                visible_from_left * visible_from_right * visible_from_above * visible_from_below;

            if score > best_score {
                best_score = score;
            }
        }
    }

    println!("best scenic score: {}", best_score);
}

fn is_largest(row: &[u8], tree: &u8) -> bool {
    if row.len() == 0 {
        // this matches all the outer trees
        return true;
    }

    // is the tree bigger than everything in the row
    row.iter().all(|t| tree.gt(t))
}

fn count_visible(row: &[u8], tree: &u8, reverse: bool) -> u32 {
    if row.len() == 0 {
        return 0;
    }

    let mut count = 0;
    if reverse {
        for i in row.iter().rev() {
            count += 1;
            if i >= tree {
                break;
            }
        }
    } else {
        for i in row.iter() {
            count += 1;
            if i >= tree {
                break;
            }
        }
    }
    count
}
