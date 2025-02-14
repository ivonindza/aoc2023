//! https://adventofcode.com/2023/day/1

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref DIGIT1: Regex = Regex::new(r"\d").unwrap();
    pub static ref DIGIT2_FORWARD: Regex =
        Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    pub static ref DIGIT2_REVERSE: Regex =
        Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
}

fn parse_digit(digit_as_str: &str) -> u32 {
    let digit: u32 = match digit_as_str {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ => digit_as_str
            .parse::<u32>()
            .expect("Expected a single digit string"),
    };
    digit
}

fn find_first_digit(input: &str, digit_re: &Regex) -> Option<u32> {
    digit_re.find(input).map(|m| parse_digit(m.as_str()))
}

// Notice that you cannot find the last digit while scanning forward, because regexes are greedy
// and for an input such as "nineight", they would match "nine" rather than the expected "eight".
fn find_last_digit(input: &str, digit_re: &Regex) -> Option<u32> {
    let rev_input: String = input.chars().rev().collect();
    digit_re.find(&rev_input).map(|m| parse_digit(m.as_str()))
}

/// For each line of the input combine the first digit and the last digit (in that order) to form a
/// single two-digit number. Then sum all the two-digit numbers and return the sum. If there are no
/// digits in the line, use 0.
pub fn calibrate(input: &str, digit_re_fwd: &Regex, digit_re_rev: &Regex) -> u32 {
    let sum = input
        .lines()
        .map(|line| {
            let first = find_first_digit(line, digit_re_fwd);
            let last = find_last_digit(line, digit_re_rev).or(first);
            let first = first.unwrap_or(0);
            let last = last.unwrap_or(0);
            let line_value = first * 10 + last;
            line_value
        })
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        let result = calibrate(&input, &DIGIT1, &DIGIT1);
        assert_eq!(result, 142);

        let result = calibrate(&input, &DIGIT2_FORWARD, &DIGIT2_REVERSE);
        assert_eq!(result, 142);
    }

    #[test]
    fn test2() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        let result = calibrate(&input, &DIGIT1, &DIGIT1);
        assert_eq!(result, 209);

        let result = calibrate(&input, &DIGIT2_FORWARD, &DIGIT2_REVERSE);
        assert_eq!(result, 281);
    }

    #[test]
    fn test3() {
        let input = indoc! {"
            justlettertwodigit
            nodigits
            twone
        "};

        let result = calibrate(&input, &DIGIT1, &DIGIT1);
        assert_eq!(result, 0);

        let result = calibrate(&input, &DIGIT2_FORWARD, &DIGIT2_REVERSE);
        assert_eq!(result, 43);
    }
}
