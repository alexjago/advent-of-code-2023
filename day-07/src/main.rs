use std::fs::read_to_string;

use anyhow::{Context, Result};
use clap::Parser;
use counter::Counter;
use itertools;
use nom;
use regex;
use strum;

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

#[derive(Debug, PartialEq, Eq)]
struct Hand(String);

impl Ord for Hand {
    /// 5 of a kind beats 4x beats 3x beats 2x beats 1x
    /// Ace > King > Queen ...
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        let this = &self.0.chars().collect::<Counter<_>>().most_common_ordered();
        let that = other
            .0
            .chars()
            .collect::<Counter<_>>()
            .most_common_ordered();

        match this[0].1.cmp(&that[0].1) {
            Less => Less,
            Greater => Greater,
            // Equal => Hand::high_card(&self.0, &other.0),
            Equal => match this[1].1.cmp(&that[1].1) {
                // Distinguish between Full House and Three of a Kind, and Two Pair and One Pair
                Less => Less,
                Greater => Greater,
                Equal => Hand::as_number(&self.0).cmp(&Hand::as_number(&other.0)),
            },
        }
    }
}

impl Hand {
    /// Two up to Ace
    pub const STRENGTHS: &str = "23456789TJQKA";
    /// Compares hands left to right to establish which has the higher card
    fn high_card(this: &str, that: &str) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        if this.is_empty() && !that.is_empty() {
            Ordering::Less
        } else if that.is_empty() && !this.is_empty() {
            Ordering::Greater
        } else if this.is_empty() && that.is_empty() {
            Ordering::Equal
        } else {
            match Hand::STRENGTHS
                .find(this.chars().next().unwrap())
                .cmp(&Hand::STRENGTHS.find(that.chars().next().unwrap()))
            {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Hand::high_card(&this[1..], &that[1..]),
            }
        }
    }

    // print as a base-14 number (so that 2 through 10 can be 2 through 10)
    fn as_number(this: &str) -> usize {
        if this.is_empty() {
            return 0;
        }
        let mut out: usize = 0;
        for s in this.chars() {
            out *= 100;
            out += Hand::STRENGTHS.find(s).expect("illegal card");
        }
        out
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// As is tradition, the sample passes but
/// 249407921 is wrong
fn part_1(infile: &str) -> Result<usize> {
    let mut input: Vec<(Hand, usize)> = infile
        .lines()
        .filter_map(|s| s.split_once(' '))
        .map(|(h, b)| (Hand(h.to_string()), b.parse().unwrap()))
        .collect();

    input.sort_unstable();

    let maybe: usize = input
        .iter()
        .enumerate()
        .map(|(i, x)| {
            println!("{}\t{}\t{}", i + 1, x.0 .0, x.1);
            (i, x)
        })
        .map(|(i, (_, b))| (i + 1) * b)
        .sum();

    // 249407921 is too low btw
    if [249407921_usize].contains(&maybe) {
        anyhow::bail!("known-bad value {maybe} in part 1");
    }

    Ok(maybe)
}
fn part_2(infile: &str) -> Result<usize> {
    todo!()
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

    // #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1).unwrap(), todo!());
    }
}
