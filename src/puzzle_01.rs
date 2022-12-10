use crate::io;

pub fn aoc_puzzle_01() {
    let data = io::load_data_to_str("data/01/puzzle_input");
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

pub fn aoc_puzzle_01_part_2() -> i32 {
    let data = io::load_data_to_str("data/01/puzzle_input");
    let splits = data.split("\n\n");

    let mut all_calories = vec!();
    for elf_cargo in splits {
        let elf_items = elf_cargo.split("\n");
        let mut calories_sum: i32 = 0;
        for item in elf_items {
            if item.len() > 0 {
                let calories = item.to_string().parse::<i32>()
                    .expect(format!("Could not parse item with value {0}", item).as_str());
                calories_sum += calories;
            }
        }
        all_calories.push(-calories_sum);
    }

    all_calories.sort();
    let mut sum_top_3 = 0;
    for i in 0..3 {
        sum_top_3 += - all_calories[i];
    }

    sum_top_3
}

