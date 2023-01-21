use std::fs;
use std::collections::HashMap;
// A X - Rock 1 
// B Y - Paper 2 
// C Z - Scissor 3
// Lost 0 | Draw 3 | Win 6
// 
//


fn main() {
    part_1();
    part_2();


}
fn part_1() {
    let content = fs::read_to_string("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-02/input.txt")
            .expect("Failed to read file");

    let matches: Vec<&str> = content.split("\n")
        .collect::<Vec<&str>>();

    let mut points = 0;
    

    let map = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);


    let option_map = HashMap::from([
        ("X", "A"),
        ("Y", "B"),
        ("Z", "C")
    ]);

    let win_map = HashMap::from([
        ("A", "C"),
        ("B", "A"),
        ("C", "B")
    ]);

    for m in matches.iter() {
        if !m.is_empty() {
            let splitted: Vec<&str> = m.split(" ").collect();

            let opponent = splitted[0];
            let me = splitted[1];

            let my_option = option_map[me];

            if win_map[my_option] == opponent {
                points += 6;
            } else if my_option == opponent {
                points += 3;
            }

            points += map[me];
        }
    }

    println!("Hello, world! {}", points);
}

fn part_2() {
    let content = fs::read_to_string("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-02/input.txt")
            .expect("Failed to read file");

    let matches: Vec<&str> = content.split("\n")
        .collect::<Vec<&str>>();

    let mut points = 0;
    
// X - lose, Y - draw, Z - win
//
    let map = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3) 
    ]);

    let option_map = HashMap::from([
        ("X", HashMap::from([("A", "C"), ("B", "A"), ("C", "B")])),
        ("Y", HashMap::from([("A", "A"), ("B", "B"), ("C", "C")])),
        ("Z", HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]))
    ]);

    let win_map = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6)
    ]);

    for m in matches.iter() {
        if !m.is_empty() {
            let splitted: Vec<&str> = m.split(" ").collect();

            let opponent = splitted[0];
            let me = splitted[1];

            let my_option = option_map[me][opponent];

            points += win_map[me];
            points += map[my_option];
        }
    }

    println!("Hello, world! {}", points);
}
