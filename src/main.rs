// use challenges::day01::day1_part1;
use challenges::day01::day1_part2;
use std::time::Instant;

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
        (
            "Day 1, Part 2, challenge input",
            String::from("src/challenges/day01/input.txt"),
            day1_part2 as fn(&String) -> i32,
            53868,
        ),
        (
            "Day 1, Part 2, big input",
            String::from("src/challenges/day01/input_big.txt"),
            day1_part2 as fn(&String) -> i32,
            19,
        ),
        (
            "Day 1, Part 2, bigger input",
            String::from("src/challenges/day01/input_bigger.txt"),
            day1_part2 as fn(&String) -> i32,
            19,
        ),
        (
            "Day 1, Part 2, biggest input",
            String::from("src/challenges/day01/input_biggest.txt"),
            day1_part2 as fn(&String) -> i32,
            19,
        ),
    ] {
        let input = std::fs::read_to_string(input).unwrap();

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
