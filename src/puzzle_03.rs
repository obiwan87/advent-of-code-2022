use crate::io;

pub(crate) fn aoc_puzzle_03_part_1() -> i64 {
    let data = io::load_data_to_str("data/03/input.txt");
    let lines = data.split("\n");
    let mut total_sum = 0;
    for line in lines {
        let length = line.len();

        let mut left: i64 = 0;
        let mut right: i64 = 0;
        let chars = line.chars().collect::<Vec<char>>();

        for i in 0..length / 2 {
            left |= 1 << (calc_prio(chars[i]) - 1);
            right |= 1 << (calc_prio(chars[i + length / 2]) - 1);
        }

        let mut partial_sum = 0;
        let mut both = left & right;
        for i in 1..53 {
            partial_sum += (both & 1) * i;
            both >>= 1;
        }
        total_sum += partial_sum;
    }
    total_sum
}

pub(crate) fn aoc_puzzle_03_part_2() -> u64 {
    let data = io::load_data_to_str("data/03/input_2.txt");
    let lines = data.split("\n");
    let mut common_items: u64 = 0;

    let mut sum: u64 = 0;
    for (i, line) in lines.enumerate() {
        if i % 3 == 0 {
            // this is the first member of the group
            common_items = 0xFFFFFFFFFFFFFFFF;
        }

        let chars = line.chars().collect::<Vec<char>>();
        let mut curr_items: u64 = 0;
        for char in chars {
            curr_items |= 1 << (calc_prio(char) - 1);
        }

        common_items &= curr_items;

        if (i + 1) % 3 == 0 {
            // Last member of the group

            // This loop calculates log2(x) + 1
            for j in 1..53 {
                let mut is_bit_set = (common_items & 1);
                sum += is_bit_set * j;
                common_items >>= 1;
            }
        }
    }
    sum
}


fn calc_prio(c: char) -> u8 {
    if c.is_uppercase() {
        return c as u8 - 'A' as u8 + 27;
    }
    return c as u8 - 'a' as u8 + 1;
}