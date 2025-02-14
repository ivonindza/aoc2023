use crate::{CubeSet, Game};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_red(input: &str) -> IResult<&str, CubeSet> {
    let (remainder, count) = terminated(u32, tag(" red"))(input)?;
    Ok((remainder, CubeSet::red(count)))
}

fn parse_green(input: &str) -> IResult<&str, CubeSet> {
    let (remainder, count) = terminated(u32, tag(" green"))(input)?;
    Ok((remainder, CubeSet::green(count)))
}

fn parse_blue(input: &str) -> IResult<&str, CubeSet> {
    let (remainder, count) = terminated(u32, tag(" blue"))(input)?;
    Ok((remainder, CubeSet::blue(count)))
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
    let (remainder, colors) =
        separated_list1(tag(", "), alt((parse_red, parse_green, parse_blue)))(input)?;

    let mut cubeset = CubeSet::new();
    for color in colors {
        cubeset.union(&color);
    }

    Ok((remainder, cubeset))
}

fn parse_cube_set_list(input: &str) -> IResult<&str, Vec<CubeSet>> {
    separated_list1(tag("; "), parse_cube_set)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (remainder, _) = tag("Game ")(input)?;
    let (remainder, (game_id, cube_set_list)) =
        separated_pair(u32, tag(": "), parse_cube_set_list)(remainder)?;

    let game = Game {
        id: game_id,
        draws: cube_set_list,
    };

    Ok((remainder, game))
}

pub fn parse_input(input: &str) -> Result<Vec<Game>, Box<dyn std::error::Error + '_>> {
    let games = input
        .lines()
        .map(|line| parse_game(line).map(|(_, game)| game))
        .try_collect()?;

    Ok(games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_color() {
        let (_, cubeset) = parse_red("1 red").unwrap();
        assert!(matches!(
            cubeset,
            CubeSet {
                red: 1,
                green: 0,
                blue: 0,
            }
        ));

        let (_, cubeset) = parse_green("10 green").unwrap();
        assert!(matches!(
            cubeset,
            CubeSet {
                red: 0,
                green: 10,
                blue: 0,
            }
        ));

        let (_, cubeset) = parse_blue("0 blue").unwrap();
        assert!(matches!(
            cubeset,
            CubeSet {
                red: 0,
                green: 0,
                blue: 0,
            }
        ));
    }

    #[test]
    fn cube_set() {
        let input = "5 blue";
        let (_, cubeset) = parse_cube_set(input).unwrap();
        assert!(matches!(
            cubeset,
            CubeSet {
                red: 0,
                green: 0,
                blue: 5,
            }
        ));

        let input = "5 blue, 6 red";
        let (_, cubeset) = parse_cube_set(input).unwrap();
        assert!(matches!(
            cubeset,
            CubeSet {
                red: 6,
                green: 0,
                blue: 5,
            }
        ));

        let input = "4 red, 5 blue, 6 green";
        let (_, cubeset) = parse_cube_set(input).unwrap();
        assert!(matches!(
            cubeset,
            CubeSet {
                red: 4,
                green: 6,
                blue: 5,
            }
        ));
    }

    #[test]
    fn cube_set_list() {
        let input = "4 red, 5 blue, 6 green; 10 blue";
        let (_, cubeset_list) = parse_cube_set_list(input).unwrap();
        assert!(matches!(
            cubeset_list[..],
            [
                CubeSet {
                    red: 4,
                    green: 6,
                    blue: 5,
                },
                CubeSet {
                    red: 0,
                    green: 0,
                    blue: 10,
                },
            ]
        ));
    }

    #[test]
    fn game() {
        let input = "Game 10: 4 red, 5 blue, 6 green; 10 blue";
        let (_, game) = parse_game(input).unwrap();
        assert!(matches!(game, Game { id: 10, .. }));
    }
}
