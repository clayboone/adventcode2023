// use challenges::day01::day1_part1;
use challenges::day01::day1_part2;
use std::{time::Instant, io::BufRead};

mod challenges {
    pub mod day01 {
        pub mod solution;
        pub use super::super::challenges::day01::solution::day1_part1;
        pub use super::super::challenges::day01::solution::day1_part2;
    }
}

fn main() {
    let mut start_time: Instant;

    for (day_and_part, input, func, correct_answer) in [
        // (
        //     "Day 1, Part 1",
        //     String::from("src/challenges/day01/input.txt"),
        //     day1_part1 as fn(&String) -> i32,
        //     54953,
        // ),
        // (
        //     "Day 1, Part 2, challenge input",
        //     String::from("src/challenges/day01/input.txt"),
        //     day1_part2 as fn(&String) -> i32,
        //     53868,
        // ),
        (
            "Day 1, Part 2, 10mb input",
            String::from("src/challenges/day01/big_input_10mb.txt"),
            day1_part2 as fn(&String) -> i32,
            0,
        ),
        (
            "Day 1, Part 2, 100mb input",
            String::from("src/challenges/day01/big_input_100mb.txt"),
            day1_part2 as fn(&String) -> i32,
            0,
        ),
        (
            "Day 1, Part 2, 1000mb input",
            String::from("src/challenges/day01/big_input_1000mb.txt"),
            day1_part2 as fn(&String) -> i32,
            0,
        ),
        (
            "Day 1, Part 2, 10000mb input",
            String::from("src/challenges/day01/big_input_10000mb.txt"),
            day1_part2 as fn(&String) -> i32,
            0,
        ),
    ] {
        // let input = std::fs::read_to_string(input).unwrap();
        // read line-by-line
        let file = std::fs::File::open(input).unwrap();
        let reader = std::io::BufReader::new(file);
        let mut input = String::new();
        for line in reader.lines() {
            input.push_str(&line.unwrap());
            // reduce here to avoid allocating a new string for every line
            // TODO
        }

        start_time = Instant::now();
        let result = func(&input);
        println!("{}: {}ms", day_and_part, start_time.elapsed().as_millis());

        println!(
            "{}: {} ({})",
            day_and_part,
            result,
            if result == correct_answer {
                "CORRECT"
            } else {
                "INCORRECT"
            }
        );
    }
}
