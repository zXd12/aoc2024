// --- Day 1: Historian Hysteria ---

use std::collections::HashMap;

use crate::day_solver::{test_day, DaySolver};

fn part1(input: &str) -> String {
    let (mut first_column, mut second_column): (Vec<u32>, Vec<u32>) = input.lines().into_iter().map(parse_line).unzip();
    first_column.sort_unstable();
    second_column.sort_unstable();
    first_column.into_iter().zip(second_column).fold(0, |acc, (a, b)| acc + a.abs_diff(b)).to_string()
}

fn part2(input: &str) -> String {
    let mut similarity_map = HashMap::<u32, (u32, u32)>::with_capacity(approximate_line_count(input));
    for line in input.lines() {
        let (a, b) = parse_line(line);
        similarity_map.entry(a).and_modify(|e| e.0 += 1).or_insert((1, 0));
        similarity_map.entry(b).and_modify(|e| e.1 += 1).or_insert((0, 1));
    }
    similarity_map.iter().map(|(i, (a, b))| a * b * i).sum::<u32>().to_string()
}

fn parse_line(val: &str) -> (u32, u32) {
    let bytes = val.as_bytes();
    let mut acc_a = 0;
    let mut acc_b = 0;
    let mut i = 0;
    while i < val.len() {
        if bytes[i] as char == ' ' {
            i += 3; // numbers are separated by 3 spaces
            acc_a = acc_b;
            acc_b = 0;
            continue;
        }
        acc_b *= 10;
        acc_b += bytes[i] as u32 - '0' as u32;
        i += 1;
    }
    (acc_a, acc_b)
}

fn approximate_line_count(input: &str) -> usize {
    ((input.len() / input.lines().next().unwrap().len()) as f64 * 1.2) as usize
}

pub const SOLVER: DaySolver = DaySolver::new(
    part1,
    "3   4
4   3
2   5
1   3
3   9
3   3",
    "11",
    part2,
    "3   4
4   3
2   5
1   3
3   9
3   3",
    "31",
    1,
);

test_day!(SOLVER);
