//! https://adventofcode.com/2023/day/14

use std::collections::HashMap;

pub mod parser;

#[derive(Clone, PartialEq)]
pub struct Platform {
    columns: Vec<Vec<char>>,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rows = transpose(&self.columns);
        for row in rows {
            writeln!(f, "{}", String::from_iter(row))?;
        }
        Ok(())
    }
}

fn transpose<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut output: Vec<Vec<T>> = Vec::new();
    let row_len = input[0].len();

    for i in 0..row_len {
        let mut col: Vec<T> = Vec::new();
        for row in input {
            col.push(row[i])
        }
        output.push(col);
    }
    output
}

fn swap(line: &mut Vec<char>, i: usize, j: usize) {
    if i != j {
        let tmp = line[i];
        line[i] = line[j];
        line[j] = tmp;
    }
}

/// Shift all 'O' rocks as far as they will go to the beginning of the line
fn shift_rocks_left(line: &mut Vec<char>) {
    let mut swap_index: usize = 0;
    for i in 0..line.len() {
        match line[i] {
            '#' => {
                swap_index = i + 1;
            },
            'O' => {
                swap(line, i, swap_index);
                swap_index = swap_index + 1;
            },
            '.' => { /* do nothing */ },
            _ => panic!("Unexpected symbol"),
        }
    }
}

/// Shift all 'O' rocks as far as they will go to the end of the line
fn shift_rocks_right(line: &mut Vec<char>) {
    let mut swap_index: usize = line.len() - 1;
    for i in (0..line.len()).rev() {
        match line[i] {
            '#' => {
                swap_index = i.saturating_sub(1);
            },
            'O' => {
                swap(line, i, swap_index);
                swap_index = swap_index.saturating_sub(1);
            },
            '.' => { /* do nothing */ },
            _ => panic!("Unexpected symbol"),
        }
    }
}

/// Compute the load of a column
fn compute_load(column: &Vec<char>) -> usize {
    column
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            if *ch == 'O' {
                column.len() - i
            } else {
                0
            }
        })
        .sum()
}

fn cycle(platform: &mut Platform) {
    // tip N
    platform.columns.iter_mut().for_each(|mut column| shift_rocks_left(&mut column));
    platform.columns = transpose(&platform.columns);

    // tip W
    platform.columns.iter_mut().for_each(|mut column| shift_rocks_left(&mut column));
    platform.columns = transpose(&platform.columns);

    // tip S
    platform.columns.iter_mut().for_each(|mut column| shift_rocks_right(&mut column));
    platform.columns = transpose(&platform.columns);

    // tip E
    platform.columns.iter_mut().for_each(|mut column| shift_rocks_right(&mut column));
    platform.columns = transpose(&platform.columns);
}

/// Compute the total load on the platform after tipping it north
pub fn solve_part1(platform: &Platform) -> u32 {
    let columns = platform.columns.clone();
    
    columns
        .into_iter()
        .map(|mut column| {
            shift_rocks_left(&mut column);
            compute_load(&column) as u32
        })
        .sum()
}

/// Compute the total load on the platform after 1_000_000_000 cycles
pub fn solve_part2(platform: &Platform) -> u32 {
    let mut platform: Platform = platform.clone();

    // Track in which loop iteration each platform snapshot is first seen
    let mut snapshots: HashMap<String, usize> = HashMap::new();
    let mut last_iteration: usize = 0;
    let mut loop_length: usize = 0;

    for i in 0..1_000_000_000 {
        cycle(&mut platform);

        let key = platform.to_string();
        match snapshots.get(&key) {
            None => {
                snapshots.insert(key, i);
            }
            Some(prev_iteration) => {
                last_iteration = i;
                loop_length = i - *prev_iteration;
                break;
            }
        }
    }

    if loop_length != 0 {
        let remainder = (1_000_000_000 - 1 - last_iteration) % loop_length;
        for _ in 0..remainder {
            cycle(&mut platform);
        }
    }

    platform
        .columns
        .iter()
        .map(|column| compute_load(&column) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn tip_north() {
        let column = ".O.O#O..#..O.";
        let mut column: Vec<char> = column.chars().collect();

        super::shift_rocks_left(&mut column);
        assert_eq!(String::from_iter(&column), "OO..#O..#O...");

        super::shift_rocks_right(&mut column);
        assert_eq!(String::from_iter(&column), "..OO#..O#...O");
    }

    #[test]
    fn test1() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};

        let platform = parser::parse_input(&input).unwrap();
        let result = solve_part1(&platform);
        assert_eq!(result, 136);
        let result = solve_part2(&platform);
        assert_eq!(result, 64);
    }
}
