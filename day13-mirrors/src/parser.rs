use crate::Pattern;
use nom::{
    character::complete::{line_ending, multispace1, one_of},
    multi::{many1, separated_list1},
    IResult,
};

fn transpose<T: Clone>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut output: Vec<Vec<T>> = Vec::new();
    let row_len = input[0].len();

    for i in 0..row_len {
        let mut col: Vec<T> = Vec::new();
        for row in input {
            col.push(row[i].clone())
        }
        output.push(col);
    }
    output
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let (remainder, rows) = separated_list1(line_ending, many1(one_of(".#")))(input)?;

    let cols: Vec<Vec<char>> = transpose(&rows);

    let pattern = Pattern { rows, cols };

    Ok((remainder, pattern))
}

pub fn parse_input(input: &str) -> Result<Vec<Pattern>, Box<dyn std::error::Error + '_>> {
    let (_, patterns) = separated_list1(multispace1, parse_pattern)(input)?;

    Ok(patterns)
}
