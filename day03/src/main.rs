use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    slice::Iter,
};

use regex::Regex;

static INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    w: usize,
}

impl Position {
    fn xs(&self, border: usize) -> Range<usize> {
        (self.x.max(border) - border)..(self.x + self.w + border)
    }
    fn ys(&self, border: usize) -> Range<usize> {
        (self.y.max(border) - border)..(self.y + 1 + border)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Part {
    number: u32,
    position: Position,
    length: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Symbol {
    symbol: char,
    position: Position,
}

trait Positioned {
    fn position(&self) -> Position;

    fn surrounding(&self, size: usize) -> Vec<Position> {
        let mut r = vec![];
        for x in self.position().xs(size) {
            for y in self.position().ys(size) {
                r.push(Position { x, y, w: 1 })
            }
        }
        r
    }
}

impl Positioned for Part {
    fn position(&self) -> Position {
        self.position
    }
}

impl Positioned for Symbol {
    fn position(&self) -> Position {
        self.position
    }
}

struct Things<T>
where
    T: Positioned,
{
    indexed: Vec<T>,
    hashed: HashMap<Position, usize>,
}

impl<T: Positioned> Things<T> {
    fn from(things: Vec<T>) -> Self {
        let mut indexed = vec![];
        let mut hashed = HashMap::new();

        for t in things {
            for position in t.surrounding(0) {
                hashed.insert(position, indexed.len());
            }
            indexed.push(t);
        }

        Self { indexed, hashed }
    }

    fn find(&self, position: &Position) -> Option<&T> {
        self.hashed.get(position).map(|index| &self.indexed[*index])
    }

    fn iter(&self) -> Iter<'_, T> {
        self.indexed.iter()
    }
}

struct Engine {
    parts: Things<Part>,
    symbols: Things<Symbol>,
}

impl Engine {
    fn load(input: &str) -> Self {
        let part_numbers_regex = Regex::new(r"\d+").unwrap();

        let mut ps = vec![];
        let mut ss = vec![];

        for (y, line) in input.lines().enumerate() {
            ps.extend(part_numbers_regex.find_iter(line).map(|m| {
                let number = m.as_str().parse().unwrap();
                let x = m.start();
                let length = m.len();
                Part {
                    number,
                    position: Position { x, y, w: length },
                    length,
                }
            }));
            for (x, c) in line.chars().enumerate() {
                if !c.is_numeric() && c != '.' {
                    ss.push(Symbol {
                        symbol: c,
                        position: Position { x, y, w: 1 },
                    });
                }
            }
        }

        Engine {
            parts: Things::from(ps),
            symbols: Things::from(ss),
        }
    }

    fn surrounding_things<'a, T: Positioned>(
        &'a self,
        a: &dyn Positioned,
        bs: &'a Things<T>,
    ) -> HashSet<&T>
    where
        T: std::hash::Hash,
        T: Eq,
        T: Copy,
    {
        a.surrounding(1)
            .iter()
            .filter_map(|position| bs.find(position))
            .collect()
    }

    fn surrounding_symbols(&self, part: &Part) -> Option<&Symbol> {
        self.surrounding_things(part, &self.symbols)
            .into_iter()
            .next()
    }

    fn surrounding_parts(&self, s: &Symbol) -> HashSet<&Part> {
        self.surrounding_things(s, &self.parts)
    }
}

fn part1(input: &str) -> u32 {
    let e = Engine::load(input);

    e.parts
        .iter()
        .filter_map(|part| e.surrounding_symbols(part).map(|_| part.number))
        .sum()
}

fn part2(input: &str) -> u32 {
    let e = Engine::load(input);

    e.symbols
        .iter()
        .filter(|sy| sy.symbol == '*')
        .map(|sy| e.surrounding_parts(sy))
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().map(|p| p.number).product::<u32>())
        .sum()
}

fn main() {
    println!("Day 03, part I:  {}", part1(INPUT));
    println!("Day 03, part II: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(TEST_INPUT), 467835);
    }
}
