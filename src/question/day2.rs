// --- Day 2: Red-Nosed Reports ---

use crate::day_solver::{test_day, DaySolver};

fn part1(input: &str) -> String {
    let mut count = 0;
    'line: for line in input.lines() {
        let bytes = line.as_bytes();
        let mut i = 0;
        let mut last_number = parse_int(bytes, &mut i);
        let mut current_number = parse_int(bytes, &mut i);
        if (last_number - current_number).abs() > 3 || current_number == last_number {
            continue 'line;
        }
        let is_decreasing = last_number > current_number;
        while i < line.len() {
            last_number = current_number;
            current_number = parse_int(bytes, &mut i);
            if (last_number - current_number).abs() > 3 || current_number == last_number {
                continue 'line;
            }
            if current_number < last_number && !is_decreasing
                || current_number > last_number && is_decreasing
            {
                continue 'line;
            }
        }
        count += 1;
    }
    count.to_string()
}

fn parse_int(bytes: &[u8], i: &mut usize) -> i32 {
    let mut integer = 0;
    while *i < bytes.len() {
        let char = bytes[*i] as char;
        *i += 1;
        if char == ' ' {
            break;
        }
        integer *= 10;
        integer += char as i32 - '0' as i32;
    }
    integer
}

fn part2(input: &str) -> String {
    "0".to_string()
}

pub const SOLVER: DaySolver = DaySolver::new(
    part1,
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    "2",
    part2,
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    "0",
);

test_day!(SOLVER);
