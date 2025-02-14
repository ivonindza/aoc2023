use crate::{transpose, Platform};
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn parse_platform(input: &str) -> IResult<&str, Platform> {
    let (remainder, rows) = separated_list1(line_ending, many1(one_of(".#O")))(input)?;

    let platform = Platform {
        columns: transpose(&rows),
    };

    Ok((remainder, platform))
}

pub fn parse_input(input: &str) -> Result<Platform, Box<dyn std::error::Error + '_>> {
    let (_, platform) = parse_platform(input)?;

    Ok(platform)
}
