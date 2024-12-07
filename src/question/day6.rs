// --- Day 6: Guard Gallivant ---

use crate::day_solver::{test_day, DaySolver};

fn part1(input: &str) -> String {
    let mut input_clone = input.to_owned();
    let bytes: &mut [u8] = unsafe { input_clone.as_bytes_mut() };
    // find the line size
    let mut line_size = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'\n' {
            line_size = i as i32 + 1;
            break;
        }
    }
    // find the starting position
    let mut starting_position = 0;
    for i in 0..input.len() {
        if bytes[i] == b'^' {
            bytes[i] = b'X';
            starting_position = i;
            break;
        }
    }
    // walk
    let step_by_direction = [-line_size, 1, line_size, -1];
    let mut direction = 0;
    let mut result = 1;
    let mut x = starting_position;
    let mut step_by = -line_size;
    loop {
        let next_x = x as i32 + step_by;
        if next_x < 0 || next_x >= bytes.len() as i32 {
            break;
        } else {
            x = next_x as usize
        }
        match bytes[x] {
            b'.' => {
                bytes[x] = b'X';
                result += 1;
            },
            b'#' => {
                x = (x as i32 - step_by) as usize;
                direction += 1;
                direction %= 4;
                step_by = step_by_direction[direction];
            },
            b'X' => {},
            b'\n' => {break;},
            _ => panic!("Invalid character found: {}", bytes[x] as char),
        }
    }
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut input_clone = input.to_owned();
    let bytes: &mut [u8] = unsafe { input_clone.as_bytes_mut() };
    // find the line size
    let mut line_size = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'\n' {
            line_size = i as i32 + 1;
            break;
        }
    }
    // find the starting position
    let mut starting_position = 0;
    for i in 0..input.len() {
        if bytes[i] == b'^' {
            bytes[i] = 0;
            starting_position = i;
            break;
        }
    }
    // walk
    let step_by_direction = [-line_size, 1, line_size, -1];
    let mut direction = 0;
    let mut path_list = Vec::new();
    let mut x = starting_position;
    let mut step_by = -line_size;
    loop {
        let next_x = x as i32 + step_by;
        if next_x < 0 || next_x >= bytes.len() as i32 || next_x+1 % line_size == 0 {
            break;
        } else {
            x = next_x as usize
        }
        match bytes[x] {
            0b0001..=0b1111 => {
                bytes[x] |= 1 << direction;
            },
            b'.' => {
                bytes[x] |= 1 << direction;
                path_list.push((direction, x));
            },
            b'#' => {
                x = (x as i32 - step_by) as usize;
                direction += 1;
                direction %= 4;
                step_by = step_by_direction[direction];
            },
            _ => panic!("Invalid character found: {}", bytes[x] as char),
        }
        x = (x as i32 + step_by) as usize;
    }
    //
    "".to_string()
}

const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub const SOLVER: DaySolver = DaySolver::new(
    part1,
    SAMPLE_INPUT,
    "41",
    part2,
    SAMPLE_INPUT,
    "6",
    6
);

test_day!(SOLVER);