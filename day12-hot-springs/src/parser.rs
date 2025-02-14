use crate::{Record, Status};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{one_of, space1, u32},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), u32)(input)
}

fn parse_statuses(input: &str) -> IResult<&str, Vec<Status>> {
    let (remainder, statuses) = many1(one_of(".#?"))(input)?;

    let statuses = statuses
        .into_iter()
        .map(|ch| Status::from(ch))
        .collect();

    Ok((remainder, statuses))
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    let (remainder, (statuses, cluster_sizes)) =
        separated_pair(parse_statuses, space1, parse_number_list)(input)?;

    let record = Record {
        seq: statuses,
        cluster_sizes,
    };

    Ok((remainder, record))
}

pub fn parse_input(input: &str) -> Result<Vec<Record>, Box<dyn std::error::Error + '_>> {
    let records = input
        .lines()
        .map(|line| parse_record(line).map(|(_, record)| record))
        .try_collect()?;

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn record() {
        let (_, record) = parse_record("#?????###??#?. 1,7,2").unwrap();
        assert_matches!(record, Record { .. });
    }
}
