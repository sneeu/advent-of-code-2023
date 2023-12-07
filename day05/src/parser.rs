use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self as cc, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mapping {
    pub from: Range<u64>,
    pub delta: u64,
}

#[derive(Debug)]
pub struct Map {
    pub name: Option<String>,
    pub mappings: Vec<Mapping>,
}

fn num(i: &str) -> IResult<&str, u64> {
    map_res(cc::digit1, str::parse)(i)
}

fn parse_mapping(i: &str) -> IResult<&str, Mapping> {
    let (i, (to, _, from, _, length)) = tuple((num, space1, num, space1, num))(i)?;

    Ok((
        i,
        Mapping {
            from: from..from + length,
            delta: to - from,
        },
    ))
}

fn mappings(i: &str) -> IResult<&str, Vec<Mapping>> {
    let (i, mappings) = separated_list1(tag("\n"), parse_mapping)(i)?;

    let mut new_mappings = mappings.clone();
    new_mappings.sort_by(|a, b| a.from.start.cmp(&b.from.start));

    Ok((i, new_mappings))
}

fn parse_map(i: &str) -> IResult<&str, Map> {
    let (i, (name, _, mappings)) = tuple((take_until(" map:\n"), tag(" map:\n"), mappings))(i)?;

    Ok((
        i,
        Map {
            name: Some(name.to_string()),
            mappings,
        },
    ))
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<u64>> {
    let (i, (_, seeds)) = tuple((tag("seeds: "), separated_list1(space1, num)))(i)?;

    Ok((i, seeds))
}

pub fn parse_maps(i: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    let (i, (seeds, _, mappings)) = tuple((
        parse_seeds,
        tag("\n\n"),
        separated_list1(tag("\n\n"), parse_map),
    ))(i)?;

    Ok((i, (seeds, mappings)))
}
