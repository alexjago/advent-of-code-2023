use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

use anyhow::Result;
use clap::Parser;
use itertools::join;
use nom;
use regex;
use std::str::FromStr;
use strum::{self, Display, EnumString};

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

/// Do some ray tracing!
/// Note: if the beam *starts* on a block which would change its direction, its direction should be changed.
/// (A previous implementation didn't do this...)
fn part_1(infile: &str) -> usize {
    let grid: Vec<Vec<Tile>> = infile
        .lines()
        .map(|s| s.chars().map(|c| Tile::try_from(c).unwrap()).collect())
        .collect();

    let rmin = 0_isize;
    let rmax = grid.iter().map(|s| s.len()).max().unwrap_or_default() as isize;
    let cmin = 0_isize;
    let cmax = grid.len() as isize;

    // Now that we have our grid, it's time to raytrace.

    // (Position, Direction)
    let mut done: HashSet<(Coord, Coord)> = HashSet::new();
    let mut queue: VecDeque<(Coord, Coord)> = VecDeque::new();

    queue.push_back(([0, 0], [0, 1]));

    while let Some((pos, dir)) = queue.pop_front() {
        if done.contains(&(pos, dir)) || dir == [0, 0] {
            continue;
        }

        if pos[0] < rmin || pos[0] >= rmax || pos[1] < cmin || pos[1] >= cmax {
            continue;
        }
        let next = add(pos, dir);

        match grid[pos[0] as usize][pos[1] as usize] {
            Tile::Empty => queue.push_back((next, dir)),
            Tile::MirrorF => {
                let newdir = [-dir[1], -dir[0]];
                // println!("{next:?}, {newdir:?}");
                queue.push_back((add(pos, newdir), newdir));
            }
            Tile::MirrorB => {
                let newdir = [dir[1], dir[0]];
                // println!("{next:?}, {newdir:?}");
                queue.push_back((add(pos, newdir), newdir));
            }
            Tile::SplitterV => match dir {
                [_, 0] => queue.push_back((next, dir)),
                [0, _] => {
                    queue.push_back((add(pos, [-1, 0]), [-1, 0]));
                    queue.push_back((add(pos, [1, 0]), [1, 0]));
                }
                _ => unimplemented!(),
            },
            Tile::SplitterH => match dir {
                [0, _] => queue.push_back((next, dir)),
                [_, 0] => {
                    queue.push_back((add(pos, [0, -1]), [0, -1]));
                    queue.push_back((add(pos, [0, 1]), [0, 1]));
                }
                _ => unimplemented!(),
            },
        }

        done.insert((pos, dir));

        // if done.len() > 5 {
        //     break;
        // }
    }

    let mut done_vis = vec![vec!['.'; grid[0].len()]; grid.len()];

    for (pos, _dir) in done.iter() {
        done_vis[pos[0] as usize][pos[1] as usize] = '#'
    }

    // for (row, orig) in done_vis.iter().zip(infile.lines()) {
    //     print!("{}", join(row, ""));
    //     println!("\t{orig}")
    // }
    let energised: HashSet<Coord> = done.iter().map(|(p, _)| *p).collect();

    energised.len()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumString, Display)]
enum Tile {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "/")]
    MirrorF,
    #[strum(serialize = "\\")]
    MirrorB,
    #[strum(serialize = "|")]
    SplitterV,
    #[strum(serialize = "-")]
    SplitterH,
}

type Coord = [isize; 2];

fn add(x: Coord, y: Coord) -> Coord {
    [x[0] + y[0], x[1] + y[1]]
}

impl TryFrom<char> for Tile {
    type Error = char;
    fn try_from(c: char) -> std::result::Result<Tile, char> {
        use Tile::*;
        match c {
            '.' => Ok(Empty),
            '/' => Ok(MirrorF),
            '\\' => Ok(MirrorB),
            '|' => Ok(SplitterV),
            '-' => Ok(SplitterH),
            _ => Err(c),
        }
    }
}

fn part_2(infile: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 46);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), todo!());
    }
}

/*
- First attempt at actual for 1 got 8406, which is too high
- Maybe starting on a / means

*/
