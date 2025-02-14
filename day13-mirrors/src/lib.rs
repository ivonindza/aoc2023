//! https://adventofcode.com/2023/day/13

pub mod parser;

#[derive(Debug)]
pub struct Pattern {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

#[derive(Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

/// Reflection line is before the row/column with index post_line_index
struct ReflectionLine {
    pub post_line_index: usize,
    pub orientation: Orientation,
}

trait LineComparator {
    /// Compare two lines
    fn cmp_line(line1: &Vec<char>, line2: &Vec<char>) -> bool;

    /// Compare two sets of lines to find if they are a mirror match
    fn mirror_match(lines1: &[Vec<char>], lines2: &[Vec<char>]) -> bool;
}

struct StrictLineComparator {}

impl LineComparator for StrictLineComparator {
    fn cmp_line(line1: &Vec<char>, line2: &Vec<char>) -> bool {
        line1 == line2
    }

    fn mirror_match(lines1: &[Vec<char>], lines2: &[Vec<char>]) -> bool {
        for (line1, line2) in lines1.iter().zip(lines2.iter().rev()) {
            if line1 != line2 {
                return false;
            }
        }
        true
    }
}

/// Line comparator that will account for one difference (smudge).
struct SmudgeLineComparator {}

impl LineComparator for SmudgeLineComparator {
    fn cmp_line(line1: &Vec<char>, line2: &Vec<char>) -> bool {
        line1 == line2 || is_off_by_one(line1, line2)
    }

    /// Returns true of there is exactly one smudge
    fn mirror_match(lines1: &[Vec<char>], lines2: &[Vec<char>]) -> bool {
        let mut count_smudges: u32 = 0;
        for (line1, line2) in lines1.iter().zip(lines2.iter().rev()) {
            if is_off_by_one(line1, line2) {
                count_smudges += 1;
            } else if line1 != line2 {
                return false;
            }
        }
        count_smudges == 1
    }
}

fn is_off_by_one(line1: &Vec<char>, line2: &Vec<char>) -> bool {
    let count_diffs = line1
        .iter()
        .zip(line2.iter())
        .filter(|(ch1, ch2)| ch1 != ch2)
        .count();

    count_diffs == 1
}

/// Check a candidate reflection line by verifing that outside lines are also mirrored.
fn check_candidate_reflection_line<C>(lines: &Vec<Vec<char>>, candidate: &ReflectionLine) -> bool
where
    C: LineComparator,
{
    let n_rows_before_line = candidate.post_line_index;
    let n_rows_after_line = lines.len() - candidate.post_line_index;
    let match_lenght = std::cmp::min(n_rows_before_line, n_rows_after_line);

    let lines_pre = &lines[(candidate.post_line_index - match_lenght)..candidate.post_line_index];
    let lines_post = &lines[candidate.post_line_index..(candidate.post_line_index + match_lenght)];
    C::mirror_match(lines_pre, lines_post)
}

fn find_reflection_line_inner<C>(
    lines: &Vec<Vec<char>>,
    orientation: Orientation,
) -> Option<ReflectionLine>
where
    C: LineComparator,
{
    // Match neighbouring pairs of rows to identify candidate reflection lines.
    for i in 0..(lines.len() - 1) {
        if C::cmp_line(&lines[i], &lines[i + 1]) {
            let candidate = ReflectionLine {
                post_line_index: i + 1,
                orientation,
            };
            if check_candidate_reflection_line::<C>(lines, &candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

fn find_reflection_line<C>(pattern: &Pattern) -> Option<ReflectionLine>
where
    C: LineComparator,
{
    find_reflection_line_inner::<C>(&pattern.rows, Orientation::Horizontal)
        .or_else(|| find_reflection_line_inner::<C>(&pattern.cols, Orientation::Vertical))
}

/// For each pattern find the reflection line using the supplied LineComparator. Compute the score
/// of each pattern as follows. For horizonal ones, return the number of rows above it multiplied
/// by 100. For vertical ones, return the number of columns to the left of it. Return the sum of
/// these values.
fn process_patterns<C>(patterns: &Vec<Pattern>) -> u32
where
    C: LineComparator,
{
    patterns
        .iter()
        .map(|pattern| {
            let reflection_line = find_reflection_line::<C>(pattern).expect("No reflection line");
            let score = match reflection_line.orientation {
                Orientation::Horizontal => 100 * reflection_line.post_line_index,
                Orientation::Vertical => reflection_line.post_line_index,
            };
            score as u32
        })
        .sum()
}

/// For each pattern find the reflection line. For horizonal ones, return the number of rows above
/// it multiplied by 100. For vertical ones, return the number of columns to the left of it. Return
/// the sum of these values.
pub fn solve_part1(patterns: &Vec<Pattern>) -> u32 {
    process_patterns::<StrictLineComparator>(patterns)
}

/// Flip a single symbol on each pattern that reveals a different reflection line. Then compute the
/// same sum as in part 1.
pub fn solve_part2(patterns: &Vec<Pattern>) -> u32 {
    process_patterns::<SmudgeLineComparator>(patterns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        let patterns = parser::parse_input(&input).unwrap();
        let result = solve_part1(&patterns);
        assert_eq!(result, 405);
        let result = solve_part2(&patterns);
        assert_eq!(result, 400);
    }
}
