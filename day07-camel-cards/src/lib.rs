//! https://adventofcode.com/2023/day/7

use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

pub mod parser;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardType {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for CardType {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '2' => Ok(CardType::Two),
            '3' => Ok(CardType::Three),
            '4' => Ok(CardType::Four),
            '5' => Ok(CardType::Five),
            '6' => Ok(CardType::Six),
            '7' => Ok(CardType::Seven),
            '8' => Ok(CardType::Eight),
            '9' => Ok(CardType::Nine),
            'T' => Ok(CardType::Ten),
            'J' => Ok(CardType::Jack),
            'Q' => Ok(CardType::Queen),
            'K' => Ok(CardType::King),
            'A' => Ok(CardType::Ace),
            _ => Err("Invalid card symbol"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq)]
pub struct Hand {
    cards: [CardType; 5],
    bid: u32,
    hand_type: HandType,
}

fn determine_hand_type(cards: &[CardType; 5]) -> HandType {
    use itertools::sorted;

    let mut card_counts: HashMap<CardType, u32> = HashMap::new();
    for card in cards {
        *card_counts.entry(card.clone()).or_insert(0) += 1;
    }

    let n_jokers = card_counts.remove(&CardType::Joker).unwrap_or(0);

    let mut card_counts: Vec<u32> = sorted(card_counts.into_values()).rev().collect();

    // If card_counts vec is empty, it means that the hand is 5 Jokers
    if card_counts.is_empty() {
        card_counts.push(5);
    } else {
        card_counts[0] += n_jokers;
    }

    match card_counts[..] {
        [5] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => panic!("Impossible hand"),
    }
}

impl Hand {
    pub fn new(cards: [CardType; 5], bid: u32) -> Hand {
        let hand_type = determine_hand_type(&cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Sort hands to obtain their rank, then multiply each hands bid by its rank and add it all up.
fn sort_and_sum_hands(hands: &Vec<Hand>) -> u32 {
    let mut sorted_hands: Vec<Hand> = hands.to_vec();
    sorted_hands.sort();

    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = (i + 1) as u32;
            rank * hand.bid
        })
        .sum()
}

fn replace_jacks_with_jokers(hands: &Vec<Hand>) -> Vec<Hand> {
    let mut new_hands: Vec<Hand> = Vec::new();
    for hand in hands {
        let cards: [CardType; 5] = hand
            .cards
            .iter()
            .map(|card| match card {
                CardType::Jack => CardType::Joker,
                other_card => other_card.clone(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Hand lenght is not 5");
        new_hands.push(Hand::new(cards, hand.bid));
    }

    new_hands
}

/// Sort hands to obtain their rank, then multiply each hands bid by its rank and add it all up.
pub fn solve_part1(hands: &Vec<Hand>) -> u32 {
    sort_and_sum_hands(hands)
}

/// Sort hands to obtain their rank, then multiply each hands bid by its rank and add it all up.
/// Letter 'J' designates a Joker instead of a Jack.
pub fn solve_part2(hands: &Vec<Hand>) -> u32 {
    let hands = replace_jacks_with_jokers(hands);
    sort_and_sum_hands(&hands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let hands = parser::parse_input(&input).unwrap();
        let result = solve_part1(&hands);
        assert_eq!(result, 6440);
        let result = solve_part2(&hands);
        assert_eq!(result, 5905);
    }
}
