use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = env::args().nth(1).expect("Missing input file path");
    let file = File::open(input_file).expect("Failed to open input file");
    let buf_read = BufReader::new(file);

    let mut joltage = 0;

    for raw_line in buf_read
        .lines()
        .map(|v| v.expect("Failed to read input file"))
    {
        let line = raw_line.chars().collect::<Vec<_>>();
        let mut left_largest_digit = '0';
        let mut right_largest_digit = '0';

        for idx in 0..line.len() {
            let c = line[idx];
            if (c > left_largest_digit) && (idx != line.len() - 1) {
                // If this is now the largest new digit, we don't care about what came before.
                // We are also not the last digit.
                left_largest_digit = c;
                right_largest_digit = line[idx + 1];
            } else {
                if c > right_largest_digit {
                    right_largest_digit = c;
                }
            }
        }

        let curr_joltage = format!("{}{}", left_largest_digit, right_largest_digit)
            .parse::<u32>()
            .expect("Invalid number");
        //println!("{}: {}", raw_line, curr_joltage);

        joltage += curr_joltage;
    }

    println!("{}", joltage);
}
