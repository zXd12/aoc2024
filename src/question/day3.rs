// --- Day 3: Mull It Over ---

use crate::day_solver::{test_day, DaySolver};

#[derive(Debug)]
enum State {
    NotSet,
    M,
    U,
    L,
    MulParenthesis,
    Int1(u32),
    Comma,
    Int2(u32),
    D,
    O,
    N,
    Apostrophe,
    T,
    DoParenthesis,
    DontParenthesis,
}

fn part1(input: &str) -> String {
    let mut res = 0;
    let mut state = State::NotSet;
    let mut int1 = 0;
    let mut int2 = 0;
    for char in input.chars() {
        match state {
            State::NotSet => {
                state = match char {
                    'm' => State::M,
                    _ => State::NotSet,
                }
            }
            State::M => {
                state = match char {
                    'u' => State::U,
                    _ => State::NotSet,
                }
            }
            State::U => {
                state = match char {
                    'l' => State::L,
                    _ => State::NotSet,
                }
            }
            State::L => {
                state = match char {
                    '(' => State::MulParenthesis,
                    _ => State::NotSet,
                }
            }
            State::MulParenthesis => {
                state = match char {
                    '0'..='9' => {
                        int1 = char as i32 - '0' as i32;
                        State::Int1(0)
                    }
                    _ => State::NotSet,
                }
            }
            State::Int1(i) => {
                if i > 2 {
                    state = State::NotSet;
                } else {
                    state = match char {
                        ',' => State::Comma,
                        '0'..='9' => {
                            int1 *= 10;
                            int1 += char as i32 - '0' as i32;
                            State::Int1(i + 1)
                        }
                        _ => State::NotSet,
                    }
                }
            }
            State::Comma => {
                state = match char {
                    '0'..='9' => {
                        int2 = char as i32 - '0' as i32;
                        State::Int2(0)
                    }
                    _ => State::NotSet,
                }
            }
            State::Int2(i) => {
                if i > 2 {
                    state = State::NotSet;
                } else {
                    state = match char {
                        ')' => {
                            res += int1 * int2;
                            int1 = 0;
                            int2 = 0;
                            State::NotSet
                        }
                        '0'..='9' => {
                            int2 *= 10;
                            int2 += char as i32 - '0' as i32;
                            State::Int2(i + 1)
                        }
                        _ => State::NotSet,
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    res.to_string()
}

fn part2(input: &str) -> String {
    let mut res = 0;
    let mut dont = false;
    let mut state = State::NotSet;
    let mut int1 = 0;
    let mut int2 = 0;
    for char in input.chars() {
        state = if dont {
            match state {
                State::NotSet => match char {
                    'd' => State::D,
                    _ => State::NotSet,
                },
                State::D => match char {
                    'o' => State::O,
                    _ => State::NotSet,
                },
                State::O => match char {
                    '(' => State::DoParenthesis,
                    _ => State::NotSet,
                },
                State::DoParenthesis => match char {
                    ')' => {
                        dont = false;
                        State::NotSet
                    }
                    _ => State::NotSet,
                },
                _ => unreachable!(),
            }
        } else {
            match state {
                State::NotSet => match char {
                    'm' => {
                        if dont {
                            State::NotSet
                        } else {
                            State::M
                        }
                    }
                    'd' => State::D,
                    _ => State::NotSet,
                },
                State::M => match char {
                    'u' => State::U,
                    _ => State::NotSet,
                },
                State::U => match char {
                    'l' => State::L,
                    _ => State::NotSet,
                },
                State::L => match char {
                    '(' => State::MulParenthesis,
                    _ => State::NotSet,
                },
                State::MulParenthesis => match char {
                    '0'..='9' => {
                        int1 = char as i32 - '0' as i32;
                        State::Int1(0)
                    }
                    _ => State::NotSet,
                },
                State::Int1(i) => {
                    if i > 2 {
                        State::NotSet
                    } else {
                        match char {
                            ',' => State::Comma,
                            '0'..='9' => {
                                int1 *= 10;
                                int1 += char as i32 - '0' as i32;
                                State::Int1(i + 1)
                            }
                            _ => State::NotSet,
                        }
                    }
                }
                State::Comma => match char {
                    '0'..='9' => {
                        int2 = char as i32 - '0' as i32;
                        State::Int2(0)
                    }
                    _ => State::NotSet,
                },
                State::Int2(i) => {
                    if i > 2 {
                        State::NotSet
                    } else {
                        match char {
                            ')' => {
                                res += int1 * int2;
                                int1 = 0;
                                int2 = 0;
                                State::NotSet
                            }
                            '0'..='9' => {
                                int2 *= 10;
                                int2 += char as i32 - '0' as i32;
                                State::Int2(i + 1)
                            }
                            _ => State::NotSet,
                        }
                    }
                }
                State::D => match char {
                    'o' => State::O,
                    _ => State::NotSet,
                },
                State::O => match char {
                    'n' => State::N,
                    _ => State::NotSet,
                },
                State::N => match char {
                    '\'' => State::Apostrophe,
                    _ => State::NotSet,
                },
                State::Apostrophe => match char {
                    't' => State::T,
                    _ => State::NotSet,
                },
                State::T => match char {
                    '(' => State::DontParenthesis,
                    _ => State::NotSet,
                },
                State::DontParenthesis => match char {
                    ')' => {
                        dont = true;
                        State::NotSet
                    }
                    _ => State::NotSet,
                },
                _ => unreachable!(),
            }
        };
    }
    res.to_string()
}

pub const SOLVER: DaySolver = DaySolver::new(
    part1,
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    "161",
    part2,
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    "48",
    3,
);

test_day!(SOLVER);
