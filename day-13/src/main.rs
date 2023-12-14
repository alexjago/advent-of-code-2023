/*!
Find a reflection point within columns (i.e. a horizontal reflection).

Process: a reflection point (between columns) exists when the columns
equidistant from the point are equal.

A reflection is anchored by one or both edges of the grid:

    bd..........
    bbdd........
    bbbddd......
    bbbbdddd....
    bbbbbddddd..
    bbbbbbdddddd

So to find a reflection, we:
- set the rightmost edge of "b" as each of 0..len()-1
- set d = b+1
- if !=, break
- if equal, b-- and d++ until one hits an edge

What happens if different columns are locally reflective?
Only the largest reflection (in the sense of distance from the edge) is correct?

No.

We need to take an intersection:

       12 34 56 78
    A: xo ox xx xx    local reflections at 2|3 (left), 6|7 (right) and 7|8 (right)
    B: xo ox xo ox    local reflections at 2|3 (left), 4|5 (both) and 6|7 (right)
    C: ox xo xx xx    local reflections at 2|3 (left), 6|7 (right) and 7|8 (right)

In the above example, the only shared reflection point in all three rows is at 2|3

*/

use anyhow::Result;
use clap::Parser;
use mapgrid::{Coord, Grid};
use nom;
use regex;
use std::{collections::HashSet, fs::read_to_string};
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
    /*!
    We're looking for reflections in patterns in a grid.

    Reflection can be on the horizontal or the vertical axis
    **/

    infile
        .split("\n\n")
        .map(|pat| {
            let vert = find_reflection_col(pat).map(|x| x + 1).unwrap_or(0);
            let horz = find_reflection_row(pat).map(|x| x + 1).unwrap_or(0);

            // println!("{pat}\n{horz}\t{vert}");

            100 * horz + vert
        })
        .sum()
}

/// Find a reflection by columns ("vertical")
/// newline-delimited strings are inherently row-major
fn find_reflection_col(input: &str) -> Option<usize> {
    let rows: Vec<Vec<char>> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let width = rows.iter().map(|r| r.len()).min().unwrap_or(0);
    let height = rows.len();

    let mut out: Option<usize> = None;

    for start in 0..(width - 1) {
        let mut b = start;
        let mut d = start + 1;

        loop {
            // Test for column equality
            if rows.iter().any(|r| r[b] != r[d]) {
                break;
            }
            // Have b or d hit the edge?
            if b == 0 || d >= width - 1 {
                // println!("Hit the edges at {b} {d}")
                if out.is_none() || (start > out.unwrap() && start < height - out.unwrap()) {
                    // println!("Adding vertical reflection at {start}   ({b} {d})");
                    out = Some(start);
                }
                break;
            }
            b -= 1;
            d += 1;
        }
    }

    out
}

/// Find a reflection by rows ("horizontal")
fn find_reflection_row(input: &str) -> Option<usize> {
    // we can compare rows at a time, lol

    let rows: Vec<&str> = input.lines().filter(|s| !s.is_empty()).collect();

    let mut out: Option<usize> = None;
    for start in 0..(rows.len() - 1) {
        let mut b = start;
        let mut d = start + 1;
        loop {
            if rows[b] != rows[d] {
                break;
            }
            if b == 0 || d >= rows.len() - 1 {
                if out.is_none() || (start > out.unwrap() && start < rows.len() - out.unwrap()) {
                    out = Some(start);
                }
                break;
            }
            b -= 1;
            d += 1;
        }
    }
    out
}

fn part_2(infile: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Vertical reflection between cols 5 & 6 (1-indexed)
    const EXAMPLE_1_A: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    /// Horizontal reflection between rows 4 & 5 (1-indexed)
    const EXAMPLE_1_B: &str = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_row_find() {
        assert_eq!(find_reflection_row(EXAMPLE_1_B).unwrap() + 1, 4);
        assert_eq!(find_reflection_row(EXAMPLE_1_A), None);
    }

    #[test]
    fn test_col() {
        assert_eq!(find_reflection_col(EXAMPLE_1_A).unwrap() + 1, 5);
        assert_eq!(find_reflection_col(EXAMPLE_1_B), None);
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(&format!("{EXAMPLE_1_A}\n\n{EXAMPLE_1_B}")), 405);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1_A), todo!());
    }
}
