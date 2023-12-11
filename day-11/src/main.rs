use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::{Range, RangeInclusive},
};

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
    let expando = expand_universe(infile);

    // now for each pair of galaxies, we need the shortest path, and thence the sum thereof

    let mut listy: Vec<(usize, usize)> = expando.into_iter().collect();
    listy.sort();

    let mut total = 0_usize;

    for (j, a) in listy.iter().enumerate() {
        for b in listy.iter().skip(j) {
            let x = a.0.abs_diff(b.0);
            let y = a.1.abs_diff(b.1);

            total += x + y;
        }
    }
    total
}
fn part_2(infile: &str) -> usize {
    todo!()
}

fn bounds(input: &HashSet<(usize, usize)>) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let xmax = input.iter().map(|v| v.0).max().unwrap_or(0);
    let ymax = input.iter().map(|v| v.1).max().unwrap_or(0);
    let xmin = input.iter().map(|v| v.0).min().unwrap_or(0);
    let ymin = input.iter().map(|v| v.1).min().unwrap_or(0);

    (xmin..=xmax, ymin..=ymax)
}

fn visualise(input: &HashSet<(usize, usize)>) {
    let (xs, ys) = bounds(input);

    for y in ys {
        for x in xs.clone() {
            if input.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn load_universe(infile: &str) -> HashSet<(usize, usize)> {
    //! Galaxy: `#`
    infile
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn expand_universe(infile: &str) -> HashSet<(usize, usize)> {
    let galaxy_orig = load_universe(infile);
    let (xs, ys) = bounds(&galaxy_orig);
    let (xmin, xmax) = (*xs.start(), *xs.end());
    let (ymin, ymax) = (*ys.start(), *ys.end());

    let no_galaxy_col: Vec<usize> = (xmin..=xmax)
        .filter(|x| !(ymin..=ymax).any(|y| galaxy_orig.contains(&(*x, y))))
        .collect();
    let no_galaxy_row: Vec<usize> = (ymin..=ymax)
        .filter(|y| !(xmin..=xmax).any(|x| galaxy_orig.contains(&(x, *y))))
        .collect();

    // println!("Rows without galaxies:\t{no_galaxy_row:?}");
    // println!("Cols without galaxies:\t{no_galaxy_col:?}");

    // if a galaxy's position in a coordinate is strictly greater than the coordinates of N rows/columns then its new position is +N

    let mut out = HashSet::new();
    for (x, y) in (0..=xmax).flat_map(move |x| (0..=ymax).map(move |y| (x, y))) {
        if galaxy_orig.contains(&(x, y)) {
            let xplus = &no_galaxy_col.iter().filter(|t| **t < x).count();
            let yplus = &no_galaxy_row.iter().filter(|t| **t < y).count();
            // println!("Expanding ({x}, {y}) by ({xplus}, {yplus})");

            out.insert((x + xplus, y + yplus));
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const EXPANDO_1: &str = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    #[test]
    fn test_part_1_expando() {
        let mut a = expand_universe(EXAMPLE_1)
            .into_iter()
            .collect::<Vec<(usize, usize)>>();
        a.sort();
        let mut b = load_universe(EXPANDO_1)
            .into_iter()
            .collect::<Vec<(usize, usize)>>();
        b.sort();
        println!("{a:?}");
        println!("{b:?}");

        assert_eq!(a, b);
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 374);
    }

    // #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), todo!());
    }
}
