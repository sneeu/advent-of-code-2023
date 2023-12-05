use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self as cc, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct Scratchcard {
    pub id: u32,
    pub winning_numbers: HashSet<u32>,
    pub numbers: HashSet<u32>,
}

fn parse_scratchcard(i: &str) -> IResult<&str, Scratchcard> {
    let (i, (_, id, _, wins, _, nums)) = tuple((
        tuple((tag("Card"), space1)),
        cc::u32,
        tuple((tag(":"), space1)),
        separated_list1(space1, cc::u32),
        tuple((space1, tag("|"), space1)),
        separated_list1(space1, cc::u32),
    ))(i)?;

    Ok((
        i,
        Scratchcard {
            id,
            winning_numbers: HashSet::from_iter(wins),
            numbers: HashSet::from_iter(nums),
        },
    ))
}

pub fn parse_scratchcards(i: &str) -> IResult<&str, Vec<Scratchcard>> {
    separated_list1(tag("\n"), parse_scratchcard)(i)
}
