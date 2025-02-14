use day03_gear_ratios::Solver;
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let mut solver = Solver::parse_from_input(&input);

    let (result_part1, result_part2) = solver.solve();
    solver.print_colored_input(&input);
    println!("Part one result: {result_part1}");
    println!("Part two result: {result_part2}");
}
