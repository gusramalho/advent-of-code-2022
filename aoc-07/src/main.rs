use std::fs::File as RFile;
use std::io::{self, BufReader};
use std::collections::HashMap;
use std::io::prelude::*;
use regex::Regex;

const TOTAL_DISK_SPACE: u32 = 70000000;
const REQUIRED_SPACE: u32 = 30000000;

#[derive(Debug)]
struct File {
    name: String,
    size: u32
}

impl File {
    fn new(filename: String, size: u32) -> File {
        File { name: filename, size }
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}

fn main() -> io::Result<()> {
    let file = RFile::open("/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-07/input.txt")?;
    let file = BufReader::new(file);

    let mut current_dir: Vec<String> = vec![];
    let mut current_listing: String = "".to_string();

    let mut dir_files: HashMap<String, Vec<File>> = HashMap::new();
    let mut subdirs: HashMap<String, Vec<String>> = HashMap::new();
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let cd_re = Regex::new(r"(\$ cd (?P<directory>(\w|\W)+))").unwrap();
    let ls_re = Regex::new(r"(\$ ls)").unwrap();
    let ls_dir_re = Regex::new(r"(dir (?P<directory>\w+))").unwrap();
    let ls_file_re = Regex::new(r"((?P<size>\d+) (?P<name>\w+.\w+))").unwrap();

    for line in file.lines() {
        let line = line.unwrap();

        match cd_re.captures(&line).and_then(|cap| { 
            cap.name("directory").map(|dir| dir.as_str())
        }) {
            Some(dir) => {
                if dir == "/" {
                    current_dir.clear();
                } else if dir == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(dir.to_string());
                }

                continue;
            }
            None => {  } 
        }

        match ls_re.captures(&line) {
            Some(_) => {
                current_listing = to_path(&current_dir);
                continue;
            }
            _ => {}
        }

        match ls_dir_re.captures(&line).and_then(|cap| {
            cap.name("directory").map(|dir| dir.as_str())
        }) {
            Some(dir) => {
                let new_dir = format!("{}{}/", current_listing, dir);

                if let Some(value) = subdirs.get_mut(&current_listing) {
                    if !value.contains(&new_dir) {
                        value.push(new_dir);
                    }
                } else {
                    subdirs.insert(current_listing.clone(), vec![new_dir]);
                }
                continue;
            }
            _ => {}
        }

        match ls_file_re.captures(&line).and_then(|cap| {
            let name = cap.name("name").map(|name| name.as_str()).unwrap();
            let size = cap.name("size").map(|size| size.as_str()).unwrap();

            Some(File::new(name.to_string(), size.parse::<u32>().unwrap()))

        }) {
            Some(file) => {
                if let Some(value) = dir_files.get_mut(&current_listing) {
                    if !value.contains(&file) {
                        value.push(file);
                    }

                } else {
                    dir_files.insert(current_listing.clone(), vec![file]);
                }
            }
            _ => {
                println!("not match");
            }
        }
    }

    let total_size = calculate_dir_size(&("/".to_string()), &dir_files, &subdirs, &mut dir_sizes);

    let result_part_1: u32 = dir_sizes.values()
        .filter(|it| *it <= &100000)
        .sum();

    let available_space = TOTAL_DISK_SPACE - total_size;

    println!("Part 1: {}", result_part_1);
    println!("available_space: {}/{}", available_space, TOTAL_DISK_SPACE);

    let required = REQUIRED_SPACE - available_space;

    println!("required: {}/{}", required, TOTAL_DISK_SPACE);

    let result_part_2: u32 = *dir_sizes.values()
        .filter(|it| *it >= &required)
        .min()
        .unwrap();

    println!("Part 2: {}", result_part_2);

    Ok(())
}

fn to_path(dirs: &Vec<String>) -> String {
    let mut result = String::from("");

    if (*dirs).is_empty() {
        return "/".to_string();
    }

    for dir in (*dirs).iter() {
        result = format!("{}{}/", result, dir);
    }

    return format!("/{}", result);
}

fn calculate_dir_size(
    dir_name: &String,
    dir_files: &HashMap<String, Vec<File>>,
    subdirs: &HashMap<String, Vec<String>>,
    dir_sizes: &mut HashMap<String, u32>
) -> u32 {
    if dir_sizes.contains_key(dir_name) {
        return *dir_sizes.get(dir_name).unwrap();
    }

    let mut size = 0;

    if let Some(dirs) = subdirs.get(dir_name) {
        for dir in dirs.iter() {
            size += calculate_dir_size(&dir, &dir_files, &subdirs, dir_sizes);
        }
    }

    if let Some(files) = dir_files.get(dir_name) {
        for file in files.iter() {
            size += file.size;
        }
    }

    dir_sizes.insert(dir_name.to_string(), size);

    return size;
}

