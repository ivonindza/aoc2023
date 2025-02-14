use crate::Configuration;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, line_ending, multispace1, not_line_ending, space1},
    multi::{count, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};
use rangemap::map::RangeMap;

fn gobble_line(input: &str) -> IResult<&str, &str> {
    let (remainder, _) = not_line_ending(input)?;
    line_ending(remainder)
}

struct RangeMapping {
    src_range_start: i64,
    dst_range_start: i64,
    range_len: i64,
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, i64)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), parse_number_list)(input)
}

fn parse_range_mapping(input: &str) -> IResult<&str, RangeMapping> {
    let (remainder, (dst_range_start, _, src_range_start, _, range_len)) =
        tuple((i64, char(' '), i64, char(' '), i64))(input)?;

    let mapping = RangeMapping {
        src_range_start,
        dst_range_start,
        range_len,
    };

    Ok((remainder, mapping))
}

fn parse_map(input: &str) -> IResult<&str, RangeMap<i64, i64>> {
    let (remainder, mappings) = preceded(
        gobble_line,
        separated_list1(line_ending, parse_range_mapping),
    )(input)?;

    let mut range_map = RangeMap::new();
    for mapping in mappings {
        let range_start = mapping.src_range_start;
        let range_end = range_start + mapping.range_len;
        let offset = mapping.dst_range_start - mapping.src_range_start;
        range_map.insert(range_start..range_end, offset);
    }

    Ok((remainder, range_map))
}

pub fn parse_input(input: &str) -> Result<Configuration, Box<dyn std::error::Error + '_>> {
    let (remainder, seeds) = parse_seeds(input)?;
    let (_, mut maps) = count(preceded(multispace1, parse_map), 7)(remainder)?;

    let cfg = Configuration {
        seeds,
        humid_to_location: maps.remove(6),
        temp_to_humid: maps.remove(5),
        light_to_temp: maps.remove(4),
        water_to_light: maps.remove(3),
        fert_to_water: maps.remove(2),
        soil_to_fert: maps.remove(1),
        seed_to_soil: maps.remove(0),
    };

    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Range;
    use indoc::indoc;

    #[test]
    fn seeds() {
        let (_, numbers) = parse_seeds("seeds: 22 34 9").unwrap();
        assert!(matches!(numbers[..], [22, 34, 9]));
    }

    #[test]
    fn rangemap() {
        let (_, map) = parse_map(indoc! {"
            soil-to-fertilizer map:
            0 15 37
            37 52 2
        "})
        .unwrap();

        let as_vec = map.iter().collect::<Vec<_>>();
        assert!(matches!(as_vec[..], [(Range { start: 15, end: 54 }, -15)]));
    }
}
