use std::env;

use crate::CURRENT_DAY;

pub struct Params {
    pub session: Option<String>,
    pub question_list: Option<Vec<(u32, u32)>>,
}

impl Params {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        Self {
            session: match env::var("AOC_SESSION") {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            question_list: None,
        }
    }

    pub fn parse_args(mut self) -> Self {
        let args = env::args().collect::<Vec<String>>();
        let mut arg_index = 1;
        let day_regex = regex::Regex::new(r"^(\d+)(:(1|2))?$").unwrap();

        loop {
            match args.get(arg_index) {
                Some(arg) => {
                    if arg.starts_with("-") {
                        match arg.as_str() {
                            "-s" | "--session" => {
                                arg_index += 1;
                                self.session = Some(args.get(arg_index).unwrap().to_string());
                            }
                            "-c" | "--current" => match self.question_list {
                                Some(ref mut list) => list.push((CURRENT_DAY, 0)),
                                None => self.question_list = Some(vec![(CURRENT_DAY, 0)]),
                            }
                            _ => panic!("Invalid option: {}", arg),
                        }
                    } else if day_regex.is_match(arg) {
                        let captures = day_regex.captures(arg).unwrap();
                        let day = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let question = captures
                            .get(3)
                            .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
                        if day > CURRENT_DAY {
                            panic!("Day {} is not available yet", day);
                        }
                        match self.question_list {
                            Some(ref mut list) => list.push((day, question)),
                            None => self.question_list = Some(vec![(day, question)]),
                        }
                    } else {
                        panic!("Invalid argument: {}", arg);
                    }
                    arg_index += 1;
                }
                None => return self,
            }
        }
    }
}
