//! https://adventofcode.com/2023/day/9

pub mod parser;

/// Extrapolate the next and previous values for each sequence by creating the delta sequences
/// until you get an all-0 delta sequence.
pub fn solve(sequences: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut sum_backwards_extrapolate: i32 = 0;
    let mut sum_forwards_extrapolate: i32 = 0;

    for seq in sequences {
        let first = *seq.first().expect("Sequence is empty");
        let last = *seq.last().expect("Sequence is empty");

        let mut seq_starts: Vec<i32> = vec![first];
        let mut seq_ends: Vec<i32> = vec![last];
        let mut current_seq: Vec<i32> = seq.to_vec();

        loop {
            let delta_seq: Vec<i32> = current_seq
                .iter()
                .zip(current_seq.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect();

            if delta_seq.iter().all(|x| *x == 0) {
                break;
            }

            let first = *delta_seq.first().expect("Sequence is empty");
            seq_starts.push(first);
            let last = *delta_seq.last().expect("Sequence is empty");
            seq_ends.push(last);

            current_seq = delta_seq;
        }

        // Forward extrapolate is equal to the sum of ends of each sequence
        let forward_extrapolate = seq_ends.into_iter().sum::<i32>();
        sum_forwards_extrapolate += forward_extrapolate;

        // Backwards extrapolate is equal to the sum of starts of each sequence, except that the
        // sign is changing for every other element.
        let backwards_extrapolate = seq_starts
            .into_iter()
            .enumerate()
            .map(|(i, x)| if i % 2 == 0 { x } else { -x })
            .sum::<i32>();
        sum_backwards_extrapolate += backwards_extrapolate;
    }

    (sum_forwards_extrapolate, sum_backwards_extrapolate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        let sequences = parser::parse_input(&input).unwrap();
        let (result_part1, result_part2) = solve(&sequences);
        assert_eq!(result_part1, 114);
        assert_eq!(result_part2, 2);
    }
}
