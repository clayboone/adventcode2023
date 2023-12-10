use rstest::rstest;

pub fn day1_part1(input: &String) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        let all_numeric_bytes = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .into_bytes();

        if all_numeric_bytes.len() < 1 {
            continue;
        }

        let first_and_last = [
            all_numeric_bytes[0],
            all_numeric_bytes[all_numeric_bytes.len() - 1],
        ]
        .into_iter()
        .map(|b| b as char)
        .collect::<String>()
        .parse::<i32>()
        .unwrap();

        result += first_and_last;
    }

    result
}

pub fn day1_part2(input: &String) -> i32 {
    // We can't iterate over a list of strings to replace, then replace them,
    // because the replacements will be applied in the order of the list.
    // Instead we have to replace the words for the numbers as we come across
    // them. Eg. We can't do:
    // "twone".replace("one", "1").replace("two", "2")...
    // Because "one" will be replaced with "1", leaving "tw1" which will then
    // not be replaced at all. Instead we'll read from the front of the
    // string until we find either "one", "two", etc, a number, or the end of
    // the string. We'll repeat this process starting from the end of the
    // string as well to find the last digit, which will result in a string
    // like "11" given "one" and "21" given "twone".

    let mut parsed_lines = String::new();

    for line in input.lines() {
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

            for (word, number) in [
                ("one", "1"),
                ("two", "2"),
                ("three", "3"),
                ("four", "4"),
                ("five", "5"),
                ("six", "6"),
                ("seven", "7"),
                ("eight", "8"),
                ("nine", "9"),
            ] {
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

            for (word, number) in [
                ("one", "1"),
                ("two", "2"),
                ("three", "3"),
                ("four", "4"),
                ("five", "5"),
                ("six", "6"),
                ("seven", "7"),
                ("eight", "8"),
                ("nine", "9"),
            ] {
                if end_of_line.ends_with(word.chars().rev().collect::<String>().as_str()) {
                    first_and_last_digits_in_line.push_str(number);
                    break 'outer;
                }
            }
        }

        if first_and_last_digits_in_line.len() < 2 {
            first_and_last_digits_in_line.push('0');
        }

        parsed_lines.push_str(&format!("{}\n", first_and_last_digits_in_line));
    }

    day1_part1(&parsed_lines)
}

#[rstest(
    input,
    expected,
    case(String::from(""), 0),
    case(String::from("a"), 0),
    case(String::from("one"), 11),
    case(String::from("aoneb"), 11),
    case(String::from("one1abc2two"), 12),
    case(String::from("one2abc1two"), 12),
    case(String::from("aonebtwocthreed"), 13),
    case(String::from("oneatwobthreecfour"), 14),
    case(String::from("aoneb\noneabctwo"), 23),
    case(String::from("one\ntwo\nthree\nfour"), 110),
    case(String::from("aonea\nbtwob\ncthreec\ndfourd"), 110),
    case(String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n"), 142),
    case(String::from("9four94kpmvbtblbxthreefourone"), 91),
    case(String::from("nineonebmfdxxfqvvkrblrd9"), 99),
    case(String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n"), 281),
)]
fn test_day1_part2(input: String, expected: i32) {
    assert_eq!(day1_part2(&input), expected);
}

#[rstest(
    input,
    expected,
    case(String::from(""), 0),
    case(String::from("a"), 0),
    case(String::from("1"), 11),
    case(String::from("one"), 0),
    case(String::from("one2"), 22),
    case(String::from("a1b"), 11),
    case(String::from("1abc2"), 12),
    case(String::from("a1b2c3d"), 13),
    case(String::from("1a2b3c4"), 14),
    case(String::from("a1b\n1abc2"), 23),
    case(String::from("1\n2\n3\n4"), 110),
    case(String::from("a1a\nb2b\nc3c\nd4d"), 110),
    case(String::from("1a1\n2b2\n3c3\n4d4"), 110),
    case(String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n"), 142)
)]
fn test_day1_part1(input: String, expected: i32) {
    assert_eq!(day1_part1(&input), expected);
}
