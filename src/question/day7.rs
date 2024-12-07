// --- Day 7: Bridge Repair ---

use crate::day_solver::{test_day, DaySolver};

const MAX_TEST_VALUES: usize = 16;

fn part1(input: &str) -> String {
    let mut result = 0;
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let mut target = 0;
        loop {
            if bytes[i] == b':' {
                break;
            }
            target *= 10;
            target += (bytes[i] - b'0') as i64;
            i += 1;
        }
        i += 2;
        let mut usable_numbers = [0; MAX_TEST_VALUES];
        let mut number_index = 0;
        while bytes[i] != b'\n' {
            if bytes[i] == b' ' {
                number_index += 1;
                i += 1;
                continue;
            }
            usable_numbers[number_index] *= 10;
            usable_numbers[number_index] += (bytes[i] - b'0') as i64;
            i += 1;
        }
        if check_part1(target, usable_numbers, number_index + 1) {
            result += target;
        }
        i += 1;
    }
    result.to_string()
}

fn check_part1(target: i64, usable_numbers: [i64; MAX_TEST_VALUES], index: usize) -> bool {
    if target <= 0 {
        return target == 0;
    }
    if index == 0 {
        return false;
    }
    let number = usable_numbers[index - 1];
    (target % number == 0 && check_part1(target / number, usable_numbers, index - 1))
        || check_part1(target - number, usable_numbers, index - 1)
}

fn part2(input: &str) -> String {
    let mut result = 0;
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let mut target = 0;
        loop {
            if bytes[i] == b':' {
                break;
            }
            target *= 10;
            target += (bytes[i] - b'0') as i64;
            i += 1;
        }
        i += 2;
        let mut usable_numbers = [(1, 0); MAX_TEST_VALUES];
        let mut number_index = 0;
        while bytes[i] != b'\n' {
            if bytes[i] == b' ' {
                number_index += 1;
                i += 1;
                continue;
            }
            usable_numbers[number_index].1 *= 10;
            usable_numbers[number_index].1 += (bytes[i] - b'0') as i64;
            usable_numbers[number_index].0 *= 10;
            i += 1;
        }
        if check_part2(target, usable_numbers, number_index + 1) {
            result += target;
        }
        i += 1;
    }
    result.to_string()
}

fn check_part2(target: i64, usable_numbers: [(i64, i64); MAX_TEST_VALUES], index: usize) -> bool {
    if target <= 0 {
        return target == 0;
    }
    if index == 0 {
        return false;
    }
    let (number_magnitude, number) = usable_numbers[index - 1];
    (target % number_magnitude == number
        && check_part2(target / number_magnitude, usable_numbers, index - 1))
        || (target % number == 0 && check_part2(target / number, usable_numbers, index - 1))
        || check_part2(target - number, usable_numbers, index - 1)
}

const SAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

pub const SOLVER: DaySolver =
    DaySolver::new(part1, SAMPLE_INPUT, "3749", part2, SAMPLE_INPUT, "11387", 7);

test_day!(SOLVER);
