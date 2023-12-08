use challenges::day01::day1_part1;
use challenges::day01::day1_part2;

mod challenges {
    pub mod day01 {
        pub mod solution;
        pub use super::super::challenges::day01::solution::day1_part1;
        pub use super::super::challenges::day01::solution::day1_part2;
    }
}

fn main() {
    for (day_and_part, input, func, correct_answer) in [
        (
            "Day 1, Part 1",
            String::from("src/challenges/day01/input.txt"),
            day1_part1 as fn(&String) -> i32,
            54953,
        ),
        (
            "Day 1, Part 2",
            String::from("src/challenges/day01/input.txt"),
            day1_part2 as fn(&String) -> i32,
            53868,
        ),
    ] {
        let input = std::fs::read_to_string(input).unwrap();
        let result = func(&input);
        println!(
            "{}: {} ({})",
            day_and_part,
            result,
            match result == correct_answer {
                true => "CORRECT",
                false => "INCORRECT",
            }
        );
    }
}
