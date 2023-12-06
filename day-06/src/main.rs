use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;
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

    let input = parse_input_1(&infile);

    println!("{input:?}");

    println!("Part 1:\n{}", part_1(&infile)?);
    println!("Part 2:\n{}", part_2(&infile)?);

    Ok(())
}

fn part_1(infile: &str) -> Result<usize> {
    let input = parse_input_1(infile);

    Ok(input
        .iter()
        .map(|r| {
            strategise(r.time)
                .iter()
                .filter(|d| **d > r.distance)
                .count()
        })
        .product())
}
fn part_2(infile: &str) -> Result<usize> {
    todo!()
}

fn parse_input_1(infile: &str) -> Vec<Race> {
    // two lines, columns are associative
    let lines: Vec<&str> = infile.lines().collect();

    let times: Vec<usize> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let dists: Vec<usize> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    times
        .iter()
        .zip(dists.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Race {
    time: usize,
    distance: usize,
}

/// For each whole ms you spend holding the button,
/// the boat's speed increases by 1 ms/s
/// but you only have `total_time` to hold and go
/// returns a vec of distances where the index is the # of ms spent holding
fn strategise(total_time: usize) -> Vec<usize> {
    (0..=total_time).map(|t| (total_time - t) * t).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1).unwrap(), 288);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1).unwrap(), todo!());
    }
}
