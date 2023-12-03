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
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::BuildHasher;
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
    println!("Part 2:\n{}", part_2(&infile)?);

    Ok(())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Number {
    value: usize,
    x_start: usize,
    x_end: usize,
    y: usize,
}

// 335384 is wrong (deduplicated, ..=x_end)
// 544664 is *correct* (non deduplicated, ..=x_end)
fn part_1(infile: &str) -> Result<usize> {
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();
    let mut numbers: Vec<Number> = Vec::new();
    let parser = Regex::new(r"([0123456789]+|[^0123456789.])")?;

    for (y, line) in infile.lines().enumerate() {
        let matches = parser.find_iter(line);
        for m in matches {
            if let Ok(x) = m.as_str().parse::<usize>() {
                let num = Number {
                    value: x,
                    x_start: m.start(),
                    x_end: m.end(),
                    y,
                };
                // println!("{y} {m:?} {num:?}");
                numbers.push(num);
            } else {
                symbols.insert((m.start(), y));
                // println!("{y} {m:?} symbol");
            }
        }
        // end of line?
        // symbols.insert((line.len(), y));
    }

    let numbers = numbers;
    let symbols = symbols;
    // println!("{numbers:?}");
    // println!("{symbols:?}");
    // dbg!(&numbers);
    // dbg!(&symbols);

    // THERE ARE DUPLICATE PART NUMBERS !?!?!?

    let mut potentials: Vec<usize> = numbers
        .iter()
        .map(|num| {
            for x in num.x_start.saturating_sub(1)..=num.x_end {
                // end is one past already
                for y in num.y.saturating_sub(1)..=(num.y + 1) {
                    // println!("({x}, {y})");
                    if symbols.contains(&(x, y)) {
                        // println!("matched: {num:?} with ({x}, {y})");
                        return num.value;
                    }
                }
            }
            // println!("--- no match for {num:?} ---");
            0
        })
        .collect();

    // println!(
    //     "non-deduplicated count of known part numbers: {}",
    //     potentials.len()
    // );

    potentials.sort();

    // println!("{:?}", potentials);

    let potsum: usize = potentials.iter().sum();
    // println!("non-deduplicated sum: {potsum}");

    // let dedupe: HashSet<usize> = potentials.into_iter().collect();

    // println!(
    //     "deduped: {} numbers, down from {}",
    //     dedupe.len(),
    //     numbers.len()
    // );

    // Ok(dedupe.iter().sum())
    Ok(potsum)
    // I'm so mad that I'm not even gonna delete the code
}
fn part_2(infile: &str) -> Result<usize> {
    let mut gears: HashSet<(usize, usize)> = HashSet::new();
    let mut numbers: HashMap<(usize, usize), Number> = HashMap::new();
    let parser = Regex::new(r"([0123456789]+|[^0123456789.])")?;

    for (y, line) in infile.lines().enumerate() {
        let matches = parser.find_iter(line);
        for m in matches {
            if let Ok(x) = m.as_str().parse::<usize>() {
                let num = Number {
                    value: x,
                    x_start: m.start(),
                    x_end: m.end(),
                    y,
                };
                for k in m.start()..m.end() {
                    numbers.insert((k, y), num);
                }
            } else if m.as_str() == "*" {
                gears.insert((m.start(), y));
            }
        }
    }

    let numbers = numbers;
    let gears = gears;

    let mut total = 0;
    for g in gears {
        let mut adj = HashSet::new();
        for x in g.0.saturating_sub(1)..=(g.0 + 1) {
            for y in g.1.saturating_sub(1)..=(g.1 + 1) {
                if let Some(n) = numbers.get(&(x, y)) {
                    adj.insert(n);
                }
            }
        }
        if adj.len() == 2 {
            // println!("gear: {g:?} with ratios {adj:?}");
            total += adj.iter().map(|n| n.value).product::<usize>();
        }
    }

    Ok(total)
}

// numbers: have a line number (y coordinate) and a range (x coordinates)
// symbols: have an x and a y coordinate
// if a number has an adjacent symbol with Ysym in +/- 1 and Xsym in +/- 1
// we add it to total
// absolute line numbers don't matter so we can use lines().enumerate()
//

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1).unwrap(), 4361);
    }

    const DUPES_1: &str = r"467..467..
...*......
..35..467.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1_dupes() {
        assert_eq!(part_1(DUPES_1).unwrap(), 4361 - (633 - 467));
    }

    const EOL_1: &str = r"467..114..
...*......
..35..6333
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1_eol() {
        assert_eq!(part_1(EOL_1).unwrap(), 4361 + (6333 - 633));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1).unwrap(), 467835);
    }
}
