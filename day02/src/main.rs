mod parser;

use parser::{parse_games, Colour, Game};

static INPUT: &str = include_str!("../input.txt");

impl Game {
    fn min_cubes(&self, c: Colour) -> u32 {
        self.rounds
            .iter()
            .flat_map(|round| {
                round.iter().filter_map(|show| {
                    if show.colour == c {
                        Some(show.number)
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap_or(0)
    }
}

fn part1(input: &str) -> u32 {
    let (_, games) = parse_games(input).unwrap();

    games
        .iter()
        .filter(|g| {
            g.min_cubes(Colour::Red) <= 12
                && g.min_cubes(Colour::Green) <= 13
                && g.min_cubes(Colour::Blue) <= 14
        })
        .map(|g| g.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    let (_, games) = parse_games(input).unwrap();

    let colours = [Colour::Red, Colour::Green, Colour::Blue];

    games
        .iter()
        .map(|game| {
            colours
                .clone()
                .map(|c| game.min_cubes(c))
                .iter()
                .product::<u32>()
        })
        .sum()
}

fn main() {
    println!("Day 2, part I:  {}", part1(INPUT));
    println!("Day 2, part II: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    static TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2286);
    }
}
