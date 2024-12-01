// --- Day 1: Historian Hysteria ---

use std::collections::HashMap;

use crate::day_solver::{test_day, DaySolver};

fn part1(input: &str) -> String {
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();
    for line in input.lines() {
        let (a, b) = parse_line(line);
        first_column.push(a);
        second_column.push(b);
    }
    first_column.sort_unstable();
    second_column.sort_unstable();
    let mut sum = 0;
    for i in 0..first_column.len() {
        sum += first_column[i].abs_diff(second_column[i]);
    }
    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut similarity_map = HashMap::with_capacity(approximate_line_count(input));
    for line in input.lines() {
        let (a, b) = parse_line(line);
        match similarity_map.get_mut(&a) {
            Some((a_count, _)) => {
                *a_count += 1;
            }
            None => {
                similarity_map.insert(a, (1, 0));
            }
        }
        match similarity_map.get_mut(&b) {
            Some((_, b_count)) => {
                *b_count += 1;
            }
            None => {
                similarity_map.insert(b, (0, 1));
            }
        }
    }
    let mut sum = 0;
    for (i, (a, b)) in similarity_map {
        sum += a * b * i;
    }
    sum.to_string()
}

fn parse_line(val: &str) -> (u32, u32) {
    let bytes = val.as_bytes();
    let mut acc_a = 0;
    let mut acc_b = 0;
    let mut i = 0;
    while i < val.len() {
        let c = bytes[i] as char;
        if c == ' ' {
            i += 3; // numbers are separated by 3 spaces
            acc_b = acc_a;
            acc_a = 0;
            continue;
        }
        acc_a *= 10;
        acc_a += c as u32 - '0' as u32;
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
);

test_day!(SOLVER);
