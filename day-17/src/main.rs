use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Returns the opposing direction
    fn opp(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

/// Riffing off the binary heap docs
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    point: Point,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
// CREDIT: https://doc.rust-lang.org/std/collections/binary_heap/index.html
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// If you step from A to B, in which direction are you moving?
/// -[row] = North, -[col] = West
fn angle(a: [usize; 2], b: [usize; 2]) -> Direction {
    use std::cmp::Ordering::*;
    use Direction::*;
    match [a[0].cmp(&b[0]), a[1].cmp(&b[1])] {
        [Less, Equal] => North,
        [Greater, Equal] => South,
        [Equal, Less] => West,
        [Equal, Greater] => East,
        _ => unreachable!(),
    }
}

fn step(r: usize, c: usize, d: Direction) -> Option<[usize; 2]> {
    use Direction::*;
    let out = match d {
        North => [r.checked_sub(1)?, c],
        South => [r + 1, c],
        West => [r, c.checked_sub(1)?],
        East => [r, c + 1],
    };
    Some(out)
}

/// row, column, number of steps taken so far, in Direction
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    row: usize,
    col: usize,
    run: usize,
    dir: Direction,
}

fn part_1(infile: &str) -> usize {
    use crate::Direction::*;
    let grid: Vec<Vec<u32>> = infile
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        // .map(|v| {
        //     println!("{v:?}");
        //     v
        // })
        .collect();

    let cmax = grid[0].len();
    let rmax = grid.len();

    // row, column, number of steps : heat loss in path
    // to know how many steps we've taken also depends on which direction we're going
    // this takes some more thought

    // ALSO, this is basically shortest-path, and has the same issue
    // of needing to consider shortest-tentative-cost first.
    let mut done: HashMap<Point, u32> = HashMap::new();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    // The path is symmetrical so it doesn't matter which direction we build in
    // We don't need to count the cost of the starting block, but it might be easier to just subtract that off
    // When you turn, you have travelled 0 blocks in your new direction (see example trace RH edge)

    // for k in 1..=3 {
    for d in [North, West, East, South] {
        let p = Point {
            row: 0,
            col: 0,
            run: 0,
            dir: d,
        };
        done.insert(p, grid[p.row][p.col]);
        queue.push(State {
            cost: grid[p.row][p.col],
            point: p,
        });
    }
    // }

    // standard dynamic programming from here?

    while let Some(cur) = queue.pop() {
        // we need to try each neighbour - we might be cheaper for them
        let point = cur.point;

        for d in [North, South, East, West] {
            if let Some(p) = step(point.row, point.col, d) {
                // - OOB in Row/Col => skip
                // - run >= 4 => skip
                // - directional reversal => skip
                // - run == 0 => force same-direction (initial condition)
                if p[0] < rmax
                    && p[1] < cmax
                    && point.run < 4
                    && (d != point.dir.opp())
                    && (point.run > 0 || point.dir == d)
                    && p != [point.row, point.col]
                {
                    // n is our neighbouring point
                    let n = Point {
                        row: p[0],
                        col: p[1],
                        run: if point.dir == d { point.run + 1 } else { 1 },
                        dir: d,
                    };

                    // simple adjacency
                    let maybe_cost = grid[p[0]][p[1]] + cur.cost;
                    let existing_cost = *done.get(&n).unwrap_or(&u32::MAX);

                    if maybe_cost < existing_cost {
                        done.insert(n, maybe_cost);

                        queue.push(State {
                            point: n,
                            cost: maybe_cost,
                        });
                    }
                }
            }
        }
    }

    // println!("{done:#?}");
    [North, South, East, West]
        .into_iter()
        .cartesian_product(0..=3_usize)
        .filter_map(|(d, r)| {
            let p = done.get(&Point {
                row: rmax - 1,
                col: cmax - 1,
                run: r,
                dir: d,
            });
            // println!("(run: {r} dir: {d:?}) -> {p:?}");
            p
        })
        .min()
        .map(|&x| x - grid[0][0])
        .unwrap() as usize
}

fn part_2(infile: &str) -> usize {
    use crate::Direction::*;
    let grid: Vec<Vec<u32>> = infile
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        // .map(|v| {
        //     println!("{v:?}");
        //     v
        // })
        .collect();

    let cmax = grid[0].len();
    let rmax = grid.len();

    // row, column, number of steps : heat loss in path
    // to know how many steps we've taken also depends on which direction we're going
    // this takes some more thought

    // ALSO, this is basically shortest-path, and has the same issue
    // of needing to consider shortest-tentative-cost first.
    let mut done: HashMap<Point, u32> = HashMap::new();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    // The path is symmetrical so it doesn't matter which direction we build in
    // We don't need to count the cost of the starting block, but it might be easier to just subtract that off
    // When you turn, you have travelled 0 blocks in your new direction (see example trace RH edge)

    // for k in 1..=3 {
    for d in [North, West, East, South] {
        let p = Point {
            row: 0,
            col: 0,
            run: 0,
            dir: d,
        };
        done.insert(p, grid[p.row][p.col]);
        queue.push(State {
            cost: grid[p.row][p.col],
            point: p,
        });
    }
    // }

    // standard dynamic programming from here?

    while let Some(cur) = queue.pop() {
        // we need to try each neighbour - we might be cheaper for them
        let point = cur.point;

        for d in [North, South, East, West] {
            if let Some(p) = step(point.row, point.col, d) {
                // - OOB in Row/Col => skip
                // - directional reversal => skip
                // - run == 0 => force same-direction (initial condition)
                // Ultra Crucibles *must* run at least 4 in the same direction
                // but have a limit of 10 before turning
                if p[0] < rmax
                    && p[1] < cmax
                    && (d != point.dir.opp())
                    && (point.run >= 4 || point.dir == d)
                    && p != [point.row, point.col]
                    && point.run <= 10
                {
                    // n is our neighbouring point
                    let n = Point {
                        row: p[0],
                        col: p[1],
                        run: if point.dir == d { point.run + 1 } else { 1 },
                        dir: d,
                    };

                    // simple adjacency
                    let maybe_cost = grid[p[0]][p[1]] + cur.cost;
                    let existing_cost = *done.get(&n).unwrap_or(&u32::MAX);

                    if maybe_cost < existing_cost {
                        done.insert(n, maybe_cost);

                        queue.push(State {
                            point: n,
                            cost: maybe_cost,
                        });
                    }
                }
            }
        }
    }

    // println!("{done:#?}");
    [North, South, East, West]
        .into_iter()
        .cartesian_product(4..=10_usize)
        .filter_map(|(d, r)| {
            let p = done.get(&Point {
                row: rmax - 1,
                col: cmax - 1,
                run: r,
                dir: d,
            });
            // println!("(run: {r} dir: {d:?}) -> {p:?}");
            p
        })
        .min()
        .map(|&x| x - grid[0][0])
        .unwrap() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 102);
    }

    const SIMPLE: &str = r"111
991
121
199
191
121
991";

    #[test]
    fn part_1_simple() {
        assert_eq!(part_1(SIMPLE), 14);
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(part_2(EXAMPLE_1), 94);
    }
    const EXAMPLE_2: &str = r"111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part_2_example_2() {
        assert_eq!(part_2(EXAMPLE_2), 71);
    }
}
