mod parser;

use std::ops::Range;

use parser::{parse_maps, Map, Mapping};

static INPUT: &str = include_str!("../input.txt");

fn range_intersection<'a, T: Copy + Ord>(a: &'a Range<T>, b: &'a Range<T>) -> Option<Range<T>> {
    if b.end < a.start || a.end < b.start {
        return None;
    }

    Some(b.start.max(a.start)..a.end.min(b.end))
}

impl Mapping {
    fn convert(&self, n: u64) -> u64 {
        n + self.delta
    }

    fn intersection(&self, other: &Range<u64>) -> Option<Mapping> {
        range_intersection(&self.from, other).map(|range| Mapping { from: range, delta: self.delta })
    }
}

impl Map {
    fn find_mapping(&self, n: u64) -> Option<&Mapping> {
        self.mappings
            .iter()
            .find(|&mapping| mapping.from.contains(&n))
    }

    fn reduce_mappings(&self, ns: &Range<u64>) -> Map {
        Map {
            name: None,
            mappings: self.mappings
                .iter()
                .filter_map(|mapping| mapping.intersection(ns))
                .collect()
        }
    }
}

fn part1(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_maps(input).unwrap();

    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, e| {
                e.find_mapping(acc).map(|m| m.convert(acc)).unwrap_or(acc)
            })
        })
        .reduce(|acc, e| acc.min(e))
        .unwrap()
}

fn seed_ranges(seeds: &[u64]) -> Vec<Range<u64>> {
    seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect()
}

fn part2(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_maps(input).unwrap();
    let seed_ranges = seed_ranges(&seeds);

    
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
    fn test_range_intersection() {
        assert_eq!(crate::range_intersection(&(0..10), &(20..30)), None);
        assert_eq!(crate::range_intersection(&(20..30), &(0..10)), None);

        assert_eq!(crate::range_intersection(&(0..30), &(10..20)), Some(10..20));
        assert_eq!(crate::range_intersection(&(0..20), &(10..30)), Some(10..20));
        assert_eq!(crate::range_intersection(&(10..30), &(0..20)), Some(10..20));
    }

    #[test]
    fn test_map_find_mappings() {
        let m = crate::Map {
            name: "Cool".to_string(),
            mappings: vec![
                crate::Mapping {
                    from: 0..10,
                    to: 100..110,
                },
                crate::Mapping {
                    from: 20..40,
                    to: 120..140,
                },
            ],
        };

        assert_eq!(m.find_mappings(&(5..30)), vec![5..10, 20..30]);
    }
}
