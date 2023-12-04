use anyhow::Result;
use clap::Parser;


use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;


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

fn part_1(infile: &str) -> Result<usize> {
    let pat = regex::Regex::new(r"\d+")?;

    let mut total: usize = 0;

    for (_i, line) in infile.lines().enumerate() {
        let wins = line.split_once('|').unwrap().0.split_once(':').unwrap().1;
        let cands = line.split_once('|').unwrap().1;

        let winners = pat
            .find_iter(wins)
            .filter_map(|s| s.as_str().parse().ok())
            .collect::<HashSet<usize>>();

        let match_count = pat
            .find_iter(cands)
            .filter_map(|s| s.as_str().parse().ok())
            .filter(|x| winners.contains(x))
            .count();

        if match_count > 0 {
            total += 1 << match_count.saturating_sub(1);
        }

    Ok(total)
}
fn part_2(infile: &str) -> Result<usize> {
    let pat = regex::Regex::new(r"\d+")?;

    let mut total: usize = 0;

    // copy id, count
    let mut copy_counts: HashMap<usize, usize> = HashMap::new();

    for (id, line) in infile.lines().enumerate() {
        let wins = line.split_once('|').unwrap().0.split_once(':').unwrap().1;
        let cands = line.split_once('|').unwrap().1;

        let winners = pat
            .find_iter(wins)
            .filter_map(|s| s.as_str().parse().ok())
            .collect::<HashSet<usize>>();

        let match_count = pat
            .find_iter(cands)
            .filter_map(|s| s.as_str().parse().ok())
            .filter(|x| winners.contains(x))
            .count();

        let multiplier: usize = *copy_counts.get(&id).unwrap_or(&1);
        total += multiplier;

        if match_count > 0 {
            for k in (id + 1)..=(id + match_count) {
                *copy_counts.entry(k).or_insert(1) += multiplier;
            }
        }

    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1).unwrap(), 13);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1).unwrap(), 30);
    }
}
