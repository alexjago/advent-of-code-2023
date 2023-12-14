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

    println!("Part 1:\n{}", part_1(&infile));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str) -> usize {
    // functionally equivalent to segmenting within each column
    // then summing (row, row-1, ...) for as many O as within that segment
    let rows: Vec<Vec<char>> = infile
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let width = rows.iter().map(|r| r.len()).min().unwrap_or(0);
    let height = rows.len();

    let mut total = 0;

    for c in 0..width {
        let mut score = height;
        for r in 0..height {
            match rows[r][c] {
                '#' => score = (height - r).saturating_sub(1),
                'O' => {
                    total += score;
                    score -= 1;
                }
                _ => {}
            }
            // print!("{} ({r}, {c}): {total}\t", rows[r][c]);
        }
        // println!("");
    }
    total
}
fn part_2(infile: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    /// O can roll in any of the four cardinal directions, # are fixed, . are empty
    const EXAMPLE_1: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 136);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), todo!());
    }
}
