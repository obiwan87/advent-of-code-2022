use std::cmp::min;
use crate::io;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;


pub(crate) fn aoc_puzzle_02_part_1() -> i32 {

    // rock 1, paper 2, scissors 3
    // I win if 1:3, 2:1, 3:2
    let input = io::load_data_to_str("data/02/input.txt");
    let lines = input.split("\n");

    let mut score = 0;
    for line in lines {
        let signs = line.split(" ").collect::<Vec<&str>>();
        if signs.len() == 2 {
            let opponent = signs[0].chars().nth(0).unwrap();
            let mine = signs[1].chars().nth(0).unwrap();

            match (opponent, mine) {
                ('A', 'X') => { score += DRAW + ROCK; }
                ('A', 'Y') => { score += WIN + PAPER; }
                ('A', 'Z') => { score += LOSS + SCISSORS; }
                ('B', 'X') => { score += LOSS + ROCK; }
                ('B', 'Y') => { score += DRAW + PAPER; }
                ('B', 'Z') => { score += WIN + SCISSORS; }
                ('C', 'X') => { score += WIN + ROCK; }
                ('C', 'Y') => { score += LOSS + PAPER; }
                ('C', 'Z') => { score += DRAW + SCISSORS; }
                _ => { eprintln!("Illegal state") }
            }
        }
    }
    score
}

pub(crate) fn aoc_puzzle_02_part_2() -> i32 {

    // rock 1, paper 2, scissors 3
    // I win if 1:3, 2:1, 3:2
    let input = io::load_data_to_str("data/02/input.txt");
    let lines = input.split("\n");

    let mut score = 0;
    for line in lines {
        let signs = line.split(" ").collect::<Vec<&str>>();
        if signs.len() == 2 {
            let opponent = signs[0].chars().nth(0).unwrap();
            let mine = signs[1].chars().nth(0).unwrap();
            // X = lose, Y = draw, Z = win
            match (opponent, mine) {
                ('A', 'X') => { score += LOSS + SCISSORS; }
                ('A', 'Y') => { score += DRAW + ROCK; }
                ('A', 'Z') => { score += WIN + PAPER; }
                ('B', 'X') => { score += LOSS + ROCK; }
                ('B', 'Y') => { score += DRAW + PAPER; }
                ('B', 'Z') => { score += WIN + SCISSORS; }
                ('C', 'X') => { score += LOSS  + PAPER; }
                ('C', 'Y') => { score += DRAW + SCISSORS; }
                ('C', 'Z') => { score += WIN + ROCK; }
                _ => { eprintln!("Illegal state") }
            }
        }
    }
    score
}