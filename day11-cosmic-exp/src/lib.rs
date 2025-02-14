//! https://adventofcode.com/2023/day/11

use std::collections::HashSet;

#[derive(Clone)]
struct Coord {
    x: u64,
    y: u64,
}

impl From<(u64, u64)> for Coord {
    fn from((x, y): (u64, u64)) -> Coord {
        Coord { x, y }
    }
}

#[derive(Clone)]
pub struct CosmicImage {
    galaxies: Vec<Coord>,
    rows: u64,
    cols: u64,
}

impl CosmicImage {
    pub fn load_from_input(input: &str) -> CosmicImage {
        let mut galaxies: Vec<Coord> = Vec::new();
        let mut rows: usize = 0;
        let mut cols: usize = 0;

        for (j, line) in input.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '#' {
                    galaxies.push(Coord::from((i as u64, j as u64)));
                }
                rows = i;
            }
            cols = j;
        }

        CosmicImage {
            galaxies,
            rows: (rows + 1) as u64,
            cols: (cols + 1) as u64,
        }
    }

    /// Expand the space by moving the coordinates of the galaxies by the amount of empty rows and
    /// columns before them. Factor is a multiplicative factor of how many rows/columns each empty
    /// row/column is worth.
    fn expand(&mut self, factor: u64) {
        use itertools::sorted;

        let mut empty_rows: HashSet<u64> = HashSet::from_iter(0..self.rows);
        let mut empty_cols: HashSet<u64> = HashSet::from_iter(0..self.cols);

        for Coord { x, y } in &self.galaxies {
            empty_rows.remove(y);
            empty_cols.remove(x);
        }

        let empty_rows: Vec<u64> = sorted(empty_rows.into_iter()).collect();
        let empty_cols: Vec<u64> = sorted(empty_cols.into_iter()).collect();

        // For a galaxy at (x, y), shift x by the number of empty cols that are less than x, and
        // shift y by the number of empty rows that are less than y.
        //
        // binary_search returns the Err variant with the index where x could be inserted in the
        // array. This is equal to the number of empty columns before the galaxy. It should never
        // returns the Ok variant, since the column with a galaxy cannot be in the empty columns.
        for galaxy in &mut self.galaxies {
            let Coord { x, y } = &galaxy;
            let x_shift = factor * empty_cols.binary_search(x).unwrap_err() as u64;
            let y_shift = factor * empty_rows.binary_search(y).unwrap_err() as u64;

            *galaxy = Coord::from((x + x_shift, y + y_shift));
        }
    }
}

/// Compute Manhattan distances between each pair of galaxies. Return the sum of distances.
/// Expansion is of factor 2, i.e. it adds one additional row/column for each empty one.
pub fn solve_part1(space: &mut CosmicImage) -> u64 {
    use itertools::Itertools;

    space.expand(1);

    let mut sum: u64 = 0;
    for pair in space.galaxies.iter().combinations(2) {
        let (a, b) = (pair[0], pair[1]);
        sum += b.x.abs_diff(a.x) + b.y.abs_diff(a.y);
    }
    sum
}

/// Compute Manhattan distances between each pair of galaxies. Return the sum of distances.
/// Expansion is of factor 1_000_000, i.e. it adds 999_999 additional rows/columns for each empty
/// one.
pub fn solve_part2(space: &mut CosmicImage) -> u64 {
    use itertools::Itertools;

    space.expand(999_999);

    let mut sum: u64 = 0;
    for pair in space.galaxies.iter().combinations(2) {
        let (a, b) = (pair[0], pair[1]);
        sum += b.x.abs_diff(a.x) + b.y.abs_diff(a.y);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};

        let mut space = CosmicImage::load_from_input(&input);
        let result = solve_part1(&mut space);
        assert_eq!(result, 374);
        let result = solve_part2(&mut space);
        assert_eq!(result, 148000226);
    }
}
