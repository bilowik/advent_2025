use std::fs::File;
use std::io::{BufReader, BufRead};



fn main() {
    let path = std::env::args().nth(1).expect("No path specified");
    let f = File::open(&path).expect("Failed to open specified file");
    let mut f_reader = BufReader::new(f);
    let mut buf = String::new();
    let mut eof = false;
    let mut value = 50;
    let mut prev_value;
    let mut zero_count = 0;

    while !eof {
        f_reader.read_line(&mut buf).expect("Error reading file.");

        if buf.is_empty() {
            eof = true;
        }
        else {
            let change = parse_line(&buf);
            prev_value = value; 
            value += change;
            zero_count += change.abs() / 100;
            if (change % 100 != 0) && (prev_value % 100 != 0) {
                let remainder_sum = (prev_value % 100) + (change % 100);
                if remainder_sum.signum() != prev_value.signum() {
                    zero_count += 1;
                }
                if remainder_sum.abs() > 99 {
                    zero_count += 1;
                }
            }
            buf.clear();
        }
    }
    println!("{}", zero_count);
}

// Assumes the given line is checked to be at least 3 characters long.
fn parse_line(line: &str) -> i32 {
    let final_digit = line.len() - 1;
    let mut change = line[1..final_digit].parse::<i32>().expect("Failed parsing integer");
    if line.chars().nth(0).unwrap() == 'L' {
        change *= -1;
    }
    change
}
