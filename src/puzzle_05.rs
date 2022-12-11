use regex::Regex;
use crate::io;

pub(crate) fn aoc_puzzle_05_part_1() -> String {
    let data = io::load_data_to_str("data/05/input.txt");
    let splits = data.split("\n\n").collect::<Vec<&str>>();

    let raw_stack = splits[0];

    let mut stacks = parse_stack(raw_stack);
    let moves = splits[1].split("\n");
    let re = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for move_ in moves {
        if move_.len() > 0 {
            let captures = re.captures(move_).unwrap();
            let amount = captures[1].parse::<i32>().unwrap();
            let from = captures[2].parse::<usize>().unwrap();
            let to = captures[3].parse::<usize>().unwrap();

            for k in 0..amount {
                let mut item: char;
                {
                    let mut f = stacks.get_mut(from - 1).unwrap();
                    item = f.pop().unwrap().clone();
                }

                {
                    let mut t = stacks.get_mut(to - 1).unwrap();
                    t.push(item);
                }
            }
        }
    }

    assert_eq!(splits.len(), 2);

    let mut result: String = "".to_string().to_owned();
    for stack in stacks {
        let c = stack.get(stack.len() - 1).unwrap();
        result = result + c.to_string().as_str();
    }

    result.to_string()
}

pub(crate) fn aoc_puzzle_05_part_2() -> String {
    let data = io::load_data_to_str("data/05/input.txt");
    let splits = data.split("\n\n").collect::<Vec<&str>>();

    let raw_stack = splits[0];

    let mut stacks = parse_stack(raw_stack);
    let moves = splits[1].split("\n");
    let re = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for move_ in moves {
        if move_.len() > 0 {
            let captures = re.captures(move_).unwrap();
            let amount = captures[1].parse::<i32>().unwrap();
            let from = captures[2].parse::<usize>().unwrap();
            let to = captures[3].parse::<usize>().unwrap();

            let mut buffer = Vec::<char>::new();
            for _ in 0..amount {
                let mut item: char;
                {
                    let mut f = stacks.get_mut(from - 1).unwrap();
                    item = f.pop().unwrap().clone();
                    buffer.push(item);
                }
            }

            for b in buffer.iter().rev() {
                {
                    let mut t = stacks.get_mut(to - 1).unwrap();
                    t.push(*b);
                }
            }
        }
    }

    assert_eq!(splits.len(), 2);

    let mut result: String = "".to_string().to_owned();
    for stack in stacks {
        let c = stack.get(stack.len() - 1).unwrap();
        result = result + c.to_string().as_str();
    }

    result.to_string()
}

fn parse_stack(raw_stack: &str) -> Vec<Vec<char>> {
    let mut parsed_stack = Vec::<Vec<char>>::new();
    let stack_lines = raw_stack.split("\n").collect::<Vec<&str>>();
    let last_line = *stack_lines.get(stack_lines.len() - 1).unwrap();
    let width = last_line.split("   ").last().unwrap().trim()
        .parse::<i32>().unwrap();

    for _ in 0..width {
        parsed_stack.push(Vec::<char>::new())
    }

    for i in (0..stack_lines.len() - 1).rev() {
        let line = stack_lines.get(i).unwrap();
        let mut k = 0;
        let mut col = 0;
        while k < line.len() {
            let mut element = line.chars().skip(k + 1).next().unwrap();

            if element != ' ' {
                let mut col_vector = parsed_stack.get_mut(col).unwrap();
                col_vector.push(element);
            }
            col += 1;
            k += 4
        }
    }

    parsed_stack
}