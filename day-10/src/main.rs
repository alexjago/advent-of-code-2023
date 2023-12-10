use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

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

type Point = [isize; 2];

fn add(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1]]
}

fn sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1]]
}

fn mul(a: Point, n: isize) -> Point {
    [a[0] * n, a[1] * n]
}
fn div(a: Point, n: isize) -> Point {
    [a[0] / n, a[1] / n]
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

fn neighbours(grid: &Grid, point: Point) -> impl Iterator<Item = (&Point, &Vec<Point>)> {
    [[-1, 0], [1, 0], [0, -1], [0, 1]]
        .iter()
        .map(move |v| add(point, *v))
        .filter_map(|v| grid.get_key_value(&v))
}

/// Run a BFS from the starting point
fn scale_2x(grid: &Grid, starting: Point) -> (Point, Grid) {
    let mut out = Grid::with_capacity(grid.len() * 4);

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(starting);

    while let Some(here) = queue.pop_front() {
        let twobours: Vec<Point> = grid
            .get(&here)
            .unwrap()
            .iter()
            .map(|n| mul(*n, 2))
            .collect();

        for n in twobours {
            if !out.contains_key(&n) {
                queue.push_back(div(n, 2));
            }
            let tween = div(add(mul(here, 2), n), 2);

            out.entry(mul(here, 2)).or_default().push(tween);
            out.entry(n).or_default().push(tween);
            out.insert(tween, vec![here, n]);
        }
    }

    /*
        for p in grid {
            let newbours: Vec<Point> = p.1.iter().map(|[x, y]| [x * 2, y * 2]).collect();
            let [xx, yy] = [p.0[0] * 2, p.0[1] * 2];
            out.entry([xx, yy]).or_insert(newbours.clone());

            for n in newbours
                .iter()
                .map(|n| add([xx, yy], *n))
                .map(|[x, y]| [x / 2, y / 2])
            {}
        }
    */
    (mul(starting, 2), out)
}

fn show_me(grid: &Grid) {
    let xmax = grid.keys().map(|v| v[0]).max().unwrap_or(0);
    let ymax = grid.keys().map(|v| v[1]).max().unwrap_or(0);
    let xmin = grid.keys().map(|v| v[0]).min().unwrap_or(0);
    let ymin = grid.keys().map(|v| v[1]).min().unwrap_or(0);

    println!("x in {xmin}..={xmax}; y in {ymin}..={ymax}\n");

    for x in xmin..=xmax {
        for y in ymin..=ymax {
            if grid.contains_key(&[x, y]) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println! {}
    }
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

/// Identify tiles enclosed by the loop
/// We can have a zero-width squeeze between tiles
/// just not crossing the loop
/// the simplest solution here would be to move to 2x scale
fn part_2(infile: &str) -> usize {
    let (starting1, loop1) = load_grid(infile);
    let (_, loop2) = scale_2x(&loop1, starting1);

    // show_me(&loop1);

    // show_me(&loop2);

    let xmax = loop2.keys().map(|v| v[0]).max().unwrap_or(0);
    let ymax = loop2.keys().map(|v| v[1]).max().unwrap_or(0);
    let xmin = loop2.keys().map(|v| v[0]).min().unwrap_or(0);
    let ymin = loop2.keys().map(|v| v[1]).min().unwrap_or(0);

    let gridsize = xmax.abs_diff(xmin) * ymax.abs_diff(ymin);

    // for each point on the edge, we run a DFS for all neighbours,
    // filtering out ones we've already visited and those we know are in the loop

    let mut queue: VecDeque<Point> = VecDeque::from_iter(
        (xmin..=xmax)
            .flat_map(|x| (ymin..=ymax).map(move |y| [x, y]))
            .filter(|[x, y]| *x == xmin || *x == xmax || *y == ymin || *y == ymax)
            .filter(|k| !loop2.contains_key(k)),
    );

    // println!("Starting with the following points: {queue:?}");

    let mut outside: HashSet<Point> = HashSet::new();

    for k in queue.iter() {
        outside.insert(*k);
    }

    let mut counter = 0;

    while let Some(point) = queue.pop_front() {
        for x in -1..=1 {
            for y in -1..=1 {
                let cand = add(point, [x, y]);

                if !(outside.contains(&cand)
                    || loop2.contains_key(&cand)
                    || cand[0] > xmax
                    || cand[0] < xmin
                    || cand[1] > ymax
                    || cand[1] < ymin)
                {
                    queue.push_back(cand);
                    outside.insert(cand);
                }
            }
        }
        counter += 1;
        if counter > gridsize * 2 {
            panic!("there's probably an infinite loop, we've been running for longer than we should've");
        }
        /*
                for x in xmin..=xmax {
                    for y in ymin..=ymax {
                        if loop2.contains_key(&[x, y]) {
                            print!("X");
                        } else if queue.contains(&[x, y]) {
                            print!("!");
                        } else if outside.contains(&[x, y]) {
                            print!("@");
                        } else {
                            print!(".")
                        }
                    }
                    println!("");
                }
                println!("");
        */
    }
    (xmin..=xmax)
        .flat_map(|x| (ymin..=ymax).map(move |y| [x, y]))
        .filter(|k| !loop2.contains_key(k) && !outside.contains(k))
        .filter(|[x, y]| (x % 2 == 0) && (y % 2 == 0))
        .count()
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

    const EXAMPLE_2A: &str = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE_2B: &str = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

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
        assert_eq!(part_2(EXAMPLE_2B), 4);
    }
}
