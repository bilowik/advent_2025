use std::io;
use std::env;
use std::fs::File;

fn main() {
    let input = env::args().nth(1).expect("Missing file input argument");
    let reader = File::open(&input).expect("Failed to open input file");
    let ranges = io::read_to_string(reader)
        .expect("Failed to read from input file")
        .split(",")
        .map(|range| range.split("-").map(|range_part| range_part.parse::<u32>().expect("Not a number")).collect::<Vec<u32>>())
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(u32, u32)>>();

    let mut result = 0;

    for (start, end) in ranges.iter().copied() {
        for id in start..=end {
            let id_str = id.to_string();
            let len = id_str.len();
            if len % 2 == 0 {
                // Even digits it may be invalid.
                if &id_str[..(len / 2)] == &id_str[(len / 2)..] {
                    result += id; 
                }
            }
        }
    }

    println!("{}", result);
    
    
}
