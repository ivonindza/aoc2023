//! https://adventofcode.com/2023/day/6

pub mod parser;

#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub distance: u64,
}

/// Find the min charging time with which the boat will beat the record distance.
fn find_min_charging_time(race: &Race) -> Option<u64> {
    for i in 0..race.time {
        let charge_time = i;
        let speed = i;
        let distance = (race.time - charge_time) * speed;
        if distance > race.distance {
            return Some(charge_time);
        }
    }
    None
}

/// Find the max charging time with which the boat will beat the record distance.
fn find_max_charging_time(race: &Race) -> Option<u64> {
    for i in 0..race.time {
        let charge_time = race.time - i;
        let speed = charge_time;
        let distance = (race.time - charge_time) * speed;
        if distance > race.distance {
            return Some(charge_time);
        }
    }
    None
}

/// Compute the number of ways that the record distance can be broken.
fn ways_to_beat_race_record(race: &Race) -> u64 {
    let min_charge = find_min_charging_time(race).expect("Race record unbeatable");
    let max_charge = find_max_charging_time(race).expect("Race record unbeatable");
    max_charge - min_charge + 1
}

/// For each race compute the number of ways that the record distance can be broken. Return the
/// product of these values.
pub fn solve_part1(races: &Vec<Race>) -> u64 {
    races
        .iter()
        .map(|race| ways_to_beat_race_record(race))
        .product()
}

/// Compute the number of ways that the record distance can be broken.
pub fn solve_part2(race: &Race) -> u64 {
    ways_to_beat_race_record(race)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        let races = parser::parse_input_part1(&input).unwrap();
        let result = solve_part1(&races);
        assert_eq!(result, 288);

        let race = parser::parse_input_part2(&input).unwrap();
        let result = solve_part2(&race);
        assert_eq!(result, 71503);
    }
}
