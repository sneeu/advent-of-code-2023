use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, space0},
    combinator::value,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Vec<Show>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Show {
    pub number: u32,
    pub colour: Colour,
}

fn parse_colour(i: &str) -> IResult<&str, Colour> {
    alt((
        value(Colour::Red, tag("red")),
        value(Colour::Green, tag("green")),
        value(Colour::Blue, tag("blue")),
    ))(i)
}

fn parse_show(i: &str) -> IResult<&str, Show> {
    let (i, (number, _, colour)) = tuple((cc::u32, space0, parse_colour))(i)?;
    Ok((i, Show { number, colour }))
}

fn parse_round(i: &str) -> IResult<&str, Vec<Show>> {
    let (i, (_, shows)) = tuple((space0, separated_list1(tag(", "), parse_show)))(i)?;

    Ok((i, shows))
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    let (i, (_, id, _)) = tuple((tag("Game "), cc::u32, tag(": ")))(i)?;

    let (i, rounds) = separated_list1(tag("; "), parse_round)(i)?;

    Ok((i, Game { id, rounds }))
}

pub fn parse_games(i: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tag("\n"), parse_game)(i)
}

#[cfg(test)]
mod test {
    use super::{parse_colour, parse_round, parse_show, Colour, Show};

    #[test]
    fn test_parsers() {
        assert_eq!(parse_colour("red"), Ok(("", Colour::Red)));
        assert_eq!(parse_colour("green"), Ok(("", Colour::Green)));
        assert_eq!(parse_colour("blue"), Ok(("", Colour::Blue)));

        assert_eq!(
            parse_show("3 blue"),
            Ok((
                "",
                Show {
                    number: 3,
                    colour: Colour::Blue
                }
            ))
        );
        assert_eq!(
            parse_show("14 red"),
            Ok((
                "",
                Show {
                    number: 14,
                    colour: Colour::Red
                }
            ))
        );

        assert_eq!(
            parse_round("4 green, 2 red"),
            Ok((
                "",
                vec![
                    Show {
                        number: 4,
                        colour: Colour::Green
                    },
                    Show {
                        number: 2,
                        colour: Colour::Red
                    }
                ]
            ))
        );
    }
}
