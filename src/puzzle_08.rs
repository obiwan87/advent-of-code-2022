use std::collections::{HashMap, HashSet};
use crate::io;

pub(crate) fn aoc_puzzle_08_part_1() -> i32 {
    let forest = fill_grid();

    let columns = forest.get(0).unwrap().len();
    let rows = forest.len();

    println!("{} rows ", rows);
    println!("{} columns ", columns);

    let mut visible_trees = HashSet::<(i32, i32)>::new();

    // right
    {
        for (row_index, row) in forest.iter().enumerate() {
            let mut max = -1;
            for (col_index, tree) in row.iter().enumerate() {
                if *tree > max {
                    visible_trees.insert((row_index as i32, col_index as i32));
                    max = *tree;
                }
            }
        }
    }

    // left
    {
        for (row_index, row) in forest.iter().enumerate() {
            let mut max = -1;
            for (col_index, tree) in row.iter().enumerate().rev() {
                if *tree > max {
                    visible_trees.insert((row_index as i32, col_index as i32));
                    max = *tree;
                }
            }
        }
    }

    // down
    {
        for col_index in 0..columns {
            let mut max = -1;
            for (row_index, row) in forest.iter().enumerate() {
                let tree = *row.get(col_index).unwrap();
                if tree > max {
                    visible_trees.insert((row_index as i32, col_index as i32));
                    max = tree;
                }
            }
        }
    }

    // up
    {
        for col_index in 0..columns {
            let mut max = -1;
            for (row_index, row) in forest.iter().enumerate().rev() {
                let tree = *row.get(col_index).unwrap();
                if tree > max {
                    visible_trees.insert((row_index as i32, col_index as i32));
                    max = tree;
                }
            }
        }
    }

    visible_trees.len() as i32
}


pub(crate) fn aoc_puzzle_08_part_2() -> i32 {
    let grid = fill_grid();

    let mut max_scenic_score = 0;
    for (row_index, row) in grid.iter().enumerate() {
        if row_index == 0 || row_index == grid.len() - 1 { continue; }
        for (col_index, height) in row.iter().enumerate() {
            // right
            if col_index == 0 || col_index == row.len() - 1 { continue; }

            let mut visibility_right = 0;
            for neighbor_height in row.iter().skip(col_index + 1) {
                visibility_right += 1;
                if neighbor_height >= height {
                    break;
                }
            }

            let mut visibility_left = 0;
            for neighbor_height in row.iter().take(col_index  ).rev() {
                visibility_left += 1;
                if neighbor_height >= height {
                    break;
                }
            }

            let mut col = Vec::<i32>::new();
            for curr_row in grid.iter() {
                col.push(*curr_row.get(col_index).unwrap())
            }

            let mut visibility_down = 0;
            for neighbor_height in col.iter().skip(row_index + 1) {
                visibility_down += 1;
                if neighbor_height >= height {
                    break;
                }
            }

            let mut visibility_up = 0;
            for neighbor_height in col.iter().take(row_index  ).rev() {
                visibility_up += 1;
                if neighbor_height >= height {
                    break;
                }
            }

            let scenic_score = visibility_down * visibility_left * visibility_right * visibility_up;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
                println!("{}, {}: {} with scenic score {}", row_index, col_index, height, scenic_score);
            }
        }
    }
    max_scenic_score
}

fn fill_grid() -> Vec<Vec<i32>> {
    let data = io::load_data_to_str("data/08/input.txt");
    let mut forest = Vec::<Vec<i32>>::new();

    for line in data.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut row = Vec::<i32>::new();
        for c in line.chars() {
            let v = c.to_string().parse::<i32>().expect("Should have been an int");
            row.push(v);
        }
        forest.push(row);
    }
    forest
}