// --- Day 4: Ceres Search ---

use core::str;

use crate::day_solver::{test_day, DaySolver};

#[derive(Debug)]
enum State {
    X,
    M,
    A,
    S,
}

impl From<State> for u8 {
    fn from(state: State) -> u8 {
        match state {
            State::X => b'X',
            State::M => b'M',
            State::A => b'A',
            State::S => b'S',
        }
    }
}

impl State {
    fn next(&self) -> State {
        match self {
            State::X => State::M,
            State::M => State::A,
            State::A => State::S,
            State::S => unreachable!(),
        }
    }
}

fn part1(input: &str) -> String {
    let mut count = 0;
    let line_lenght = input.lines().next().unwrap().len() + 1;
    let bytes = input.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'X' {
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    if search(bytes, i, line_lenght as i32, (x, y), State::X) {
                        count += 1;
                    }
                }
            }
        }
    }
    count.to_string()
}

fn search(bytes: &[u8], i: usize, line_length: i32, direction: (i32, i32), state: State) -> bool {
    match state {
        State::S => {
            return true;
        }
        _ => {}
    }
    let (dx, dy) = direction;
    if ((i as i32 % line_length) + dx) / line_length != 0 {
        return false;
    }
    let next_i = i as i32 + dx + (dy * (line_length));
    if next_i < 0 || next_i as usize >= bytes.len() {
        return false;
    }
    if bytes[next_i as usize] == state.next().into() {
        return search(bytes, next_i as usize, line_length, direction, state.next());
    }
    return false;
}

fn part2(input: &str) -> String {
    let mut count = 0;
    let line_lenght = input.lines().next().unwrap().len() + 1;
    let bytes = input.as_bytes();
    for i in line_lenght + 1..bytes.len() - line_lenght - 1 {
        if bytes[i] == b'A' {
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
