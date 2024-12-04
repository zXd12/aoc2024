#![feature(test)]

mod day_solver;
mod params;
mod question;

const CURRENT_DAY: u32 = 3;

fn main() {
    let params = params::Params::new().parse_args();
    let day_map = day_solver::day_hashmap();

    match params.question_list {
        Some(questions) => {
            for (day, question) in questions {
                if let Some(day_solver) = day_map.get(&day) {
                    let input = day_solver.get_input_file(params.session.clone(), day);
                    print!("Day {}", day);
                    match question {
                        0 => {
                            println!();
                            println!("Part 1: {}", (day_solver.part2)(&input));
                            println!("Part 2: {}", (day_solver.part2)(&input));
                        }
                        1 => {
                            println!("Part 1: {}", (day_solver.part1)(&input));
                        }
                        2 => {
                            println!("Part 2: {}", (day_solver.part2)(&input));
                        }
                        _ => println!("Invalid question number: {}", question),
                    }
                } else {
                    println!("Day {} is not available yet", day);
                }
            }
        }
        None => {
            for day in 1..=CURRENT_DAY {
                if let Some(day_solver) = day_map.get(&day) {
                    let input = day_solver.get_input_file(params.session.clone(), day);
                    println!("Day {}", day);
                    println!("Part 1: {}", (day_solver.part1)(&input));
                    println!("Part 2: {}", (day_solver.part2)(&input));
                } else {
                    println!("Day {} is not available yet", day);
                }
            }
        }
    }
}