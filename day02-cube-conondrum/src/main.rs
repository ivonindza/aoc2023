use day02_cube_conondrum::{input_parser::parse_input, solve_part1, solve_part2, CubeSet};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let games = parse_input(&input).expect("Invalid input");

    let bag_config = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let result = solve_part1(&games, &bag_config);
    println!("Part one result: {result}");

    let result = solve_part2(&games);
    println!("Part two result: {result}");
}
