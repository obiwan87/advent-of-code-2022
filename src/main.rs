use std::fs;
use std::path::Path;

fn main() {
    aoc_puzzle_01()
}

fn aoc_puzzle_01() {
    let data = load_data_to_str("data/01/puzzle_input");
    let splits = data.split("\n\n");

    let mut calories_max = 0;
    for elf_cargo in splits {
        let elf_items = elf_cargo.split("\n");
        let mut calories_sum: u32 = 0;
        for item in elf_items {
            if item.len() > 0 {
                let calories = item.to_string().parse::<u32>()
                    .expect(format!("Could not parse item with value {0}", item).as_str());
                calories_sum += calories;
            }
        }
        if calories_max < calories_sum {
            calories_max = calories_sum;
        }
    }

    println!("{}", calories_max);
}

fn load_data_to_str(path: &str) -> String {
    return fs::read_to_string(path).expect("Could not read from file");
}
