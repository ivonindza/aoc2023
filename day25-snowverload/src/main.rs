use day25_snowverload::{parser::parse_input, solve_part1};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let graph = parse_input(&input).expect("Invalid input");

    let result = solve_part1(&graph);
    println!("Part one result: {result}");
}
