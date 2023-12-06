static INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> u32 {
    0
}

fn main() {
    println!("Day XXX, part I:  {}", part1(INPUT));
    println!("Day XXX, part II: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(crate::part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(crate::part2(input), 0);
    }
}