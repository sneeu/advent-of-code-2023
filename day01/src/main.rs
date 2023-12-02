static INPUT: &str = include_str!("../input.txt");

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .filter(|c| char::is_numeric(*c))
        .map(|c| (c as u8) - 48)
        .collect()
}

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line2(line: &str) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];

    let mut i = 0;

    while i < line.len() {
        let c = line[i..i + 1].parse::<u8>();

        if let Ok(d) = c {
            digits.push(d);
        } else {
            for (index, word) in NUMBERS.iter().enumerate() {
                let end = (i + word.len()).min(line.len());
                if line[i..end] == **word {
                    digits.push((index + 1) as u8);
                    // This is annoying, digits _can_ overlap:
                    // oneight twone threeight fiveight sevenine eightwo eighthree nineight
                    i += word.len() - 2;
                    break;
                }
            }
        }

        i += 1;
    }

    digits
}

fn part(input: &str, parser: impl Fn(&str) -> Vec<u8>) -> u32 {
    input
        .split(|c: char| c == '\n')
        .map(parser)
        .map(|digits| {
            format!(
                "{}{}",
                digits.first().unwrap_or(&0),
                digits.last().unwrap_or(&0)
            )
            .parse()
            .unwrap_or(0)
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn part1(input: &str) -> u32 {
    part(input, parse_line)
}

fn part2(input: &str) -> u32 {
    part(input, parse_line2)
}

fn main() {
    println!("Day 1, part I:  {}", part1(INPUT));
    println!("Day 1, part II: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(crate::part1(input), 142);
    }

    #[test]
    fn test_parse_line2() {
        assert_eq!(crate::parse_line2("two1nine"), [2, 1, 9]);
        assert_eq!(crate::parse_line2("eightwothree"), [8, 2, 3]);
        assert_eq!(crate::parse_line2("abcone2threexyz"), [1, 2, 3]);
        assert_eq!(crate::parse_line2("4nineeightseven2"), [4, 9, 8, 7, 2]);
        assert_eq!(crate::parse_line2("zoneight234"), [1, 8, 2, 3, 4]);
        assert_eq!(crate::parse_line2("7pqrstsixteen"), [7, 6]);

        assert_eq!(crate::parse_line2("144six"), [1, 4, 4, 6]);
        assert_eq!(
            crate::parse_line2("kbjtmgfrx3mpmjhncfl78nine"),
            [3, 7, 8, 9]
        );
        assert_eq!(crate::parse_line2("7sgnlbdfivecxz"), [7, 5]);
        assert_eq!(crate::parse_line2("oneqrbbnrdxgbbfl3"), [1, 3]);
        assert_eq!(crate::parse_line2("8ndmrfggfz1six87"), [8, 1, 6, 8, 7]);
        assert_eq!(
            crate::parse_line2("hjkfb8vhrhnlmbhbl59rxplvmgzspfour"),
            [8, 5, 9, 4]
        );
        assert_eq!(crate::parse_line2("3sixjhdn4hckqsnvseven"), [3, 6, 4, 7]);
        assert_eq!(crate::parse_line2("zmkgmlpfsixxhmv25bqlgm5"), [6, 2, 5, 5]);
        assert_eq!(crate::parse_line2("three48eighttwo"), [3, 4, 8, 8, 2]);
        assert_eq!(
            crate::parse_line2("rgfzfourbmpxzrh6dfjcdkhqhcdkpfpk"),
            [4, 6]
        );
        assert_eq!(
            crate::parse_line2("bpccbcqmlstwos8threenineeightg8"),
            [2, 8, 3, 9, 8, 8]
        );
        assert_eq!(crate::parse_line2("nbcpd2prckbshrbvsmrmlhxdkq"), [2]);
        assert_eq!(crate::parse_line2("6flfsxv"), [6]);
        assert_eq!(crate::parse_line2("84xqeightseven"), [8, 4, 8, 7]);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(crate::part2(input), 281);

        let input2 = "144six\nkbjtmgfrx3mpmjhncfl78nine\n7sgnlbdfivecxz\noneqrbbnrdxgbbfl3\n8ndmrfggfz1six87\nhjkfb8vhrhnlmbhbl59rxplvmgzspfour\n3sixjhdn4hckqsnvseven\nzmkgmlpfsixxhmv25bqlgm5\nthree48eighttwo\nrgfzfourbmpxzrh6dfjcdkhqhcdkpfpk\nbpccbcqmlstwos8threenineeightg8\nnbcpd2prckbshrbvsmrmlhxdkq\n6flfsxv\n84xqeightseven\n";
        assert_eq!(
            crate::part2(input2),
            16 + 39 + 75 + 13 + 87 + 84 + 37 + 65 + 32 + 46 + 28 + 22 + 66 + 87
        );
    }
}
