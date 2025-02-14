use crate::{Part, PartCategory, Rule, State, Workflow};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, multispace1, one_of, u32},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

// Values x, m, a, s are always given in the same order in the input.
fn parse_xmas(input: &str) -> IResult<&str, u32> {
    preceded(pair(one_of("xmas"), char('=')), u32)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (remainder, (x, _, m, _, a, _, s)) = delimited(
        tag("{"),
        tuple((
            parse_xmas,
            tag(","),
            parse_xmas,
            tag(","),
            parse_xmas,
            tag(","),
            parse_xmas,
        )),
        tag("}"),
    )(input)?;

    Ok((remainder, Part { x, m, a, s }))
}

fn parse_unconditional_rule(input: &str) -> IResult<&str, Rule> {
    let (remainder, name) = alpha1(input)?;

    let rule = Rule::ForwardUnconditionally {
        name: name.to_string(),
    };

    Ok((remainder, rule))
}

fn parse_conditional_rule(input: &str) -> IResult<&str, Rule> {
    let (remainder, (cat, cond, threshold, _, name)) =
        tuple((one_of("xmas"), one_of("<>"), u32, tag(":"), alpha1))(input)?;

    let category = match cat {
        'x' => PartCategory::X,
        'm' => PartCategory::M,
        'a' => PartCategory::A,
        's' => PartCategory::S,
        _ => panic!("Unrecognized part category"),
    };

    let rule = match cond {
        '<' => Rule::ForwardIfLess {
            category,
            threshold,
            name: name.to_string(),
        },
        '>' => Rule::ForwardIfGreater {
            category,
            threshold,
            name: name.to_string(),
        },
        _ => panic!("Unrecognized condition"),
    };

    Ok((remainder, rule))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_conditional_rule, parse_unconditional_rule))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (remainder, (name, rules)) = pair(
        alpha1,
        delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}")),
    )(input)?;

    let workflow = Workflow {
        name: name.to_string(),
        rules,
    };

    Ok((remainder, workflow))
}

pub fn parse_input(input: &str) -> Result<State, Box<dyn std::error::Error + '_>> {
    let (_, (workflows, parts)) = separated_pair(
        separated_list1(line_ending, parse_workflow),
        multispace1,
        separated_list1(line_ending, parse_part),
    )(input)?;

    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    Ok(State { workflows, parts })
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn part() {
        let (_, part) = parse_part("{x=2127,m=1623,a=2188,s=1013}").unwrap();
        assert_matches!(
            part,
            Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013,
            }
        )
    }

    #[test]
    fn workflow() {
        let (_, workflow) = parse_workflow("rfg{s<537:gd,x>2440:R,A}").unwrap();
        assert_matches!(workflow, Workflow { .. });
        assert_eq!(workflow.name, "rfg");
        assert_matches!(
            workflow.rules.as_slice(),
            [
                Rule::ForwardIfLess { .. },
                Rule::ForwardIfGreater { .. },
                Rule::ForwardUnconditionally { .. }
            ]
        );
    }
}
