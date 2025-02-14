use crate::{Direction, EdgeDesc};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, i64, one_of, space1},
    multi::count,
    sequence::{delimited, pair, tuple},
    IResult,
};

fn parse_color_code(input: &str) -> IResult<&str, EdgeDesc> {
    let (remainder, (length, direction)) = pair(count(anychar, 5), one_of("0123"))(input)?;

    let direction = match direction {
        '0' => Direction::E,
        '1' => Direction::S,
        '2' => Direction::W,
        '3' => Direction::N,
        _ => panic!("Unrecognized direction"),
    };
    let length =
        u32::from_str_radix(String::from_iter(length).as_str(), 16).expect("Invalid hex number");

    let edge = EdgeDesc {
        direction,
        length: length as i64,
    };

    Ok((remainder, edge))
}

fn parse_edge(input: &str) -> IResult<&str, (EdgeDesc, EdgeDesc)> {
    let (remainder, (direction, _, length, _, edge_desc_2)) = tuple((
        one_of("UDLR"),
        space1,
        i64,
        space1,
        delimited(tag("(#"), parse_color_code, tag(")")),
    ))(input)?;

    let direction = match direction {
        'U' => Direction::N,
        'D' => Direction::S,
        'L' => Direction::W,
        'R' => Direction::E,
        _ => panic!("Unrecognized direction"),
    };

    let edge_desc_1 = EdgeDesc { direction, length };

    Ok((remainder, (edge_desc_1, edge_desc_2)))
}

pub fn parse_input(
    input: &str,
) -> Result<Vec<(EdgeDesc, EdgeDesc)>, Box<dyn std::error::Error + '_>> {
    let edges = input
        .lines()
        .map(|line| parse_edge(line).map(|(_, edge_pair)| edge_pair))
        .try_collect()?;

    Ok(edges)
}
