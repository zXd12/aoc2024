use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use crate::question::day1;

pub(crate) fn day_hashmap() -> HashMap<u32, DaySolver> {
    let mut day_map = HashMap::new();
    day_map.insert(1, day1::SOLVER);
    day_map
}

pub struct DaySolver {
    pub part1: fn(&str) -> String,
    #[allow(dead_code)]
    pub part1_sample_input: &'static str,
    #[allow(dead_code)]
    pub part1_sample_solution: &'static str,
    pub part2: fn(&str) -> String,
    #[allow(dead_code)]
    pub part2_sample_input: &'static str,
    #[allow(dead_code)]
    pub part2_sample_solution: &'static str,
}

impl DaySolver {
    pub const fn new(
        part1: fn(&str) -> String,
        part1_sample_input: &'static str,
        part1_sample_solution: &'static str,
        part2: fn(&str) -> String,
        part2_sample_input: &'static str,
        part2_sample_solution: &'static str,
    ) -> Self {
        Self {
            part1,
            part1_sample_input,
            part1_sample_solution,
            part2,
            part2_sample_input,
            part2_sample_solution,
        }
    }

    pub fn get_input_file(&self, session: Option<String>, day: u32) -> String {
        match File::open(format!("input/day{}.txt", day)) {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut input = String::new();
                buf_reader.read_to_string(&mut input).unwrap();
                input
            }
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::NotFound => {
                        let input = self.get_session_input(session.unwrap(), day).unwrap();
                        let mut file = File::create(format!("input/day{}.txt", day)).unwrap();
                        file.write_all(input.as_bytes()).unwrap();
                        input
                    }
                    _ => panic!("Error reading file: {}", err),
                }
            },
        }
    }

    pub fn get_session_input(&self, session: String, day: u32) -> Result<String, Box<dyn std::error::Error>> {
        let path = format!("https://adventofcode.com/2023/day/{}/input", day);
        let response = minreq::get(&path)
            .with_header("cookie", &format!("session={}", session))
            .send()?;
        let response = String::from_utf8_lossy(&response.into_bytes()).to_string();
        Ok(response.to_owned())
    }
}

macro_rules! test_day {
    ($day:ident) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part1_sample() {
                assert_eq!(($day.part1)($day.part1_sample_input), $day.part1_sample_solution);
            }

            #[test]
            fn part2_sample() {
                assert_eq!(($day.part2)($day.part2_sample_input), $day.part2_sample_solution);
            }
        }
    };
}

pub(crate) use test_day;