use std::collections::HashSet;
use crate::io;
use crate::puzzle_04::aoc_puzzle_04_part_1;

pub(crate) fn aoc_puzzle_06_part_1() -> usize {
    let data = io::load_data_to_str("data/06/input.txt");
    let mut window =Vec::<char>::new();

    let window_size: usize = 14;
    for c in data.chars().take(window_size) {
        window.push(c)
    }

    let mut cursor = window_size;
    if window.iter().collect::<HashSet<_>>().len() == window_size {
        return cursor;
    }
    cursor += 1;
    for c in data.chars().skip(window_size) {
        window.remove(0);
        window.push(c);
        if window.iter().collect::<HashSet<_>>().len() == window_size {
            return cursor;
        }
        cursor += 1;
    }
    return 0
}