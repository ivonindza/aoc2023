use day15_lens_lib::{parser::parse_input, solve_part1, solve_part2};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let seq = parse_input(&input).expect("Invalid input");

    let result = solve_part1(&seq);
    println!("Part one result: {result}");

    let result = solve_part2(&seq);
    println!("Part two result: {result}");
}
