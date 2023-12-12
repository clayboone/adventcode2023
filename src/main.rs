// use challenges::day01::day1_part1;
// use challenges::day01::day1_part2;
use std::{io::BufRead, time::Instant};

use std::sync::mpsc;
use std::thread;

mod challenges {
    pub mod day01 {
        pub mod solution;
        pub use super::super::challenges::day01::solution::day1_part1;
        pub use super::super::challenges::day01::solution::day1_part2;
    }
}

fn main() {
    let mut start_time: Instant;

    // for (day_and_part, input, func, correct_answer) in [
    for (day_and_part, input, correct_answer) in [
        // (
        //     "Day 1, Part 1",
        //     String::from("src/challenges/day01/input.txt"),
        //     day1_part1 as fn(&String) -> i32,
        //     54953,
        // ),
        (
            "Day 1, Part 2, challenge input",
            String::from("src/challenges/day01/input.txt"),
            // day1_part2 as fn(&String) -> i32,
            53868,
        ),
        (
            "Day 1, Part 2, 10mb input",
            String::from("src/challenges/day01/big_input_10mb.txt"),
            // day1_part2 as fn(&String) -> i32,
            112640,
        ),
        (
            "Day 1, Part 2, 100mb input",
            String::from("src/challenges/day01/big_input_100mb.txt"),
            // day1_part2 as fn(&String) -> i32,
            1126400,
        ),
        (
            "Day 1, Part 2, 1000mb input",
            String::from("src/challenges/day01/big_input_1000mb.txt"),
            // day1_part2 as fn(&String) -> i32,
            11264000,
        ),
        (
            "Day 1, Part 2, 10000mb input",
            String::from("src/challenges/day01/big_input_10000mb.txt"),
            // day1_part2 as fn(&String) -> i32,
            112640000,
        ),
    ] {
        let (tx, rx) = mpsc::channel();

        // Read line-by-line in a thread.
        let reader_thread = thread::spawn(move || {
            if let Ok(file) = std::fs::File::open(&input) {
                let reader = std::io::BufReader::new(file);
                for line in reader.lines() {
                    tx.send(line.unwrap()).unwrap();
                }
            }
            drop(tx);
        });

        // Process lines in another thread.
        let accumulator_thread = thread::spawn(move || {
            let mut result = 0;
            for line in rx {
                result += get_first_and_last_digits(&line).parse::<i32>().unwrap();
            }
            result
        });

        start_time = Instant::now();
        reader_thread.join().unwrap();
        let result = accumulator_thread.join().unwrap();
        let time_elapsed = start_time.elapsed();

        let correctness = if result == correct_answer {
            "CORRECT"
        } else {
            "INCORRECT"
        };

        let time_taken = match time_elapsed.as_millis() {
            0 => format!("{}μs", time_elapsed.as_micros()),
            _ => format!("{}ms", time_elapsed.as_millis()),
        };

        println!(
            "{}: {} ({}) in {}",
            day_and_part, result, correctness, time_taken
        );
    }
}

fn get_first_and_last_digits(line: &str) -> String {
    let mut first_and_last_digits_in_line = String::new();

    let mut beginning_of_line = String::new();
    let mut end_of_line = String::new();

    'outer: for chr in line.chars() {
        if chr.is_numeric() {
            first_and_last_digits_in_line.push(chr as char);
            break;
        }

        beginning_of_line.push(chr);

        // No need to proceed if we haven't read at least 3 characters.
        if beginning_of_line.len() < 3 {
            continue;
        }

        for (word, number) in WORD_NUMBER_MAPPING {
            if beginning_of_line.ends_with(word) {
                first_and_last_digits_in_line.push_str(number);
                break 'outer;
            }
        }
    }

    if first_and_last_digits_in_line.len() < 1 {
        first_and_last_digits_in_line.push('0');
    }

    // Now go backwards...
    'outer: for chr in line.chars().rev() {
        if chr.is_numeric() {
            first_and_last_digits_in_line.push(chr as char);
            break;
        }

        end_of_line.push(chr);

        // No need to proceed if we haven't read at least 3 characters.
        if end_of_line.len() < 3 {
            continue;
        }

        for (word, number) in REVERSED_WORD_NUMBER_MAPPING {
            if end_of_line.ends_with(word) {
                first_and_last_digits_in_line.push_str(number);
                break 'outer;
            }
        }
    }

    if first_and_last_digits_in_line.len() < 2 {
        first_and_last_digits_in_line.push('0');
    }

    first_and_last_digits_in_line
}

const WORD_NUMBER_MAPPING: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];
const REVERSED_WORD_NUMBER_MAPPING: [(&str, &str); 9] = [
    ("eno", "1"),
    ("owt", "2"),
    ("eerht", "3"),
    ("ruof", "4"),
    ("evif", "5"),
    ("xis", "6"),
    ("neves", "7"),
    ("thgie", "8"),
    ("enin", "9"),
];
