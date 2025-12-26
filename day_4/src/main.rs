use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

fn main() {
    let input_path = std::env::args().nth(1).expect("Requires two path args");
    let work_file_path = std::env::args().nth(2).expect("Requires two path args");
    let mut input = tempfile::tempfile().expect("failed to open tepmfile");

    let ro_input = std::fs::File::open(input_path).expect("Failed to read input file");
    let mut neighbor_tracker = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(work_file_path)
        .expect("Failed to open work file path");

    // remove newlines from our input whilst getting line lenght.
    let mut line_len = 0;
    for line in BufReader::new(ro_input)
        .lines()
        .map(|line| line.expect("failed to read input file"))
    {
        if line_len == 0 {
            line_len = line.len() as u64;
        }
        input
            .write(line[..line.len()].as_bytes())
            .expect("failed to write to input file");
    }

    let file_size = input
        .metadata()
        .expect("Failed to get metadata of file")
        .len();

    neighbor_tracker
        .set_len(0)
        .expect("failed to truncate neighbor_tracker file"); // Truncate existing data.
    neighbor_tracker
        .set_len(file_size)
        .expect("Failed to set len of neighbor_tracker file");

    input
        .seek(SeekFrom::Start(0))
        .expect("Failed to seek input file");

    let mut curr_value;
    let mut line_num;
    let mut idx;
    let grid_size_x = line_len;
    let grid_size_y = file_size / line_len;
    for curr_pos in 0..file_size {
        line_num = curr_pos / line_len;
        idx = curr_pos % line_len;
        let c = char_at(&mut input, curr_pos) as char;
        if c == '@' {
            // Add one to every neighbor.
            for (x, y) in get_neighbors(idx, line_num, grid_size_x, grid_size_y) {
                let neighbor_pos = (y * line_len) + x;
                curr_value = char_at(&mut neighbor_tracker, neighbor_pos);
                curr_value += 1;
                set_char(&mut neighbor_tracker, neighbor_pos, curr_value);
            }
        }
    }

    // Now we iterate through removing all the ones that can be removed and repeating that
    // iteration until we've removed as many as possible
    let mut curr_removed;
    let mut total_removed = 0;
    while {
        curr_removed = 0u64;
        for curr_pos in 0..file_size {
            line_num = curr_pos / line_len;
            idx = curr_pos % line_len;
            let c = char_at(&mut input, curr_pos) as char;

            if c == '@' {
                curr_value = char_at(&mut neighbor_tracker, curr_pos);
                if curr_value < 4 {
                    total_removed += 1;
                    curr_removed += 1;
                    set_char(&mut input, curr_pos, '.' as u8);
                    // Remove one to every neighbor.
                    for (x, y) in get_neighbors(idx, line_num, grid_size_x, grid_size_y) {
                        let neighbor_pos = (y * line_len) + x;
                        curr_value = char_at(&mut neighbor_tracker, neighbor_pos);
                        curr_value -= 1;
                        set_char(&mut neighbor_tracker, neighbor_pos, curr_value);
                    }
                }
            }
        }
        curr_removed > 0
    } {}
    println!("{}", total_removed);
}

fn get_neighbors(x: u64, y: u64, grid_size_x: u64, grid_size_y: u64) -> Vec<(u64, u64)> {
    let left = x.saturating_sub(1);
    let top = y.saturating_sub(1);
    let right = std::cmp::min(x + 1, grid_size_x - 1);
    let bottom = std::cmp::min(y + 1, grid_size_y - 1);
    let mut neighbors = Vec::new();

    for nx in left..=right {
        for ny in top..=bottom {
            if (nx, ny) != (x, y) {
                neighbors.push((nx, ny));
            }
        }
    }
    neighbors
}

fn set_char(file: &mut std::fs::File, offset: u64, v: u8) {
    file.seek(SeekFrom::Start(offset))
        .expect("failed to seek in file");
    file.write(&[v]).expect("Failed to set char in file");
}

fn char_at(file: &mut std::fs::File, offset: u64) -> u8 {
    let mut buf = [0u8];
    file.seek(SeekFrom::Start(offset))
        .expect("Failed to seek in file");
    file.read(&mut buf).expect("Fialed to read in file");
    buf[0]
}
