use crate::{Command, InitSeq, Operation};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit0, one_of},
    combinator::recognize,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_command_v1(input: &str) -> IResult<&str, &str> {
    recognize(tuple((alpha1, one_of("-="), digit0)))(input)
}

fn parse_command_list_v1(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), parse_command_v1)(input)
}

fn parse_command_v2(input: &str) -> IResult<&str, Command> {
    let (remainder, (tag, op, focus)) = tuple((alpha1, one_of("-="), digit0))(input)?;

    let command = Command {
        tag: tag.chars().collect(),
        operation: Operation::from(op),
        focus_value: focus.parse::<u32>().unwrap_or(0),
    };

    Ok((remainder, command))
}

fn parse_command_list_v2(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(tag(","), parse_command_v2)(input)
}

pub fn parse_input(input: &str) -> Result<InitSeq, Box<dyn std::error::Error + '_>> {
    let (_, commands) = parse_command_list_v1(input.trim())?;
    let commands_v1 = commands
        .into_iter()
        .map(|cmd| cmd.chars().collect())
        .collect();

    let (_, commands_v2) = parse_command_list_v2(input.trim())?;

    Ok(InitSeq {
        commands_v1,
        commands_v2,
    })
}
