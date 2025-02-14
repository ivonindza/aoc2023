//! https://adventofcode.com/2023/day/8

use std::collections::HashMap;

pub mod parser;

const PART1_START_NODE: &str = "AAA";

#[derive(Debug)]
pub struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn is_start_node(&self) -> bool {
        self.id.ends_with("A")
    }

    fn is_finish_node(&self) -> bool {
        self.id.ends_with("Z")
    }
}

#[derive(Debug)]
pub struct Map {
    instructions: String,
    nodes: HashMap<String, Node>,
}

/// Follow the instructions to reach the final node from the start node and return the number of
/// steps it took.
pub fn count_steps(start_node: &str, map: &Map) -> u64 {
    let mut steps = 0;
    let mut current_node: &Node = &map.nodes[start_node];

    for step in map.instructions.chars().cycle() {
        match step {
            'L' => current_node = &map.nodes[&current_node.left],
            'R' => current_node = &map.nodes[&current_node.right],
            _ => panic!("Unexpected direction"),
        }
        steps += 1;
        if current_node.is_finish_node() {
            break;
        }
    }

    steps
}

/// Follow the instructions to reach the final node from the start node and return the number of
/// steps it took.
pub fn solve_part1(map: &Map) -> u64 {
    count_steps(PART1_START_NODE, map)
}

/// Follow the instructions to reach all final nodes from all start nodes and return the number of
/// steps it took.
pub fn solve_part2(map: &Map) -> u64 {
    let start_nodes: Vec<&str> = map
        .nodes
        .values()
        .filter(|node| node.is_start_node())
        .map(|node| node.id.as_str())
        .collect();

    start_nodes
        .into_iter()
        .map(|node_id| count_steps(node_id, map))
        .reduce(|left, right| num::integer::lcm(left, right))
        .expect("Error: No start nodes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};

        let items = parser::parse_input(&input).unwrap();
        let result = solve_part1(&items);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        let items = parser::parse_input(&input).unwrap();
        let result = solve_part1(&items);
        assert_eq!(result, 6);
    }

    #[test]
    fn test3() {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};

        let items = parser::parse_input(&input).unwrap();
        let result = solve_part2(&items);
        assert_eq!(result, 6);
    }
}
