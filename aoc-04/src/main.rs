use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() {
    part_2();
}

fn part_1() -> io::Result<()> {
    let file = File::open("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-04/input.txt")?;
    let reader = BufReader::new(file);
    
    let mut count = 0;
    for line in reader.lines() {
        let pairs = parse_pairs(&line.unwrap());
        let left = pairs.0;
        let right = pairs.1;

        if left.1 < left.0 || right.1 < right.0 {
            panic!("File is invalid");
        }

        if left.0 <= right.0 && left.1 >= right.1 {
            count += 1;
        } else if right.0 <= left.0 && right.1 >= left.1 {
            count += 1;
        }
    }

    println!("{}", count);
    Ok(())
}

fn part_2() -> io::Result<()> {
    let file = File::open("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-04/input.txt")?;
    let reader = BufReader::new(file);
    
    let mut count = 0;
    for line in reader.lines() {
        let pairs = parse_pairs(&line.unwrap());
        let left = pairs.0;
        let right = pairs.1;

        if left.1 < left.0 || right.1 < right.0 {
            panic!("File is invalid");
        }

        if left.0 <= right.0 && left.1 >= right.1 {
            count += 1;
        } else if right.0 <= left.0 && right.1 >= left.1 {
            count += 1;
        } else if left.1 >= right.0 && right.1 >= left.0 {
            count += 1;
        } else if right.1 >= left.0 && left.1 >= right.0 {
            count += 1;
        }
    }

    println!("{}", count);
    Ok(())
}
fn parse_pairs(line: &str) -> ((i32, i32), (i32, i32)) {
    let values: Vec<&str> = line.split(",").collect();
    let left: Vec<i32> = values[0].split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|val| val.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let right: Vec<i32> = values[1].split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|val| val.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    ((left[0],left[1]),(right[0],right[1]))    
}

