use std::io::{BufRead, BufReader, Read, Write, Seek, SeekFrom};


fn main() {
    let input_path = std::env::args().nth(1).expect("Requires two path args");
    let work_file_path = std::env::args().nth(2).expect("Requires two path args");

    let input = std::fs::File::open(input_path).expect("Failed to read input file");
    let mut output = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(work_file_path)
        .expect("Failed to open work file path");

    let file_size = input.metadata().expect("Failed to get metadata of file").len();

    output.set_len(file_size).expect("Fialed to set len of output file");
    
    let buf_input = BufReader::new(input);
   
    let mut curr_value = [0u8];
    let mut roll_count = 0;
    for (line_num, line) in buf_input.lines().map(|l| l.expect("Failed to read line from input file")).enumerate().map(|(line_num, line)| (line_num as u64, line)) {


        for (idx, c) in line.chars().enumerate().map(|(idx, c)| (idx as u64, c)) {
            if c == '@' {
                roll_count += 1;
                
            // Add one to every neighbor.
                for (x, y) in get_neighbors(idx, line_num, line.len() as u64 - 1, file_size / (line.len() as u64)) {
                    let curr_pos = (y * line.len() as u64) + x;
                    
                    output.seek(SeekFrom::Start(curr_pos)).expect("Failed to seek output file");
                    output.read(&mut curr_value).expect("failed to read in char at pos");
                    curr_value[0] += 1;

                    if curr_value == 4 {
                        // This one has 4 neighbors, it can't be accessed
                        roll_count -= 1;
                    }
                    output.seek(SeekFrom::Start(curr_pos)).expect("Failed to seek output file");
                    output.write(&curr_value).expect("Failed to write ot output file");
                }
            }
        }

    }

}



fn get_neighbors(x: u64, y: u64, grid_size_x: u64, grid_size_y: u64) -> Vec<(u64, u64)> {
    let left = x.saturating_sub(1);
    let top = x.saturating_sub(1);
    let right = std::cmp::min(x + 1, grid_size_x);
    let bottom = std::cmp::min(y + 1, grid_size_y);

    (left..=right).zip((bottom..=top).into_iter()).filter(|(x2, y2)| (x2, y2) != (&x, &y)).collect()
}
