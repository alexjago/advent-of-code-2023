use anyhow::Result;
use clap::Parser;
use regex::RegexBuilder;
use std::fs::read_to_string;

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Hello, world!");

    println!("Part 1:\n{}", part_1(&infile)?);
    println!("Part 2:\n{}", part_2(&infile)?);

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let parser = RegexBuilder::new(r"\d").build()?;

    let out = input
        .lines()
        .map(|s| {
            parser
                .find_iter(s)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
        })
        .filter(|m| !m.is_empty())
        .map(|s| (s[0], s[s.len() - 1]))
        .map(|(x, y)| format!("{}{}", x, y))
        .filter_map(|x| x.parse::<usize>().ok())
        .sum();
    Ok(out)
}

fn digitmap(x: &str) -> usize {
    match x {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0,
    }
}

fn part_2(input: &str) -> Result<usize> {
    // Fuck. We have to deal with overlapping matches.
    // Luckily we only need the very first and very last match...
    let parser_fwd =
        RegexBuilder::new(r"(\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
            .build()?;

    let parser_rev =
        RegexBuilder::new(r"(\d)|(enin)|(thgie)|(neves)|(xis)|(evif)|(ruof)|(eerht)|(owt)|(eno)")
            .build()?;

    let out = input
        .lines()
        .map(|s| dbg!(s))
        .filter_map(|s| {
            let x = parser_fwd.find(s)?.as_str();

            let rev: String = s.chars().rev().collect();

            let y: String = parser_rev.find(&rev)?.as_str().chars().rev().collect();

            Some(digitmap(x) * 10 + digitmap(&y))
        })
        .sum();
    Ok(out)
}

pub const SAMPLE_PART_1_INPUT: &str = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

pub const SAMPLE_PART_2_INPUT: &str = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
1
";

/// 18, 21, 38, 58, 79, 82, 98
/// total 394
pub const OVERLAPS: &str = r"
oneight
twone
threeight
fiveight
sevenine
eightwo
nineight
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_sample() {
        let rez = dbg!(part_1(SAMPLE_PART_1_INPUT).unwrap());
        assert!(rez == 142);
    }
    #[test]
    fn part_2_sample() {
        let rez = dbg!(part_2(SAMPLE_PART_2_INPUT).unwrap());
        assert!(rez == 292);
    }

    #[test]
    fn part_2_overlap() {
        let rez = dbg!(part_2(OVERLAPS).unwrap());
        assert!(rez == 394);
    }
}
