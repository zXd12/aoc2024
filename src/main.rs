mod params;
mod question;
mod day_solver;

const CURRENT_DAY: u32 = 3;

const REPEATS: u32 = 10_000;

fn main() {
    let params = params::Params::new().parse_args();
    let day_map = day_solver::day_hashmap();

    match params.question_list {
        Some(questions) => {
            for (day, question) in questions {
                if let Some(day_solver) = day_map.get(&day) {
                    let input = day_solver.get_input_file(params.session.clone(), day);
                    match question {
                        0 => {
                            println!("Day {}", day);
                            let (part1_result, part1_duration) = timed_run(day_solver.part1, &input, REPEATS);
                            println!("Part 1: {} ({} ns)", part1_result, part1_duration.as_micros());
                            let (part2_result, part2_duration) = timed_run(day_solver.part2, &input, REPEATS);
                            println!("Part 2: {} ({} ns)", part2_result, part2_duration.as_micros());
                        }
                        1 => {
                            let (part1_result, part1_duration) = timed_run(day_solver.part1, &input, REPEATS);
                            println!(
                                "Day {} Part 1: {} ({} ns)",
                                day,
                                part1_result,
                                part1_duration.as_micros()
                            );
                        }
                        2 => {
                            let (part2_result, part2_duration) = timed_run(day_solver.part2, &input, REPEATS);
                            println!(
                                "Day {} Part 2: {} ({} ns)",
                                day,
                                part2_result,
                                part2_duration.as_micros()
                            );
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
                    let (part1_result, part1_duration) = timed_run(day_solver.part1, &input, REPEATS);
                    println!("Part 1: {} ({} ns)", part1_result, part1_duration.as_micros());
                    let (part2_result, part2_duration) = timed_run(day_solver.part2, &input, REPEATS);
                    println!("Part 2: {} ({} ns)", part2_result, part2_duration.as_micros());
                } else {
                    println!("Day {} is not available yet", day);
                }
            }
        }
    }
}

fn timed_run(fun: fn(&str) -> String, input: &str, repeats: u32) -> (String, std::time::Duration) {
    let start = std::time::Instant::now();
    for _ in 0..repeats {
        fun(input);
    }
    let result = fun(input);
    let duration = start.elapsed()/repeats;
    (result, duration)
}

