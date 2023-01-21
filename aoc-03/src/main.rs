use std::fs;

fn main() {
    //part_1();
    part_2();
}

fn part_1() {
    let content =
        fs::read_to_string("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-03/input.txt")
            .expect("Failed to read file");

    let rucksacks: Vec<&str> = content.split("\n")
        .filter(|line| !line.is_empty()).collect();

    let mut priority_sum: i32 = 0;

    for rucksack in rucksacks.iter() {
        let wrong_item = find_wrong_item(rucksack);
        priority_sum += priority_of(wrong_item);
    }

    println!("{}", priority_sum);
}

fn part_2() {
    let content =
        fs::read_to_string("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-03/input.txt")
            .expect("Failed to read file");

    let rucksacks: Vec<&str> = content.split("\n")
        .filter(|line| !line.is_empty()).collect();

    let mut priority_sum: i32 = 0;

    for (idx, rucksack) in rucksacks.iter().enumerate() {
        if (idx + 1) % 3 == 0 {
            let badge = find_bagde(rucksack, rucksacks[idx - 1], rucksacks[idx - 2]);
            priority_sum += priority_of(badge);
        }
    }

    println!("{}", priority_sum);
}

fn find_wrong_item(rucksack: &str) -> char {
    let half_idx = rucksack.len() / 2;

    let first_half = &rucksack[..half_idx];
    let last_half = &rucksack[half_idx..rucksack.len()];

    for c in first_half.chars() {
        if last_half.contains(c) {
            return c;
        }
    }

    panic!("Rucksack items are correct");
}

fn priority_of(item: char) -> i32 {
    let int_value = item as i32;

    if int_value >= 97 {
        return int_value - 96;
    }

    int_value - 38
}

fn find_bagde(rs_1: &str, rs_2: &str, rs_3: &str) -> char {
    for c in rs_1.chars() {
        if rs_2.contains(c) && rs_3.contains(c) {
            return c;
        }
    }

    panic!("There is no badge")
}
