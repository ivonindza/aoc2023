use day06_boat_race::{parser::{parse_input_part1, parse_input_part2}, solve_part1, solve_part2};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let races = parse_input_part1(&input).expect("Invalid input");
    let result = solve_part1(&races);
    println!("Part one result: {result}");

    let race = parse_input_part2(&input).expect("Invalid input");
    let result = solve_part2(&race);
    println!("Part two result: {result}");
}
