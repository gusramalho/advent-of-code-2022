use std::fs;


fn main() {
    let contents = fs::read_to_string("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-01/src/input.txt")
        .expect("Failed to read file");

    let values: Vec<&str> = contents.split("\n\n")
        .collect();

    let mut values_int: Vec<i32> = values.iter()
       .map(|val| 
           val.split("\n").collect::<Vec<&str>>()
             .iter()
             .filter(|it| it.len() != 0)
             .map(|it| {
                 println!("parsing({})", it);
                 it.parse::<i32>().unwrap()
             })
             .sum()
       )
       .collect();

   values_int.sort_by(|a, b| b.cmp(a));
       
   println!("blabla {}", values_int[0]);

   println!("top 3 {}", values_int[0] + values_int[1] + values_int[2]);

}
