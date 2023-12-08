use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;
use core::iter::Repeat;
use itertools::Itertools;
use nom;
use regex;
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

type NodeName = String;

type Graph = std::collections::HashMap<NodeName, (NodeName, NodeName)>;

fn parse_input(infile: &str) -> Option<(String, Graph)> {
    let mut lines = infile.lines();
    let first = lines.next()?.to_string();
    let mut graph = Graph::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (name, rhs) = line.split_once(" = ").unwrap();
        let (left, right) = rhs.strip_prefix('(')?.strip_suffix(')')?.split_once(", ")?;
        graph.insert(name.to_string(), (left.to_string(), right.to_string()));
    }
    Some((first, graph))
}

/// Basically a finite state machine with a step counter
fn part_1(infile: &str) -> usize {
    let (dirs, graph) = parse_input(infile).unwrap();
    println!("{dirs}");
    println!("{graph:?}");

    // We don't quite need a search algo

    let mut step_count = 0;
    let mut cur = "AAA";
    for dir in dirs.chars().cycle() {
        if cur == "ZZZ" {
            break;
        }
        cur = match dir {
            'L' => &graph.get(cur).unwrap().0,
            'R' => &graph.get(cur).unwrap().1,
            _ => unimplemented!(),
        };
        step_count += 1;
    }
    step_count
}
fn part_2(infile: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part_1_example_1() {
        assert_eq!(part_1(EXAMPLE_1), 2);
    }
    #[test]
    fn part_1_example_2() {
        assert_eq!(part_1(EXAMPLE_2), 6);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), todo!());
    }
}
