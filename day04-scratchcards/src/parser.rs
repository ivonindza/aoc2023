use crate::Card;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashSet;

/// A combinator that consumes both leading and trailing whitespace.
fn trim<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (remainder, (_, _card_id, _, winning_numbers, _, scratched_numbers)) = tuple((
        tag("Card"),
        trim(u32),
        tag(":"),
        trim(parse_number_list),
        tag("|"),
        trim(parse_number_list),
    ))(input)?;

    let card = Card {
        winning_numbers: HashSet::from_iter(winning_numbers),
        scratched_numbers: HashSet::from_iter(scratched_numbers),
    };

    Ok((remainder, card))
}

pub fn parse_input(input: &str) -> Result<Vec<Card>, Box<dyn std::error::Error + '_>> {
    let cards = input
        .lines()
        .map(|line| parse_card(line).map(|(_, card)| card))
        .try_collect()?;

    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_list() {
        let (_, numbers) = parse_number_list("1 10  100").unwrap();
        assert!(matches!(numbers[..], [1, 10, 100]));
    }

    #[test]
    fn card() {
        let input = "Card  10:  1 10 100 |  2 20 200";
        let (_, card) = parse_card(input).unwrap();
        assert!(matches!(card, Card { .. }));
    }
}
