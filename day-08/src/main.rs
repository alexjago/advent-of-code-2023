use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;

use num_integer::Integer;

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
    // println!("{dirs}");
    // println!("{graph:?}");

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

/// Basically a finite state machine with a step counter
fn part_2(infile: &str) -> usize {
    let (dirs, graph) = parse_input(infile).unwrap();

    let end_a_nodes: Vec<String> = graph
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(String::clone)
        .collect();

    // We don't quite need a search algo
    // It might be heat death of the universe to simulate this directly
    // Let's simulate each one separately and take the LCM

    let mut step_counts: Vec<usize> = vec![];

    for n in &end_a_nodes {
        let mut step_count = 0;
        let mut cur = n;
        for dir in dirs.chars().cycle() {
            if cur.ends_with('Z') {
                break;
            }
            cur = match dir {
                'L' => &graph.get(cur).unwrap().0,
                'R' => &graph.get(cur).unwrap().1,
                _ => unimplemented!(),
            };
            step_count += 1;
        }
        step_counts.push(step_count)
    }
    println!("{end_a_nodes:?}");
    println!("step_counts:?");

    step_counts
        .into_iter()
        // .filter_map(FromPrimitive::from_usize)
        .reduce(|acc, e| acc.lcm(&e))
        .unwrap()
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

    const EXAMPLE_3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

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
        assert_eq!(part_2(EXAMPLE_3), 6);
    }
}
