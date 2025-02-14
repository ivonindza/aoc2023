use day13_mirrors::{parser::parse_input, solve_part1, solve_part2};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let patterns = parse_input(&input).expect("Invalid input");

    let result = solve_part1(&patterns);
    println!("Part one result: {result}");

    let result = solve_part2(&patterns);
    println!("Part two result: {result}");
}
