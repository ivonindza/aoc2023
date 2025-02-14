use day05_garden::{parser::parse_input, solve_part1, solve_part2};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let cfg = parse_input(&input).unwrap();

    let result = solve_part1(&cfg);
    println!("Part one result: {result}");

    let result = solve_part2(&cfg);
    println!("Part two result: {result}");
}
