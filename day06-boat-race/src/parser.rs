use crate::Race;
use nom::{
    bytes::complete::take_till,
    character::complete::{digit1, space1, u64},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn parse_number_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

pub fn parse_input_part1(input: &str) -> Result<Vec<Race>, Box<dyn std::error::Error + '_>> {
    let (reminder, times) =
        preceded(take_till(|ch: char| ch.is_digit(10)), parse_number_list)(input)?;
    let (_, distances) =
        preceded(take_till(|ch: char| ch.is_digit(10)), parse_number_list)(reminder)?;

    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect();

    Ok(races)
}

fn parse_separated_number(input: &str) -> IResult<&str, u64> {
    let (remainder, parts) = separated_list1(space1, digit1)(input)?;
    let number_str = parts.join("");
    let number: u64 = number_str.parse().expect("Failed parsing an input u64 number");

    Ok((remainder, number))
}

pub fn parse_input_part2(input: &str) -> Result<Race, Box<dyn std::error::Error + '_>> {
    let (reminder, time) =
        preceded(take_till(|ch: char| ch.is_digit(10)), parse_separated_number)(input)?;
    let (_, distance) =
        preceded(take_till(|ch: char| ch.is_digit(10)), parse_separated_number)(reminder)?;

    let race = Race { time, distance };

    Ok(race)
}
