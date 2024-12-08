// --- Day X: Title ---

use crate::day_solver::{test_day, DaySolver};

const MAX_ANTENNA_OF_SAME_FREQUENCY: usize = 4;

fn part1(input: &str) -> String {
    let mut input_clone = input.to_owned();
    let bytes: &mut [u8] = unsafe { input_clone.as_bytes_mut() };
    // find the line size
    let mut line_size = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'\n' {
            line_size = i + 1;
            break;
        }
    }
    // [frequency] -> ([positions], index of last position)
    let mut antenna_table = [([0usize; MAX_ANTENNA_OF_SAME_FREQUENCY], 0usize); (b'z' - b'0') as usize + 1];
    bytes.iter().enumerate().for_each(|(i, &byte)| {
        if byte == b'.' || byte == b'\n' {
            return;
        }
        antenna_table[(byte - b'0') as usize].0[antenna_table[(byte - b'0') as usize].1] = i;
        antenna_table[(byte - b'0') as usize].1 += 1;
    });
    let mut antinode_count = 0;
    for (positions, index) in antenna_table {
        if index == 0 {
            continue;
        }
        for i in 0..index {
            let antenna1 = positions[i];
            let antenna1_line = antenna1 / line_size;
            for j in i + 1..index {
                let antenna2 = positions[j];
                let antenna2_line = antenna2 / line_size;
                let difference = antenna2 - antenna1;
                let line_difference = antenna2_line - antenna1_line;
                if difference <= antenna1
                    && line_difference <= antenna1_line
                    && bytes[antenna1 - difference] != b'#'
                    && bytes[antenna1 - difference] != b'\n'
                    && (antenna1 - difference) / line_size == antenna1_line - line_difference
                {
                    antinode_count += 1;
                    bytes[antenna1 - difference] = b'#';
                }
                if antenna2 + difference < bytes.len()
                    && bytes[antenna2 + difference] != b'#'
                    && bytes[antenna2 + difference] != b'\n'
                    && (antenna2 + difference) / line_size == antenna2_line + line_difference
                {
                    antinode_count += 1;
                    bytes[antenna2 + difference] = b'#';
                }
            }
        }
    }
    antinode_count.to_string()
}

fn part2(input: &str) -> String {
    let mut input_clone = input.to_owned();
    let bytes: &mut [u8] = unsafe { input_clone.as_bytes_mut() };
    // find the line size
    let mut line_size = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'\n' {
            line_size = i + 1;
            break;
        }
    }
    let mut antenna_table = [([0usize; MAX_ANTENNA_OF_SAME_FREQUENCY], 0usize); (b'z' - b'0') as usize + 1];
    bytes.iter().enumerate().for_each(|(i, &byte)| {
        if byte == b'.' || byte == b'\n' {
            return;
        }
        antenna_table[(byte - b'0') as usize].0[antenna_table[(byte - b'0') as usize].1] = i;
        antenna_table[(byte - b'0') as usize].1 += 1;
    });
    let mut antinode_count = 0;
    for (positions, index) in antenna_table {
        if index == 0 {
            continue;
        }
        for i in 0..index {
            let antenna1 = positions[i];
            let antenna1_line = antenna1 / line_size;
            for j in i + 1..index {
                let antenna2 = positions[j];
                let antenna2_line = antenna2 / line_size;
                let difference = antenna2 - antenna1;
                let line_difference = antenna2_line - antenna1_line;
                let mut i = 0;
                while (i * difference) <= antenna1
                    && (i * line_difference) <= antenna1_line
                    && (antenna1 - (i * difference)) / line_size
                        == antenna1_line - (i * line_difference)
                {
                    if bytes[antenna1 - i * difference] != b'#' && bytes[antenna1 - i * difference] != b'\n'
                    {
                        antinode_count += 1;
                        bytes[antenna1 - i * difference] = b'#';
                    }
                    i += 1;
                }
                i = 0;
                while antenna2 + (i * difference) < bytes.len()
                    && (antenna2 + (i * difference)) / line_size == antenna2_line + (i * line_difference)
                {
                    if bytes[antenna2 + i * difference] != b'#'
                    && bytes[antenna2 + i * difference] != b'\n' {
                        antinode_count += 1;
                        bytes[antenna2 + i * difference] = b'#';
                    }
                    i += 1;
                }
            }
        }
    }
    antinode_count.to_string()
}

const SAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

pub const SOLVER: DaySolver = DaySolver::new(part1, SAMPLE_INPUT, "14", part2, SAMPLE_INPUT, "34", 8);

test_day!(SOLVER);
