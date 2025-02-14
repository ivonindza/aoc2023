//! https://adventofcode.com/2023/day/5

use rangemap::map::RangeMap;
use std::ops::Range;

pub mod parser;

/// Each RangeMap maps from the source range to the offset from the source range to the destination
/// range (i.e. dst_range_start - src_range_start).
#[derive(Debug)]
pub struct Configuration {
    seeds: Vec<i64>,
    seed_to_soil: RangeMap<i64, i64>,
    soil_to_fert: RangeMap<i64, i64>,
    fert_to_water: RangeMap<i64, i64>,
    water_to_light: RangeMap<i64, i64>,
    light_to_temp: RangeMap<i64, i64>,
    temp_to_humid: RangeMap<i64, i64>,
    humid_to_location: RangeMap<i64, i64>,
}

fn map_seed_to_location(seed: i64, cfg: &Configuration) -> i64 {
    let mut val = seed;
    for map in [
        &cfg.seed_to_soil,
        &cfg.soil_to_fert,
        &cfg.fert_to_water,
        &cfg.water_to_light,
        &cfg.light_to_temp,
        &cfg.temp_to_humid,
        &cfg.humid_to_location,
    ] {
        val = match map.get(&val) {
            Some(offset) => val + offset,
            None => val,
        };
    }
    val
}

fn shift_range(range: Range<i64>, offset: i64) -> Range<i64> {
    Range {
        start: range.start + offset,
        end: range.end + offset,
    }
}

/// Compute the overlap of `range` with `overlap_range`. Return a tuple of three elements, where
/// the first element is the part of the `range` before the `overlap_range` starts, the second
/// element is the overlap of the two ranges, and the third element is the part of the `range`
/// after the `overlap_range` ends.
fn overlap_range(
    range: &Range<i64>,
    overlap_range: &Range<i64>,
) -> (Option<Range<i64>>, Option<Range<i64>>, Option<Range<i64>>) {
    use itertools::Itertools;

    let before = Range {
        start: range.start,
        end: std::cmp::min(range.end, overlap_range.start),
    };
    let overlap = Range {
        start: std::cmp::max(range.start, overlap_range.start),
        end: std::cmp::min(range.end, overlap_range.end),
    };
    let after = Range {
        start: std::cmp::max(range.start, overlap_range.end),
        end: range.end,
    };

    [before, overlap, after]
        .into_iter()
        .map(|range| if range.is_empty() { None } else { Some(range) })
        .collect_tuple()
        .unwrap()
}

/// Map from a range into a vector of ranges using the RangeMap. The input range is divided into
/// subranges based on the overlap with the key ranges in the map. Subranges of the input range
/// that are found in the map are offet accoding to the offset value in the map. Subranges of the
/// input range that are not found in the map remain unchanged.
fn map_range(range: Range<i64>, map: &RangeMap<i64, i64>) -> Vec<Range<i64>> {
    let mut ranges: Vec<Range<i64>> = Vec::new();
    let mut current_range = Some(range.clone());

    for (key_range, offset) in map.overlapping(&range) {
        match current_range {
            None => {
                break;
            },
            Some(current_range_inner) => {
                let (before, overlap, after) = overlap_range(&current_range_inner, &key_range);
                if let Some(before_range) = before {
                    ranges.push(before_range);
                }
                if let Some(overlap_range) = overlap {
                    ranges.push(shift_range(overlap_range, *offset));
                }
                current_range = after;
            },
        }
    }

    if let Some(remainder_range) = current_range {
        ranges.push(remainder_range);
    }

    ranges
}

fn map_seed_ranges_to_location_ranges(
    seed_ranges: Vec<Range<i64>>,
    cfg: &Configuration,
) -> Vec<Range<i64>> {
    let mut ranges = seed_ranges;
    for map in [
        &cfg.seed_to_soil,
        &cfg.soil_to_fert,
        &cfg.fert_to_water,
        &cfg.water_to_light,
        &cfg.light_to_temp,
        &cfg.temp_to_humid,
        &cfg.humid_to_location,
    ] {
        ranges = ranges
            .into_iter()
            .map(|range| map_range(range, map))
            .flatten()
            .collect();
    }
    ranges
}

/// Map from the initial seeds to their locations using the 7 range maps in the order: seed, soil,
/// fertilizer, water, light, temperature, humidity, location. Return the min location.
pub fn solve_part1(cfg: &Configuration) -> i64 {
    cfg.seeds
        .iter()
        .map(|seed| map_seed_to_location(*seed, cfg))
        .min()
        .expect("Error: No locations")
}

/// Treat the seeds as seed ranges, where the first number is the start of the range and the second
/// the length of the range. Then map the seed ranges into location ranges using the 7 maps and
/// finally return the min location.
pub fn solve_part2(cfg: &Configuration) -> i64 {
    let seed_ranges: Vec<Range<i64>> = cfg
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let range_start = chunk[0];
            let range_len = chunk[1];
            Range {
                start: range_start,
                end: range_start + range_len,
            }
        })
        .collect();

    map_seed_ranges_to_location_ranges(seed_ranges, &cfg)
        .into_iter()
        .map(|range| range.start)
        .min()
        .expect("Error: No locations")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn overlap_range() {
        // |---|
        //       |---|
        let (before, overlap, after) = super::overlap_range(&(10..20), &(5..10));
        assert_eq!(before, None);
        assert_eq!(overlap, None);
        assert_eq!(after, Some(10..20));

        // |---|
        //    |---|
        let (before, overlap, after) = super::overlap_range(&(10..20), &(5..15));
        assert_eq!(before, None);
        assert_eq!(overlap, Some(10..15));
        assert_eq!(after, Some(15..20));

        // |---------|
        //    |---|
        let (before, overlap, after) = super::overlap_range(&(10..20), &(5..30));
        assert_eq!(before, None);
        assert_eq!(overlap, Some(10..20));
        assert_eq!(after, None);

        //     |-|
        //    |---|
        let (before, overlap, after) = super::overlap_range(&(10..20), &(12..15));
        assert_eq!(before, Some(10..12));
        assert_eq!(overlap, Some(12..15));
        assert_eq!(after, Some(15..20));

        //      |---|
        //    |---|
        let (before, overlap, after) = super::overlap_range(&(10..20), &(15..25));
        assert_eq!(before, Some(10..15));
        assert_eq!(overlap, Some(15..20));
        assert_eq!(after, None);

        //          |---|
        //    |---|
        let (before, overlap, after) = super::overlap_range(&(10..20), &(25..30));
        assert_eq!(before, Some(10..20));
        assert_eq!(overlap, None);
        assert_eq!(after, None);
    }

    #[test]
    fn test1() {
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        let cfg = parser::parse_input(&input).unwrap();
        let result = solve_part1(&cfg);
        assert_eq!(result, 35);
        let result = solve_part2(&cfg);
        assert_eq!(result, 46);
    }
}
