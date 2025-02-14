use day01_trebuchet::{calibrate, DIGIT1, DIGIT2_FORWARD, DIGIT2_REVERSE};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let result = calibrate(&input, &DIGIT1, &DIGIT1);
    println!("Part one result: {result}");

    let result = calibrate(&input, &DIGIT2_FORWARD, &DIGIT2_REVERSE);
    println!("Part two result: {result}");
}
