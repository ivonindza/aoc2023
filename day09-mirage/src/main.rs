use day09_mirage::{parser::parse_input, solve};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let sequences = parse_input(&input).expect("Invalid input");

    let (result_part1, result_part2) = solve(&sequences);
    println!("Part one result: {result_part1}");
    println!("Part two result: {result_part2}");
}
