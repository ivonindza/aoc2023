use crate::{CardType, Hand};
use itertools::Itertools;
use nom::{
    character::complete::{anychar, space1, u32},
    multi::count,
    sequence::separated_pair,
    IResult,
};

fn parse_cards(input: &str) -> IResult<&str, [CardType; 5]> {
    let (remainder, cards_vec) = count(anychar, 5)(input)?;
    let cards: [CardType; 5] = cards_vec
        .into_iter()
        .map(|card_symbol| CardType::try_from(card_symbol).expect("Unrecognized card symbol"))
        .collect::<Vec<CardType>>()
        .try_into()
        .expect("Cards length different from 5");

    Ok((remainder, cards))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (remainder, (cards, bid)) = separated_pair(parse_cards, space1, u32)(input)?;

    let hand = Hand::new(cards, bid);

    Ok((remainder, hand))
}

pub fn parse_input(input: &str) -> Result<Vec<Hand>, Box<dyn std::error::Error + '_>> {
    let hands = input
        .lines()
        .map(|line| parse_hand(line).map(|(_, hand)| hand))
        .try_collect()?;

    Ok(hands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn hands() {
        let hands = parse_input(indoc! {"
            32T3K 765
            T55J5 684
        "})
        .unwrap();

        assert_eq!(hands.len(), 2);
    }
}
