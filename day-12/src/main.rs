use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;
use regex::Regex;
use strum;
use winnow;

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
    let mut total = 0;
    for line in infile.lines() {
        let (conds, counts) = parse_part_1(line);
        // println!("\n{line}");
        // println!("{conds:?}\t{counts:?}");
        let matches: Vec<usize> = conds_to_usizes(&conds)
            .into_iter()
            .filter(|b| match_report(*b, &counts))
            .collect();
        // for bbb in &matches {
        //     println!("\t{:064b}", bbb);
        // }
        // println!("\t{} matches", matches.len());
        total += matches.len();
    }
    total
}
fn part_2(infile: &str) -> usize {
    todo!()
}

/// For each row, the condition records show every spring and whether it is operational (.) or damaged (#).
/// This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.
///
/// After the list of springs for a given row, the size of each contiguous group of damaged springs is listed in the order those groups appear in the row.
/// groups are always separated by at least one operational spring: #### would always be 4, never 2,2
///
/// Some rows have several possible arrangements
///
/// Anyway, we need
fn parse_part_1(row: &str) -> (Vec<Condition>, Vec<usize>) {
    let (springs, nums) = row.split_once(' ').unwrap();
    let springpat = Regex::new(r"(\#+)|(\.+)|(\?+)").unwrap();

    let conds = springpat
        .find_iter(springs)
        .map(|m| match &m.as_str()[..1] {
            "#" => Condition::Damaged(m.len()),
            "." => Condition::Operational(m.len()),
            "?" => Condition::Unknown(m.len()),
            _ => unimplemented!(),
        })
        .collect();

    let counts = nums
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    (conds, counts)
}

/// determine if a match
fn match_report(bits: usize, spec: &Vec<usize>) -> bool {
    // count bits

    if bits == 0 {
        if spec.is_empty() {
            return true;
        } else {
            return false;
        }
    }
    let mut bits = bits >> bits.trailing_zeros();

    for v in spec {
        if bits == 0 {
            return false;
        }
        bits >>= bits.trailing_zeros();

        let d = bits.trailing_ones() as usize;

        if d == *v {
            bits >>= d;
            continue;
        } else {
            return false;
        }
    }

    bits.count_ones() == 0
}

/// Panics if input.len() > 64
/// Collates all the various possibilities for the conditions
/// as a bitstring ([0] matches LSB)
/// 1 = damaged, 0 = other
fn conds_to_usizes(input: &Vec<Condition>) -> Vec<usize> {
    use Condition::*;
    if input.len() > 64 {
        panic!("too large an input!");
    }
    let mut bitstring = 0_usize;

    let mut idx = 0;

    // Set all bits to 1 where it is known-damaged
    for c in input {
        match &c {
            Damaged(n) => {
                bitstring |= (0xffffffffffffffff >> (64 - n)) << idx;
                idx += n;
            }
            Operational(n) | Unknown(n) => {
                idx += n;
            }
        }
    }

    let mut out = vec![bitstring];
    idx = 0;

    for c in input {
        match &c {
            Unknown(n) => {
                for i in idx..(idx + n) {
                    for b in out.clone() {
                        out.push(b | 1 << i)
                    }
                }
                idx += n;
            }
            Damaged(n) | Operational(n) => idx += n,
        }
    }

    out
}

#[derive(Debug, PartialEq, Eq)]
/// usize is length of contiguous springs with that condition
enum Condition {
    Operational(usize),
    Damaged(usize),
    Unknown(usize),
}

#[cfg(test)]
mod test {
    use super::*;

    const ALL_KNOWN: &str = r"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

    const EXAMPLE_1: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_parse_1() {
        use Condition::*;
        assert_eq!(
            parse_part_1("???.### 1,1,3"),
            (vec![Unknown(3), Operational(1), Damaged(3)], vec![1, 1, 3]),
        );
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 21);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), todo!());
    }
}
