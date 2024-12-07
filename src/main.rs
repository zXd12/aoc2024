#![feature(test)]

mod day_solver;
mod params;
mod question;

fn main() {
    let day_map = day_solver::day_hashmap();
    let params = params::Params::new().parse_args(day_map.len() as u32);

    match params.question_list {
        Some(questions) => {
            for (day, question) in questions {
                if let Some(day_solver) = day_map.get(&day) {
                    let input = day_solver.get_input_file(params.session.clone(), day);
                    print!("Day {}", day);
                    match question {
                        0 => {
                            println!();
                            println!("Part 1: {}", (day_solver.part1)(&input));
                            println!("Part 2: {}", (day_solver.part2)(&input));
                        }
                        1 => {
                            println!(" Part 1: {}", (day_solver.part1)(&input));
                        }
                        2 => {
                            println!(" Part 2: {}", (day_solver.part2)(&input));
                        }
                        _ => println!("Invalid question number: {}", question),
                    }
                } else {
                    println!("Day {} is not available yet", day);
                }
            }
        }
        None => {
            for day in 1..=day_map.len() as u32 {
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