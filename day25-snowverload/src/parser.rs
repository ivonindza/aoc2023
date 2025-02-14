use crate::{Component, Graph};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_component(input: &str) -> IResult<&str, Component> {
    let (remainder, ident) = alpha1(input)?;

    let component = Component {
        ident: ident
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Ident length must be 3"),
    };

    Ok((remainder, component))
}

fn parse_component_connections(input: &str) -> IResult<&str, (Component, Vec<Component>)> {
    let (remainder, (component, components)) = separated_pair(
        parse_component,
        tag(": "),
        separated_list1(space1, parse_component),
    )(input)?;

    Ok((remainder, (component, components)))
}

pub fn parse_input(input: &str) -> Result<Graph, Box<dyn std::error::Error + '_>> {
    let adj_list = input
        .lines()
        .map(|line| parse_component_connections(line).map(|(_, item)| item))
        .try_collect()?;

    let graph = Graph { adj_list };

    Ok(graph.to_undirected())
}
