mod parser;

use std::ops::Range;

use parser::{parse_maps, Map, Mapping};

static INPUT: &str = include_str!("../input.txt");

impl Mapping {
    fn convert(&self, n: u64) -> u64 {
        let r = self.to + n - self.from;
        r
    }

    fn invert(&self, n: u64) -> u64 {
        n + self.from - self.to
    }

    fn intersection(&self, other: &Range<u64>) -> Option<Range<u64>> {
        Some(0..3)
    }
}

const NULL_MAPPING: Mapping = Mapping {
    from: 0,
    to: 0,
    length: 0,
};

impl Map {
    fn find_mapping(&self, n: u64) -> &Mapping {
        for mapping in &self.mappings {
            if mapping.from <= n && n < mapping.from + mapping.length {
                return mapping;
            }
        }
        &NULL_MAPPING
    }

    fn find_mappings(&self, ns: Range<u64>) -> Vec<Range<u64>> {
        self.mappings.iter().filter_map(|mapping| mapping.intersection(&ns)).collect()
    }
}

fn part1(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_maps(input).unwrap();

    seeds
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(*seed, |acc, e| e.find_mapping(acc).convert(acc))
        })
        .reduce(|acc, e| acc.min(e))
        .unwrap()
}

fn seed_ranges(seeds: &Vec<u64>) -> Vec<Range<u64>> {
    seeds
        .chunks(2)
        .into_iter()
        .map(|c| c[0]..c[0] + c[1])
        .collect()
}

fn part2(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_maps(input).unwrap();
    let seed_ranges = seed_ranges(&seeds);

    0
}

fn main() {
    println!("Day 05, part I:  {}", part1(INPUT));
    println!("Day 05, part II: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    static TEST_INPUT: &str = "seeds: 79 14 55 13

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
";
    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(TEST_INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(TEST_INPUT), 46);
        assert_eq!(crate::part2(TEST_INPUT), 46);
    }

    #[test]
    fn test_reverse_mapping() {
        let (_, (seeds, maps)) = crate::parse_maps(TEST_INPUT).unwrap();
        // let seed_ranges = crate::seed_ranges(&seeds);

        assert_eq!(
            maps.iter()
                .rev()
                .fold(35, |acc, e| e.find_reverse_mapping(acc).invert(acc)),
            13
        );
    }
}
