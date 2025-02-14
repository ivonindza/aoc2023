use itertools::Itertools;
use nom::{
    character::complete::{i32, space1},
    multi::separated_list1,
    IResult,
};

fn parse_number_list(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, i32)(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error + '_>> {
    let sequences = input
        .lines()
        .map(|line| parse_number_list(line).map(|(_, seq)| seq))
        .try_collect()?;

    Ok(sequences)
}
