//! https://adventofcode.com/2023/day/15

use indexmap::map::IndexMap;

pub mod parser;

pub struct InitSeq {
    pub commands_v1: Vec<Vec<char>>,
    pub commands_v2: Vec<Command>,
}

pub enum Operation {
    Remove,
    Replace,
}

impl From<char> for Operation {
    fn from(ch: char) -> Operation {
        match ch {
            '-' => Operation::Remove,
            '=' => Operation::Replace,
            _ => panic!("Unrecognized operation character"),
        }
    }
}

pub struct Command {
    pub tag: Vec<char>,
    pub operation: Operation,
    pub focus_value: u32,
}

fn hash(command: &[char]) -> u32 {
    let mut hash: u32 = 0;
    for ch in command {
        let ascii_code = *ch as u32;
        hash += ascii_code;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

/// Compute a hash for each command, then return the sum of hashes
pub fn solve_part1(seq: &InitSeq) -> u32 {
    seq.commands_v1.iter().map(|command| hash(command)).sum()
}

/// Run the HASHMAP initialization sequence, then return the sum of focusing power
pub fn solve_part2(seq: &InitSeq) -> u32 {
    let mut boxes: Vec<IndexMap<String, u32>> = vec![IndexMap::new(); 256];

    // The initialization sequence
    for command in &seq.commands_v2 {
        let box_idx = hash(&command.tag) as usize;
        let key = String::from_iter(command.tag.iter());

        match command.operation {
            Operation::Remove => {
                boxes[box_idx].shift_remove(&key);
            }
            Operation::Replace => {
                boxes[box_idx].insert(key, command.focus_value);
            }
        }
    }

    // Compute and sum the focusing power
    let mut sum: u32 = 0;
    for (box_idx, boxx) in boxes.iter().enumerate() {
        for (slot_idx, focus_value) in boxx.values().enumerate() {
            let focus_power = (box_idx + 1) as u32 * (slot_idx + 1) as u32 * focus_value;
            sum += focus_power;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "};

        let seq = parser::parse_input(&input).unwrap();
        let result = solve_part1(&seq);
        assert_eq!(result, 1320);
        let result = solve_part2(&seq);
        assert_eq!(result, 145);
    }
}
