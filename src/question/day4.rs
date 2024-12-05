// --- Day 4: Ceres Search ---

use core::str;

use crate::day_solver::{test_day, DaySolver};

fn part1(input: &str) -> String {
    let mut count = 0;
    let line_length = input.lines().next().unwrap().len() + 1;
    let bytes = input.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'X' {
            count += positive_directions(bytes, i, line_length);
            count += negative_directions(bytes, i, line_length);
        }
    }
    count.to_string()
}

#[inline]
fn positive_check(bytes: &[u8], i: usize, step_by: usize) -> u32 {
    (bytes[i + step_by] == b'M' && bytes[i + 2 * step_by] == b'A' && bytes[i + 3 * step_by] == b'S')
        as u32
}

fn positive_directions(bytes: &[u8], i: usize, line_length: usize) -> u32 {
    if i + 3 * (line_length + 1) < bytes.len() {
        return positive_check(bytes, i, line_length + 1)
            + positive_check(bytes, i, line_length)
            + positive_check(bytes, i, line_length - 1)
            + positive_check(bytes, i, 1);
    } else if i + 3 * line_length < bytes.len() {
        return positive_check(bytes, i, line_length)
            + positive_check(bytes, i, line_length - 1)
            + positive_check(bytes, i, 1);
    } else if i + 3 * (line_length - 1) < bytes.len() {
        return positive_check(bytes, i, line_length - 1) + positive_check(bytes, i, 1);
    } else if i + 3 < bytes.len() {
        return positive_check(bytes, i, 1);
    }
    0
}

#[inline]
fn negative_check(bytes: &[u8], i: usize, step_by: usize) -> u32 {
    (bytes[i - step_by] == b'M' && bytes[i - 2 * step_by] == b'A' && bytes[i - 3 * step_by] == b'S')
        as u32
}

fn negative_directions(bytes: &[u8], i: usize, line_length: usize) -> u32 {
    if i >= 3 * (line_length + 1) {
        return negative_check(bytes, i, line_length + 1)
            + negative_check(bytes, i, line_length)
            + negative_check(bytes, i, line_length - 1)
            + negative_check(bytes, i, 1);
    } else if i >= 3 * line_length {
        return positive_check(bytes, i, line_length)
            + negative_check(bytes, i, line_length - 1)
            + negative_check(bytes, i, 1);
    } else if i >= 3 * (line_length - 1) {
        return negative_check(bytes, i, line_length - 1) + negative_check(bytes, i, 1);
    } else if i >= 3 {
        return negative_check(bytes, i, 1);
    }
    0
}

fn part2(input: &str) -> String {
    let mut count = 0;
    let line_lenght = input.lines().next().unwrap().len() + 1;
    let bytes = input.as_bytes();
    for i in line_lenght + 1..bytes.len() - line_lenght - 1 {
        if bytes[i] == b'A' {
            // the \n prevents x going past the boundaries of the line
            if (bytes[i + 1 - line_lenght] + bytes[i - 1 + line_lenght] == b'S' + b'M')
                && (bytes[i + 1 + line_lenght] + bytes[i - 1 - line_lenght] == b'S' + b'M')
            {
                count += 1;
            }
        }
    }
    count.to_string()
}

pub const SOLVER: DaySolver = DaySolver::new(
    part1,
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    "18",
    part2,
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    "9",
    4,
);

test_day!(SOLVER);
