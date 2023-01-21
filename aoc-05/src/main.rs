use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-05/input.txt")?;
    let mut f = BufReader::new(f);
    let mut crates = read_crates(&mut f);
    
    let commands = Command::read_commands(&mut f); 

    let cargo_mover: CargoMover9001 = CargoMover::new();

    for command in commands.iter() {
        cargo_mover.execute(command, &mut crates);
    }

    for i in crates.iter() {
        print!("{}", i.last().unwrap());
    }

    Ok(())
}

struct Command {
    crates: usize,
    from: usize,
    to: usize
}

impl Command {

    fn new(crates: usize, from: usize, to: usize) -> Command {
        Command {
            crates, from, to
        }
    }

    fn read_commands(br: &mut BufReader<File>) -> Vec<Command> {
        let mut result: Vec<Command> = vec![];

        for line in br.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let line = line.split(" ");
            let line = line.collect::<Vec<&str>>();

            let command = Command::new(
                line[1].parse::<usize>().unwrap(),
                line[3].parse::<usize>().unwrap() - 1,
                line[5].parse::<usize>().unwrap() - 1,
            );

            result.push(command);
        }

        return result;
    }

    fn execute(&self, crates: &mut Vec<Vec<char>>) {
        for _ in 0..self.crates {
            crates[self.from].pop().and_then(|x| {
                crates[self.to].push(x);
                Some(x)
            });
        }
    }
}

trait CargoMover {
    fn new() -> Self;

    fn execute(&self, command: &Command, crates: &mut Vec<Vec<char>>);
}

struct CargoMover9000;

impl CargoMover for CargoMover9000 {
    fn new() -> CargoMover9000 {
        CargoMover9000 {}
    }

    fn execute(&self, command: &Command, crates: &mut Vec<Vec<char>>) {
        for _ in 0..command.crates {
            crates[command.from].pop().and_then(|x| {
                crates[command.to].push(x);
                Some(x)
            });
        }
    }
}

struct CargoMover9001; 

impl CargoMover for CargoMover9001 {
    fn new() -> CargoMover9001 {
        CargoMover9001 {}
    }

    fn execute(&self, command: &Command, crates: &mut Vec<Vec<char>>) {
        let from_size = crates[command.from].len();

        let moved: Vec<char> = crates[command.from].drain(from_size - command.crates..from_size)
            .collect();

        for x in moved.iter() {
            crates[command.to].push(*x);
        }
    }
}

fn read_crates(br: &mut BufReader<File>) -> Vec<Vec<char>> {
    let mut crates_buf: Vec<String> = vec![];
    let mut result: Vec<Vec<char>> = vec![];

    loop {
        let mut current: String = String::from("");
        br.read_line(&mut current);

        if !current.contains("[") {
            for (idx, c) in current.chars().enumerate() {
                if c.is_numeric() {
                    let mut crate_stack: Vec<char> = vec![];

                    for line in crates_buf.iter().rev() {
                        line.chars().nth(idx).and_then(|x| {
                            if x.is_alphanumeric() {
                                crate_stack.push(x)
                            }
                            Some(x)
                        });
                    }

                    result.push(crate_stack);
                }
            }

            return result;
        }

        crates_buf.push(current); 
    }
}
