use crate::io;

pub(crate) fn aoc_puzzle_04_part_1() -> i32 {
    let data = io::load_data_to_str("data/04/input.txt");
    let lines = data.split("\n");

    let mut sum = 0;
    for line in lines {
        let ranges = line.split(",").collect::<Vec<&str>>();
        if ranges.len() == 2 {
            let a = parse_range(ranges[0]);
            let b = parse_range(ranges[1]);

            if is_subset_sym(a, b) {
                sum += 1
            }
        }
    }
    sum
}

pub(crate) fn aoc_puzzle_04_part_2() -> i32 {
    let data = io::load_data_to_str("data/04/input.txt");
    let lines = data.split("\n");

    let mut sum = 0;
    for line in lines {
        let ranges = line.split(",").collect::<Vec<&str>>();
        if ranges.len() == 2 {
            let a = parse_range(ranges[0]);
            let b = parse_range(ranges[1]);

            if intersects(a, b) {
                sum += 1
            }
        }
    }
    sum
}

fn parse_range(a: &str) -> (i32, i32) {
    let m = a.split("-").collect::<Vec<&str>>();
    assert_eq!(m.len(), 2);
    return (m[0].to_string().parse::<i32>().unwrap(), m[1].to_string().parse::<i32>().unwrap());
}

fn is_subset(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 >= b.0 && a.0 <= b.1 && a.1 <= b.1
}

fn is_subset_sym(a: (i32, i32), b: (i32, i32)) -> bool {
    is_subset(a, b) || is_subset(b, a)
}

fn intersects(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 >= b.0 && a.0 <= b.1 ||
        a.1 >= b.0 && a.1 <= b.1 ||
        b.1 >= a.0 && b.1 <= a.1
}