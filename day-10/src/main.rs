use std::{collections::HashMap, fs::read_to_string};

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
    println!("Part 2:\n{}", part_2(&infile)?);

    Ok(())
}

type Point = [isize; 2];

fn add(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1]]
}

type Grid = std::collections::HashMap<Point, Vec<Point>>;

/// Returns starting point (denoted by S)
/// along with adjecency matrix
fn load_grid(infile: &str) -> (Point, Grid) {
    let mut starting = Point::default();

    let mut grid = Grid::new();

    for (r, row) in infile.lines().enumerate() {
        for (c, tile) in row.chars().enumerate() {
            let me = [r as isize, c as isize];
            let neighbours: (Point, Point) = match tile {
                'S' => {
                    starting = me;
                    continue;
                }
                '.' => continue,
                '|' => ([-1, 0], [1, 0]),  // North & South
                '-' => ([0, -1], [0, 1]),  // West & East
                'L' => ([-1, 0], [0, 1]),  // North & East
                'J' => ([-1, 0], [0, -1]), // North & West
                '7' => ([0, -1], [1, 0]),  // West & South
                'F' => ([0, 1], [1, 0]),   // East & South
                _ => unimplemented!(),
            };

            grid.insert(me, vec![add(neighbours.0, me), add(neighbours.1, me)]);
        }
    }

    let mut s_neighbs = vec![];

    for n in [[-1, 0], [1, 0], [0, -1], [0, 1]].map(|n| add(n, starting)) {
        // println!("{n:?}");
        if let Some(v) = grid.get(&n) {
            for x in v {
                if *x == starting {
                    s_neighbs.push(n)
                }
            }
        }
    }

    s_neighbs.sort();

    grid.insert(starting, vec![s_neighbs[0], s_neighbs[1]]);

    (starting, grid)
}

/// fill a graph from a starting point and return the distance to the origin for each point visitable
fn flood_fill(grid: &Grid, starting: Point) -> HashMap<Point, isize> {
    let mut visited: HashMap<Point, isize> = HashMap::new();

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(starting);
    visited.insert(starting, 0);

    while let Some(here) = queue.pop_front() {
        let neighbs = grid.get(&here).unwrap();
        let dist = *visited.get(&here).unwrap();

        for n in neighbs {
            let e = visited.entry(*n).or_insert(isize::MAX);

            if *e > (dist + 1) {
                *e = dist + 1;
                queue.push_back(*n);
                // println!("At {here:?} ({dist}); queued {n:?} {}", dist + 1);
            }
        }
    }

    visited
}

/// load grid
/// identify starting position
/// swap starting tile for actual tile
/// - note: confirmed no spurious neighbours
/// BFS
/// Get max distance
fn part_1(infile: &str) -> isize {
    // println!("{infile}");
    let (starting, grid) = load_grid(infile);

    // println!("Starting at {starting:?}");
    // println!("First neighbours are {:?}", grid.get(&starting).unwrap());
    // println!("Grid:\n{grid:?}");

    let filled = flood_fill(&grid, starting);

    // println!("{filled:?}");

    *filled.values().max().unwrap_or(&0)
}
fn part_2(_infile: &str) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1B: &str = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const EXAMPLE_1D: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn part_1b() {
        assert_eq!(part_1(EXAMPLE_1B), 4);
    }
    #[test]
    fn part_1d() {
        assert_eq!(part_1(EXAMPLE_1D), 8);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1B).unwrap(), todo!());
    }
}
