//! https://adventofcode.com/2023/day/2

pub mod input_parser;

#[derive(Clone, Debug)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    pub fn new() -> Self {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn red(count: u32) -> Self {
        CubeSet {
            red: count,
            green: 0,
            blue: 0,
        }
    }

    pub fn green(count: u32) -> Self {
        CubeSet {
            red: 0,
            green: count,
            blue: 0,
        }
    }

    pub fn blue(count: u32) -> Self {
        CubeSet {
            red: 0,
            green: 0,
            blue: count,
        }
    }

    pub fn is_subset(&self, other: &CubeSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    pub fn union(&mut self, other: &CubeSet) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<CubeSet>,
}

fn possible_games(games: &Vec<Game>, bag_configuration: &CubeSet) -> Vec<Game> {
    games
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.is_subset(bag_configuration))
        })
        .cloned()
        .collect()
}

// Return the sum of possible game ids for a given bag configuration
pub fn solve_part1(games: &Vec<Game>, bag_configuration: &CubeSet) -> u32 {
    possible_games(games, bag_configuration)
        .iter()
        .map(|game| game.id)
        .sum()
}

fn min_bag_config_for_game(game: &Game) -> CubeSet {
    game.draws
        .iter()
        .fold(CubeSet::new(), |mut bag_config, cubeset| {
            if cubeset.red > bag_config.red {
                bag_config.red = cubeset.red;
            }
            if cubeset.green > bag_config.green {
                bag_config.green = cubeset.green;
            }
            if cubeset.blue > bag_config.blue {
                bag_config.blue = cubeset.blue;
            }
            bag_config
        })
}

// Return the sum of powers of minimum cubesets for each game. The power of a cubeset is equal to
// the product of red, green, and blue cubes.
pub fn solve_part2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let bag_config = min_bag_config_for_game(game);
            let power = bag_config.red * bag_config.green * bag_config.blue;
            power
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        let games = input_parser::parse_input(input).unwrap();
        let bag_config = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let result = solve_part1(&games, &bag_config);
        assert_eq!(result, 8);

        let result = solve_part2(&games);
        assert_eq!(result, 2286);
    }
}
