use anyhow::Result;
use clap::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::Finish;
use nom::IResult;
use std::fs::read_to_string;
use std::str::FromStr;
use strum::EnumString;

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile)?);
    // println!("Part 2:\n{}", part_2(&infile)?);

    Ok(())
}

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, EnumString, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Color {
    Red,
    Green,
    Blue,
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn colour(input: &str) -> IResult<&str, Color> {
    map_res(alt((tag("red"), tag("green"), tag("blue"))), |s: &str| {
        Color::from_str(s)
    })(input)
}

impl From<Vec<(u32, Color)>> for Handful {
    fn from(item: Vec<(u32, Color)>) -> Self {
        let mut out = Self::default();

        for (k, c) in item {
            match c {
                Color::Red => out.red += k,
                Color::Green => out.green += k,
                Color::Blue => out.blue += k,
            }
        }
        out
    }
}

const LIMITS_1: Handful = Handful {
    red: 12,
    green: 13,
    blue: 14,
};

fn part_1(input: &str) -> Result<u32> {
    // Game format
    // Game (\d+): (((\d) (blue|green|red),?\s?))(; ((\d) (blue|green|red),?\s?))*

    // working top down...
    // separated pair for the ":"
    //   left: Game N is a tag on "Game "
    //   right: a separated_list1 of...
    //     tag(";")
    //     a separated_list1 of...
    //       tag(", ")
    //       separated pair (" ") of:
    //         left: color
    //         right: qty
    let mut total = 0;

    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        let game_tag = tag("Game ");
        let game_tot = preceded(game_tag, number);
        let dice = separated_pair(number, multispace1, colour);
        let handful = map(separated_list1(tag(", "), dice), Handful::from);
        let set_list = separated_list1(tag("; "), handful);
        let mut full_line = separated_pair(game_tot, tag(": "), set_list);
        let (id, hands) = full_line(l).finish().unwrap().1;
        if hands
            .iter()
            .all(|h| h.red <= LIMITS_1.red && h.green <= LIMITS_1.green && h.blue <= LIMITS_1.blue)
        {
            total += id;
            print!("[Y] ");
        } else {
            print!("[N] ");
        }
        println!("{id}, {hands:?}");
    }

    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let eg_1 = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_1(eg_1).unwrap(), 8);
    }
}
