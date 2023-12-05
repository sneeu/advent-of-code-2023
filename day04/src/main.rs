mod parser;

use parser::{parse_scratchcards, Scratchcard};

static INPUT: &str = include_str!("../input.txt");

impl Scratchcard {
    fn matches(&self) -> Vec<&u32> {
        self.numbers
            .intersection(&self.winning_numbers)
            .into_iter()
            .collect()
    }
}

fn part1(input: &str) -> u32 {
    let (_, scratchcards) = parse_scratchcards(input).unwrap();

    scratchcards
        .iter()
        .map(|s| s.matches().len())
        .filter(|m| *m > 0)
        .map(|m| 2u32.pow(m as u32 - 1))
        .sum()
}

fn part2(input: &str) -> u32 {
    let (_, scratchcards) = parse_scratchcards(input).unwrap();

    let mut cards = [1; 200];
    let matches: Vec<_> = scratchcards.iter().map(|s| s.matches().len()).collect();

    for (i, s) in matches.iter().enumerate() {
        for _ in 0..cards[i] {
            for win in 0..*s {
                cards[1 + i + win] += 1;
            }
        }
    }

    cards[..scratchcards.len()].iter().sum::<u32>()
}

fn main() {
    println!("Day 04, part I:  {}", part1(INPUT));
    println!("Day 04, part II: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(TEST_INPUT), 30);
    }
}
