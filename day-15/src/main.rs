use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;

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
    infile.trim().split(',').map(hash).sum()
}

fn hash(input: &str) -> usize {
    let mut out = 0;

    for c in input.chars() {
        out += c as usize;
        out *= 17;
        out %= 256;
    }
    out
}

fn part_2(infile: &str) -> usize {
    // so we're implementing a hashmap with linear probing, right?

    let mut boxes: Vec<Vec<Entry>> = vec![vec![]; 256];

    for lens in infile.trim().split(',') {
        if let Some(label) = lens.strip_suffix('-') {
            let addr = hash(label);
            boxes[addr].retain(|f| f.key != label);
        } else {
            let (label, focal) = lens.split_once('=').unwrap();
            // check if existing
            let mut done = false;
            let addr = hash(label);
            for e in &mut boxes[addr] {
                if e.key == label {
                    e.value = focal.parse().unwrap();
                    done = true;
                }
            }
            if !done {
                boxes[addr].push(Entry {
                    key: label.to_string(),
                    value: focal.parse().unwrap(),
                });
            }
        }
        // println!("{lens} :\n{boxes:?}");
    }

    let mut total = 0;
    for (box_num, boxn) in boxes.iter().enumerate() {
        for (slot_num, lens) in boxn.iter().enumerate() {
            total += (1 + box_num) * (1 + slot_num) * lens.value;
        }
    }
    total
}

#[derive(Clone, Debug)]
struct Entry {
    key: String,
    value: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 1320);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 145);
    }
}
