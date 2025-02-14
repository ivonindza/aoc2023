use crate::{Brick, Coord};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::u32,
    sequence::{separated_pair, tuple},
    IResult,
};

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    let (remainder, (x, _, y, _, z)) = tuple((u32, tag(","), u32, tag(","), u32))(input)?;

    Ok((remainder, Coord::from((x, y, z))))
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    let (remainder, (start, end)) = separated_pair(parse_coord, tag("~"), parse_coord)(input)?;

    let brick = Brick::try_new(start, end).unwrap();

    Ok((remainder, brick))
}

pub fn parse_input(input: &str) -> Result<Vec<Brick>, Box<dyn std::error::Error + '_>> {
    let bricks = input
        .lines()
        .map(|line| parse_brick(line).map(|(_, brick)| brick))
        .try_collect()?;

    Ok(bricks)
}
