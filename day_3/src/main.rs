use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const JOLTAGE_SIZE: usize = 12;

fn main() {
    let input_file = env::args().nth(1).expect("Missing input file path");
    let file = File::open(input_file).expect("Failed to open input file");
    let buf_read = BufReader::new(file);

    let mut joltage: u64 = 0;

    for raw_line in buf_read
        .lines()
        .map(|v| v.expect("Failed to read input file"))
    {
        let line = raw_line.chars().collect::<Vec<_>>();
        let mut curr_joltage_str = String::with_capacity(JOLTAGE_SIZE);
        let mut cursor = 0;

        for valid_left in (0..JOLTAGE_SIZE).rev().map(|v| v + 1) {
            let next_largest_valid = find_next_largest_valid(&line, cursor, valid_left);
            cursor = next_largest_valid + 1;
            curr_joltage_str.push(line[next_largest_valid]);
        }

        joltage += curr_joltage_str
            .parse::<u64>()
            .expect("Invalid number during joltage parse");
    }

    println!("{}", joltage);
}

fn find_next_largest_valid(digits: &[char], starting_idx: usize, valid_left: usize) -> usize {
    let mut next_largest_valid = starting_idx;
    let remaining_length = digits.len() - starting_idx;
    if valid_left <= remaining_length {
        for i in (0..(remaining_length - valid_left + 1)).map(|v| v + starting_idx) {
            if digits[i] > digits[next_largest_valid] {
                next_largest_valid = i;
            }
        }
    }
    return next_largest_valid;
}
