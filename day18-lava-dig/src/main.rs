use day18_lava_dig::{parser::parse_input, solve_part1, solve_part2};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let edges = parse_input(&input).expect("Invalid input");

    let result = solve_part1(&edges);
    println!("Part one result: {result}");

    let result = solve_part2(&edges);
    println!("Part two result: {result}");
}
