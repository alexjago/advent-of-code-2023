use std::{collections::HashSet, fs::read_to_string};

use anyhow::{Result};
use clap::Parser;
use counter::Counter;





#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile)?);
    println!("Part 2:\n{}", part_2(&infile)?);

    Ok(())
}

pub trait Hand {
    fn score(&self) -> usize;
}

impl std::cmp::Ord for dyn Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for dyn Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for dyn Hand {}

impl PartialEq for dyn Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl HandType {
    fn from_cards(cards: &str) -> HandType {
        let tops = cards.chars().collect::<Counter<_>>().most_common_ordered();
        match tops[0].1 {
            2 => match tops[1].1 {
                2 => Self::TwoPair,
                _ => Self::OnePair,
            },
            3 => match tops[1].1 {
                2 => Self::FullHouse,
                _ => Self::ThreeOfAKind,
            },
            4 => Self::FourOfAKind,
            5 => Self::FiveOfAKind,
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct PartOne(String);

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct PartTwo(String);

impl PartOne {
    const STRENGTH: &str = "23456789TJQKA";
}

impl PartTwo {
    const STRENGTH: &str = "J23456789TQKA";
}

impl Hand for PartOne {
    fn score(&self) -> usize {
        let mut out = HandType::from_cards(&self.0) as usize;
        for c in self.0.chars() {
            out *= 100;
            out += Self::STRENGTH.find(c).expect("Illegal card");
        }
        out
    }
}
impl Hand for PartTwo {
    fn score(&self) -> usize {
        let avail: HashSet<String> = self.0.chars().map(|c| c.to_string()).collect();

        let mut out = avail
            .iter()
            .map(|c| self.0.replace('J', c))
            .map(|s| HandType::from_cards(&s) as usize)
            .max()
            .unwrap_or_default();
        for c in self.0.chars() {
            out *= 100;
            out += Self::STRENGTH.find(c).expect("Illegal card");
        }
        out
    }
}

/// As is tradition, the sample passes but
/// 249407921 is wrong
fn part_1(infile: &str) -> Result<usize> {
    let mut input: Vec<(usize, PartOne, usize)> = infile
        .lines()
        .filter_map(|s| s.split_once(' '))
        .map(|(h, b)| (PartOne(h.to_string()), b.parse().unwrap()))
        .map(|(h, b)| (h.score(), h, b))
        .collect();

    input.sort_unstable();

    let maybe: usize = input
        .iter()
        .enumerate()
        // .map(|(i, x)| {
        //     println!("{}\t{}\t{}\t{}", i + 1, x.0, x.1 .0, x.2);
        //     (i, x)
        // })
        .map(|(i, (_, _, b))| (i + 1) * b)
        .sum();

    // 249407921 is too low btw
    if [249407921_usize].contains(&maybe) {
        anyhow::bail!("known-bad value {maybe} in part 1");
    }

    Ok(maybe)
}
fn part_2(infile: &str) -> Result<usize> {
    let mut input: Vec<(usize, PartTwo, usize)> = infile
        .lines()
        .filter_map(|s| s.split_once(' '))
        .map(|(h, b)| (PartTwo(h.to_string()), b.parse().unwrap()))
        .map(|(h, b)| (h.score(), h, b))
        .collect();

    input.sort_unstable();

    let maybe: usize = input
        .iter()
        .enumerate()
        // .map(|(i, x)| {
        //     println!("{}\t{}\t{}\t{}", i + 1, x.0, x.1 .0, x.2);
        //     (i, x)
        // })
        .map(|(i, (_, _, b))| (i + 1) * b)
        .sum();

    // 248465369 was too high
    // 247687768 was too low
    // 248583384 was too high
    if [248465369, 247687768, 248583384].contains(&maybe) {
        anyhow::bail!("known-bad value {maybe} in part 2")
    }

    Ok(maybe)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1).unwrap(), 6440);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1).unwrap(), 5905);
    }
}
