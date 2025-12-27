use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() {
    let file_path = std::env::args().nth(1).expect("Missing input parameter");
    let input_file = File::open(file_path).expect("Failed to open input file");
    let (ranges, ids) = parse_input(input_file);
    let mut fresh_count = 0;

    for id in ids.into_iter() {
        for range in ranges.iter() {
            if *range.start() > id {
                // Since it is sorted we know it can't be in the rest.
                break;
            }
            if range.contains(&id) {
                fresh_count += 1;
                break;
            }
        }
    }

    let fresh_ids = ranges
        .into_iter()
        .fold(
            Vec::<RangeInclusive<u64>>::with_capacity(64),
            |mut merged_ranges, curr| {
                if let Some(prev) = merged_ranges.last().cloned() {
                    if prev.end() >= curr.start() {
                        merged_ranges.pop();
                        merged_ranges
                            .push((*prev.start())..=(std::cmp::max(*prev.end(), *curr.end())));
                    } else {
                        // There is no overlap, so just return curr.
                        merged_ranges.push(curr);
                    }
                } else {
                    // First range.
                    merged_ranges.push(curr)
                };
                merged_ranges
            },
        )
        .into_iter()
        .fold(0, |fresh_ids, range| {
            fresh_ids + (range.end() - range.start() + 1)
        });
    println!("{}", fresh_count);
    println!("{}", fresh_ids);
}

fn parse_input(input: File) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut state = 0;
    let mut ranges = Vec::with_capacity(64);
    let mut ids = Vec::<u64>::with_capacity(64);
    for line in BufReader::new(input)
        .lines()
        .map(|s| s.expect("Failed to read input file").trim().to_string())
    {
        if state == 0 {
            if line.is_empty() {
                state = 1;
            } else {
                let (start, end) = line.split_once("-").expect("Invalid range found");
                let range = (start.parse::<u64>().expect("Invalid start of range"))
                    ..=(end.parse::<u64>().expect("Invalid end of range"));
                ranges.push(range);
            }
        } else {
            ids.push(line.parse().expect("invalid id"));
        }
    }

    ranges.sort_by_key(|range| *range.start());

    (ranges, ids)
}
