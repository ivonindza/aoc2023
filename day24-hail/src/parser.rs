use crate::{Coord3, Hailstone, InputConfiguration};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::i64,
    sequence::{separated_pair, tuple},
    IResult,
};

fn parse_coord(input: &str) -> IResult<&str, Coord3> {
    let (remainder, (x, _, y, _, z)) = tuple((i64, tag(", "), i64, tag(", "), i64))(input)?;

    Ok((remainder, Coord3::from((x, y, z))))
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let (remainder, (position, velocity)) =
        separated_pair(parse_coord, tag(" @ "), parse_coord)(input)?;

    let hailstone = Hailstone { position, velocity };

    Ok((remainder, hailstone))
}

pub fn parse_input(input: &str) -> Result<InputConfiguration, Box<dyn std::error::Error + '_>> {
    let hailstones = input
        .lines()
        .map(|line| parse_hailstone(line).map(|(_, hailstone)| hailstone))
        .try_collect()?;

    Ok(InputConfiguration { hailstones })
}
