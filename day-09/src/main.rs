use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
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

    println!("Part 1:\n{}", part_1(&infile));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

/// Sequence of differences
fn part_1(infile: &str) -> isize {
    infile
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            println!("\n{line}");
            let mut diffs: HashMap<usize, Vec<isize>> = HashMap::new();

            diffs.insert(
                0,
                line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            );

            let mut curr_level = 0_usize;

            while diffs
                .get(&curr_level)
                .is_some_and(|sx| sx.iter().any(|&x| x != 0))
            {
                // Take successive differences
                let next_level = diffs
                    .get(&curr_level)
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(p, n)| n - p)
                    .collect();
                curr_level += 1;
                diffs.insert(curr_level, next_level);
            }

            let mut nv = 0;
            let mut cv = 0;
            for c in (0..curr_level).rev() {
                cv = nv;
                nv = cv + *diffs.get(&c).unwrap().last().unwrap_or(&0_isize);
                println!("level: {c}\tprevious: {cv}\tnext: {nv}");
            }
            nv
        })
        .sum()
}

/// Sequence of differences
fn part_2(infile: &str) -> isize {
    infile
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            println!("\n{line}");
            let mut diffs: HashMap<usize, Vec<isize>> = HashMap::new();

            diffs.insert(
                0,
                line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            );

            let mut curr_level = 0_usize;

            while diffs
                .get(&curr_level)
                .is_some_and(|sx| sx.iter().any(|&x| x != 0))
            {
                // Take successive differences
                let next_level = diffs
                    .get(&curr_level)
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(p, n)| n - p)
                    .collect();
                curr_level += 1;
                diffs.insert(curr_level, next_level);
            }

            let mut nv = 0;
            let mut cv = 0;
            for c in (0..curr_level).rev() {
                cv = nv;
                nv = *diffs.get(&c).unwrap().first().unwrap_or(&0_isize) - cv;
                println!("level: {c}\tprevious: {cv}\tnext: {nv}");
            }
            nv
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 114);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 2);
    }
}
