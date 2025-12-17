use std::env;
use std::fs::File;
use std::io;

fn main() {
    let input = env::args().nth(1).expect("Missing file input argument");
    let reader = File::open(&input).expect("Failed to open input file");
    let ranges = io::read_to_string(reader)
        .expect("Failed to read from input file")
        .replace("\n", "") // Remove trailing newline
        .split(",")
        .map(|range| {
            range
                .split("-")
                .map(|range_part| {
                    range_part
                        .parse::<u64>()
                        .expect(&format!("{} not a number", range_part))
                })
                .collect::<Vec<u64>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(u64, u64)>>();

    let mut result = 0;

    for (start, end) in ranges.iter().copied() {
        for id in start..=end {
            let s = id.to_string();
            let len = s.len();

            for group_size in (1..=(len / 2)).rev().filter(|size| len % size == 0) {
                let chunks = s
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(group_size)
                    .map(|v| v.to_vec())
                    .collect::<Vec<_>>();
                if chunks.iter().skip(1).all(|chunk| chunk == &chunks[0]) {
                    result += id;
                    break;
                }
            }
        }
    }

    println!("{}", result);
}
