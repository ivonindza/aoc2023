use crate::{Map, Node};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending, multispace1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

fn parse_node(input: &str) -> IResult<&str, Node> {
    #[rustfmt::skip]
    let (remainder, (node, _, left, _, right, _)) = tuple((
        alphanumeric1,
        tag(" = ("),
        alphanumeric1,
        tag(", "),
        alphanumeric1,
        tag(")")
    ))(input)?;

    let node = Node {
        id: node.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    };

    Ok((remainder, node))
}

fn parse_node_list(input: &str) -> IResult<&str, Vec<Node>> {
    separated_list1(line_ending, parse_node)(input)
}

pub fn parse_input(input: &str) -> Result<Map, Box<dyn std::error::Error + '_>> {
    let (_, (instructions, nodes)) = separated_pair(alpha1, multispace1, parse_node_list)(input)?;

    let nodes = HashMap::from_iter(nodes.into_iter().map(|node| (node.id.clone(), node)));
    let map = Map {
        instructions: instructions.to_string(),
        nodes,
    };

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn node() {
        let input = "AAA = (BBB, CCC)";
        let (_, node) = parse_node(input).unwrap();
        assert_matches!(node, Node { .. });
    }
}
