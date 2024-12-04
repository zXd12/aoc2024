// --- Day 2: Red-Nosed Reports ---

use crate::day_solver::{test_day, DaySolver};

fn part1(input: &str) -> String {
    let mut count = 0;
    'line: for line in input.lines() {
        let bytes = line.as_bytes();
        let mut i = 0;
        let mut current_level = parse_int(bytes, &mut i);
        let mut next_level = parse_int(bytes, &mut i);
        if (current_level - next_level).abs() > 3 || next_level == current_level {
            continue 'line;
        }
        let is_decreasing = current_level > next_level;
        while i < line.len() {
            current_level = next_level;
            next_level = parse_int(bytes, &mut i);
            if invalid_levels(current_level, next_level, is_decreasing) {
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

fn invalid_levels(current_level: i32, next_level: i32, is_decreasing: bool) -> bool {
    (current_level - next_level).abs() > 3
        || current_level == next_level
        || current_level > next_level && !is_decreasing
        || current_level < next_level && is_decreasing
}

struct LevelState {
    last: i32,
    current: i32,
    next: i32,
    had_bad_level: bool,
    is_decreasing: bool,
}

fn skip_current_or_next(level_state: &mut LevelState, next_next_level: i32) -> bool {
    // skips current
    if invalid_levels(
        level_state.last,
        level_state.next,
        level_state.is_decreasing,
    ) {
        // skips next
        if invalid_levels(
            level_state.current,
            next_next_level,
            level_state.is_decreasing,
        ) {
            return true;
        } else {
            level_state.next = next_next_level;
        }
    } else {
        level_state.current = level_state.next;
        level_state.next = next_next_level;
    }
    false
}

fn part2(input: &str) -> String {
    let mut count = 0;
    'line: for line in input.lines() {
        let bytes = line.as_bytes();
        let mut i = 0;
        let mut state = LevelState {
            last: 0,
            current: parse_int(bytes, &mut i),
            next: parse_int(bytes, &mut i),
            had_bad_level: false,
            is_decreasing: false,
        };
        if state.next == state.current {
            state.next = parse_int(bytes, &mut i);
            if state.next == state.current {
                continue 'line;
            }
            state.had_bad_level = true;
        }
        if (state.current - state.next).abs() > 3 {
            if state.had_bad_level {
                continue 'line;
            }
            let third_level = parse_int(bytes, &mut i);
            let fourth_level = parse_int(bytes, &mut i);
            state.is_decreasing = third_level > fourth_level;
            if invalid_levels(third_level, fourth_level, state.is_decreasing) {
                continue 'line;
            }
            if invalid_levels(state.current, third_level, state.is_decreasing) {
                if invalid_levels(state.next, third_level, state.is_decreasing) {
                    continue 'line;
                } else {
                    state.last = state.next;
                }
            } else {
                state.last = state.current;
            }
            state.current = third_level;
            state.next = fourth_level;
            state.had_bad_level = true;
        }
        state.is_decreasing = state.current > state.next;
        state.last = state.current;
        state.current = state.next;
        // the third level is a special case for monotonicity check, it may require removing either one of the first 3 levels
        if !state.had_bad_level {
            state.next = parse_int(bytes, &mut i);
            if state.next == state.current {
                state.current = state.last;
                state.had_bad_level = true;
            }
            if (state.current - state.next).abs() > 3 {
                if state.had_bad_level {
                    continue 'line;
                }
                let next_next_level = parse_int(bytes, &mut i);
                // skips current
                if invalid_levels(
                    state.last,
                    state.next,
                    state.last > state.next,
                ) {
                    // skips next
                    if invalid_levels(
                        state.current,
                        next_next_level,
                        state.is_decreasing,
                    ) {
                        continue 'line;
                    } else {
                        state.next = next_next_level;
                    }
                } else {
                    state.is_decreasing = state.last > state.next;
                    state.current = state.next;
                    state.next = next_next_level;
                }
                state.had_bad_level = true;
            }
            if state.next < state.current && !state.is_decreasing
                || state.next > state.current && state.is_decreasing
            {
                if state.had_bad_level {
                    continue 'line;
                }
                let next_next_level = parse_int(bytes, &mut i); // next next is the 4th level
                                                                // checks for removing lvl 2
                if invalid_levels(state.current, next_next_level, state.is_decreasing) {
                    // checks for removing lvl 0
                    let would_be_decreasing = state.current > state.next;
                    if invalid_levels(state.next, next_next_level, would_be_decreasing) {
                        // checks for removing lvl 1
                        if state.last == state.next || (state.last - next_next_level).abs() > 3 {
                            continue 'line;
                        }
                        let would_be_decreasing = state.last > state.next;
                        if invalid_levels(state.next, next_next_level, would_be_decreasing) {
                            continue 'line;
                        } else {
                            state.current = state.next;
                            state.next = next_next_level;
                            state.is_decreasing = would_be_decreasing;
                        }
                    } else {
                        state.last = state.current;
                        state.current = state.next;
                        state.next = next_next_level;
                        state.is_decreasing = would_be_decreasing;
                    }
                } else {
                    state.next = next_next_level;
                }
                state.had_bad_level = true;
            }
            state.last = state.current;
            state.current = state.next;
        }
        // levels (3)4(5)+
        while i < line.len() {
            state.next = parse_int(bytes, &mut i);
            if state.had_bad_level {
                if invalid_levels(state.current, state.next, state.is_decreasing) {
                    continue 'line;
                }
            } else {
                if i == line.len() {
                    count += 1;
                    continue 'line;
                }
                if state.next == state.current {
                    state.had_bad_level = true;
                    continue;
                }
                if (state.current - state.next).abs() > 3 {
                    let next_next_level = parse_int(bytes, &mut i);
                    if skip_current_or_next(&mut state, next_next_level) {
                        continue 'line;
                    }
                    state.had_bad_level = true;
                }
                if state.next < state.current && !state.is_decreasing
                    || state.next > state.current && state.is_decreasing
                {
                    let next_next_level = parse_int(bytes, &mut i);
                    if skip_current_or_next(&mut state, next_next_level) {
                        continue 'line;
                    }
                    state.had_bad_level = true;
                }
            }
            state.last = state.current;
            state.current = state.next;
        }
        count += 1;
    }
    count.to_string()
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
    "4",
    2,
);

test_day!(SOLVER);
