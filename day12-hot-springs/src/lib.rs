//! https://adventofcode.com/2023/day/12

use std::fmt;
use std::collections::HashMap;

pub mod parser;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Status {
    fn from(ch: char) -> Status {
        match ch {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => panic!("Unexpected Status character"),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Status::Operational => '.',
            Status::Damaged => '#',
            Status::Unknown => '?',
        };
        write!(f, "{}", ch)
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    /// The sequence of statuses for each spring in one row
    seq: Vec<Status>,

    /// The list of sizes of damaged springs' clusters
    cluster_sizes: Vec<u32>,
}

/// Try to match one cluster in the sequence. If successful, return true. Update `seq` and
/// `cluster_sizes` to remove the matched cluster.
fn match_cluster(
    seq: &mut Vec<Status>,
    cluster_sizes: &mut Vec<u32>,
) -> bool {
    let cluster_size = cluster_sizes.pop().unwrap();

    // Remove `cluster_size` elements from the sequence. They must be either Damaged or Unknown.
    for _ in 0..cluster_size {
        match seq.pop() {
            Some(Status::Damaged) | Some(Status::Unknown) => { /* ok, do nothing */ },
            None | Some(Status::Operational) => return false,
        }
    }
    // Remove one more element from the sequence to separate the next cluster. It must be either
    // Operational or Unknown. If the sequence is empty, that is also ok, since that matches a
    // cluster at the beginning of the sequence.
    match seq.pop() {
        None | Some(Status::Operational) | Some(Status::Unknown) => { /* ok, do nothing */ },
        Some(Status::Damaged) => return false,
    }

    true
}

/// Count the number of possible matches between the sequence of statuses and the list of cluster
/// sizes of damaged clusters. We have a match for each arrangement of Operational/Damaged to
/// Unknown spring statuses that corresponds to the list of clusters sizes.
///
/// This is a recursive function, which recurses in order to account for both possible states of
/// Unknown statuses.
///
/// Matching is performed from the reverse, because it's more efficient to remove from the back of
/// a Vec than from the front.
///
/// Memoization with the `memo` cache is used to speed up the processing time.
fn count_matches(
    mut seq: Vec<Status>,
    mut damaged_clusters: Vec<u32>,
    memo: &mut HashMap<(Vec<Status>, Vec<u32>), u64>,
) -> u64 {
    // Skip Operational springs
    while let Some(Status::Operational) = seq.last() {
        seq.pop();
    }

    let key = (seq.clone(), damaged_clusters.clone());
    if let Some(result) = memo.get(&key) {
        return *result;
    }

    // If both lists are empty, then we have a match and return 1
    if seq.is_empty() && damaged_clusters.is_empty() {
        return 1;
    }
    // If seq is empty, but not the list of cluster sizes, then we don't have a match
    if seq.is_empty() {
        return 0;
    }
    // If the list of cluster sizes is empty, then we have a match only if there are no remaining
    // Damaged statuses in seq (Unknowns will be replaced as Operational).
    if damaged_clusters.is_empty() {
        match seq.iter().any(|s| *s == Status::Damaged) {
            true => return 0,
            false => return 1,
        }
    }

    let mut count: u64 = 0;

    match seq.last() {
        Some(Status::Damaged) => {
            if !match_cluster(&mut seq, &mut damaged_clusters) {
                return 0;
            }
            count += count_matches(seq, damaged_clusters, memo);
        }
        Some(Status::Unknown) => {
            // Option 1: Operational. If the status is Operational, we just remove it and recurse.
            seq.pop();
            count += count_matches(seq.clone(), damaged_clusters.clone(), memo);
            // Option 2: Damaged. If the status is Damaged, we push the Damaged status instead of
            // Unknown and recurse.
            seq.push(Status::Damaged);
            count += count_matches(seq, damaged_clusters, memo);
        }
        _ => unreachable!()
    }

    memo.insert(key, count);
    count
}

fn process_records(rows: Vec<Record>) -> u64 {
    let mut sum: u64 = 0;
    let mut memo = HashMap::new();

    for row in rows {
        sum += count_matches(row.seq, row.cluster_sizes, &mut memo);
    }

    sum
}

/// For each record, compute the number of possible operational/damaged spring arrangements
/// to unknown spots. Return the sum of arrangements.
pub fn solve_part1(records: &Vec<Record>) -> u64 {
    process_records(records.clone())
}

/// First unfold each record, by repeating the seq 5 times with '?' separators, and repeating the
/// cluster sizes 5 times as well. Then compute the same thing as in part 1, namely the sum of all
/// possible arrangements of operational/damaged springs to unknown spots.
pub fn solve_part2(records: &Vec<Record>) -> u64 {
    let records: Vec<Record> = records
        .iter()
        .map(|record| {
            let mut seq = record.seq.clone();
            for _ in 0..4 {
                seq.push(Status::Unknown);
                seq.extend(&record.seq);
            }

            let mut cluster_sizes = record.cluster_sizes.clone();
            for _ in 0..4 {
                cluster_sizes.extend(&record.cluster_sizes);
            }

            Record { seq, cluster_sizes }
        })
        .collect();

    process_records(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        let rows = parser::parse_input(&input).unwrap();
        let result = solve_part1(&rows);
        assert_eq!(result, 21);
        let result = solve_part2(&rows);
        assert_eq!(result, 525152);
    }
}
