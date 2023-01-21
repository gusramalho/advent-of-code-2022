use std::fs;

const SIZE: usize = 14; // 4 for Part 1

fn main() {
    let content = fs::read_to_string("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-06/input.txt")
        .expect("Failed to read file");

    let mut buffer: [Option<char>; SIZE] = [None; SIZE];

    for (idx, c) in content.chars().enumerate() {
        let pos = idx % SIZE;
        buffer[pos] = Some(c);

        if all_different(&buffer) {
            println!("{}", idx + 1);
            return;
        }
    }

    println!("Hello, world!");
}

fn all_different(arr: &[Option<char>]) -> bool {
    let values: Vec<char> = arr.iter().filter(|it| it.is_some())
        .map(|it| it.unwrap())
        .collect();

    if values.len() != SIZE {
        return false;
    }

    let mut sub_vec: Vec<char> = vec![];

    for c in values.iter() {
        if sub_vec.contains(c) {
            return false;
        }

        sub_vec.push(*c);
    }

    true
}
