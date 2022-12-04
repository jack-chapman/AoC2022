use std::{fs::read_to_string, ops::RangeInclusive};

#[derive(Copy, Clone, Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn range(self) -> RangeInclusive<u32> {
        self.start..=self.end
    }
}

impl TryFrom<&str> for Section {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split("-").collect();
        let a = parts.get(0).unwrap().to_owned();
        let b = parts.get(1).unwrap().to_owned();
        let start: u32 = a.parse().unwrap();
        let end: u32 = b.parse().unwrap();
        Ok(Self { start, end })
    }
}

#[derive(Clone, Copy)]
struct Group {
    a: Section,
    b: Section,
}

impl Group {
    fn contains(self) -> bool {
        let a_range = self.a.range();
        let b_range = self.b.range();
        let a_contains_b = a_range.contains(&self.b.start) && a_range.contains(&self.b.end);
        let b_contains_a = b_range.contains(&self.a.start) && b_range.contains(&self.a.end);
        a_contains_b || b_contains_a
    }

    fn overlaps(self) -> bool {
        self.a.start <= self.b.end && self.b.start <= self.a.end
    }
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    // 1. split input into rows
    let rows: Vec<&str> = input.split("\n").filter(|i| i.len() > 0).collect();

    // 2. convert rows into groups of sections
    let groups: Vec<Group> = rows
        .into_iter()
        .map(|row| {
            let section_inputs: Vec<&str> = row.split(",").collect();
            let sections: Vec<Section> = section_inputs
                .into_iter()
                .map(|i| Section::try_from(i).unwrap())
                .collect();
            let a = sections.get(0).unwrap();
            let b = sections.get(1).unwrap();
            Group { a: *a, b: *b }
        })
        .collect();

    // 3. count how many contain themselves
    let mut contains_count = 0;
    for group in &groups {
        if group.contains() {
            contains_count += 1;
        }
    }

    // 4. print the total containing groups to solve part 1
    println!("Total containing groups: {}", contains_count);

    // part 2
    // 5. count how many overlap
    let mut overlaps_count = 0;
    for group in groups {
        if group.overlaps() {
            overlaps_count += 1;
        }
    }

    // 6. print the total overlapping groups to solve part 2
    println!("Total overlapping groups: {}", overlaps_count);
}
