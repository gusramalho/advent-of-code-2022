use std::collections::HashSet;
use std::fs;

const INPUT: &str = "/home/gustavo/Projects/rusty/advent-of-code-2022/aoc-08/input.txt";

fn main() {
    let trees = TreeGrid::from_input();
    let visible = trees.visible_trees();

    println!("{}", visible.len());

    let mut heigher_score = 0;

    for i in 0..trees.rows {
        for j in 0..trees.cols {
            let scenic_score = trees.scenic_score(i, j);
            if scenic_score > heigher_score {
                heigher_score = scenic_score;
            }
        }
    }

    println!("{}", heigher_score);
}

fn read_trees() -> Vec<Vec<u32>> {
    let content = fs::read_to_string(INPUT).expect("Error while reading file");
    let mut result: Vec<Vec<u32>> = vec![];
    let mut current_line: Vec<u32> = vec![];

    for c in content.chars() {
        if c == '\n' {
            result.push(current_line.clone());
            current_line.clear();
            continue;
        }
        current_line.push(c.to_digit(10).unwrap());
    }

    result
}

struct TreeGrid {
    grid: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl TreeGrid {
    fn from_input() -> Self {
        let grid = read_trees();
        let rows = grid.len();
        let cols = grid.first().unwrap().len();

        TreeGrid { grid, rows, cols }
    }

    fn height_of(&self, row: usize, column: usize) -> u32 {
        *self.grid.get(row).unwrap().get(column).unwrap()
    }

    fn visible_trees(&self) -> HashSet<(usize, usize)> {
        let mut result: HashSet<(usize, usize)> = HashSet::new();

        for i in 1..self.rows - 1 {
            let mut tallest_left: u32 = 0;
            let mut tallest_right: u32 = 0;

            for j in 0..self.cols {
                let left_height = self.height_of(i, j);
                let right_height = self.height_of(i, self.cols - j - 1);

                if i == 0 || left_height > tallest_left {
                    result.insert((i, j));
                    tallest_left = left_height;
                }

                if j == self.cols - 1 || right_height > tallest_right {
                    result.insert((i, self.cols - j - 1));
                    tallest_right = right_height;
                }
            }
        }

        for j in 1..self.cols - 1 {
            let mut tallest_top: u32 = 0;
            let mut tallest_bottom: u32 = 0;

            for i in 0..self.rows {
                let top_height = self.height_of(i, j);
                let bottom_height = self.height_of(self.rows - 1 - i, j);

                if i == 0 || top_height > tallest_top {
                    tallest_top = top_height;
                    result.insert((i, j));
                }

                if i == self.rows - 1 || bottom_height > tallest_bottom {
                    tallest_bottom = bottom_height;
                    result.insert((self.rows - 1 - i, j));
                }
            }
        }

        for i in 0..self.rows {
            result.insert((i, 0));
            result.insert((i, self.cols - 1));
        }
        for j in 0..self.cols {
            result.insert((0, j));
            result.insert((self.rows - 1, j));
        }
        result
    }

    fn scenic_score(&self, row: usize, column: usize) -> u32 {
        let mut up_view: u32 = 0;
        let mut right_view: u32 = 0;
        let mut down_view: u32 = 0;
        let mut left_view: u32 = 0;

        let tree_height = self.height_of(row, column);

        for i in (0..row).rev() {
            if i != row {
                up_view = up_view + 1;
            }

            if tree_height <= self.height_of(i, column) {
                break;
            }
        }

        for i in (row + 1)..self.rows {
            down_view = down_view + 1;

            if tree_height <= self.height_of(i, column) {
                break;
            }
        }

        for j in (0..column).rev() {
            if j != column {
                left_view = left_view + 1;
            }

            if tree_height <= self.height_of(row, j) {
                break;
            }
        }

        for j in (column + 1)..self.cols {
            right_view = right_view + 1;

            if tree_height <= self.height_of(row, j) {
                break;
            }
        }

        up_view * right_view * down_view * left_view
    }
}
