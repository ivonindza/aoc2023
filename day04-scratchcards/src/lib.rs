//! https://adventofcode.com/2023/day/4

use std::collections::HashSet;

pub mod parser;

#[derive(Debug)]
pub struct Card {
    winning_numbers: HashSet<u32>,
    scratched_numbers: HashSet<u32>,
}

/// Return the sum of card points. Card has 1 point for the first match, and the points are doubled
/// for each match after the first.
pub fn solve_part1(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card| {
            let matches = card
                .scratched_numbers
                .intersection(&card.winning_numbers)
                .count();
            let points = match matches {
                0 => 0,
                n => 1 << (n - 1),
            };
            points
        })
        .sum()
}

/// Return the total number of cards won. Having N matches on card i wins you cards i+1..i+N-1.
pub fn solve_part2(cards: &Vec<Card>) -> u32 {
    let mut cards_won: Vec<u32> = vec![1; cards.len()];

    for (card_id, card) in cards.iter().enumerate() {
        let copies = cards_won[card_id];
        let matches = card
            .scratched_numbers
            .intersection(&card.winning_numbers)
            .count();

        for i in 1..=matches {
            let next_id = card_id + i;
            if let Some(count) = cards_won.get_mut(next_id) {
                *count += copies;
            }
        }
    }

    cards_won.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        let cards = parser::parse_input(&input).unwrap();
        let result = solve_part1(&cards);
        assert_eq!(result, 13);
        let result = solve_part2(&cards);
        assert_eq!(result, 30);
    }
}
