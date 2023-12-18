use std::{
    collections::{HashSet},
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

fn part_1(infile: &str) -> usize {
    let dig_list: Vec<Vec<&str>> = infile
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();

    let mut coords: HashSet<[isize; 2]> = HashSet::new();

    let mut r: isize = 0;
    let mut c: isize = 0;
    coords.insert([0, 0]);

    for dig in dig_list {
        let run: isize = dig[1].parse().unwrap();
        let [dr, dc] = match dig[0] {
            "U" => [-1, 0],
            "D" => [1, 0],
            "L" => [0, -1],
            "R" => [0, 1],
            _ => unreachable!(),
        };
        for _ in 0..run {
            r += dr;
            c += dc;
            coords.insert([r, c]);
        }
    }

    let rmin = coords.iter().map(|[r, _]| *r).min().unwrap_or(0);
    let rmax = coords.iter().map(|[r, _]| *r).max().unwrap_or(0);
    let cmin = coords.iter().map(|[_, c]| *c).min().unwrap_or(0);
    let cmax = coords.iter().map(|[_, c]| *c).max().unwrap_or(0);

    // if I had a way to get a point which was guaranteed to be inside the polygon, I could do a flood fill...
    let start_point = find_point_in_polygon(&coords).unwrap();
    println!("{rmin} <= r <= {rmax}\t{cmin} < c < {cmax}\tstarting at {start_point:?}");
    let mut queue = vec![start_point];
    coords.insert(start_point);
    let mut counter = 0;

    while let Some(me) = queue.pop() {
        let [r, c] = me;
        for i in -1..=1 {
            for j in -1..=1 {
                let n = [r + i, c + j];
                if n[0] > rmax || n[0] < rmin || n[1] > cmax || n[1] < cmin {
                    continue;
                }
                if !coords.contains(&n) {
                    queue.push(n);
                    coords.insert(n);
                    // println!("{n:?}");
                }
            }
        }
        if counter > rmin.abs_diff(rmax) * cmin.abs_diff(cmax) {
            break;
        }
        counter += 1;
    }

    for r in rmin..=rmax {
        for c in cmin..=cmax {
            if coords.contains(&[r, c]) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }

    coords.len()
}

/// Finds a point in a polygon by iteration over the bounding box
/// Returned point is guaranteed to not be on the edge
fn find_point_in_polygon(polygon: &HashSet<[isize; 2]>) -> Option<[isize; 2]> {
    let rmin = polygon.iter().map(|[r, _]| *r).min().unwrap_or(0);
    let rmax = polygon.iter().map(|[r, _]| *r).max().unwrap_or(0);
    let cmin = polygon.iter().map(|[_, c]| *c).min().unwrap_or(0);
    let cmax = polygon.iter().map(|[_, c]| *c).max().unwrap_or(0);

    for r in rmin + 1..rmax {
        for c in cmin + 1..cmax {
            if is_point_in_polygon([r, c], polygon) && !polygon.contains(&[r, c]) {
                return Some([r, c]);
            }
        }
    }
    None
}

/// Calculate the number of times a scanline crosses to the right
/// (Should be odd if it's inside)
fn is_point_in_polygon(point: [isize; 2], polygon: &HashSet<[isize; 2]>) -> bool {
    let cmax = polygon.iter().map(|[_, c]| *c).max().unwrap_or(0);
    // let cmin = polygon.iter().map(|[_, c]| *c).min().unwrap_or(0);

    let r = point[0];
    let mut count = 0;
    for c in point[1] + 1..=cmax + 1 {
        if polygon.contains(&[r, c]) && !polygon.contains(&[r, c - 1]) {
            count += 1;
        }
    }
    count % 2 == 1
}

fn part_2(infile: &str) -> usize {
    // now our scanline approach will be much too slow
    // Perhaps the Shoelace Formula will prove useful?
    // We even have the points in a specific order around the perimeter!
    // for (x_i, y_i) the total area is equal to 0.5 * sum(i in 1..=n; (y_i * (x_{i-1} - x_{i+1})))

    let mut points: Vec<[isize; 2]> = vec![[0, 0]];

    let mut r = 0;
    let mut c = 0;
    for [i, j] in infile
        .lines()
        .map(|s| s.split_once("(#").unwrap().1)
        .filter_map(|s| s.strip_suffix(')'))
        .map(|s| (isize::from_str_radix(&s[0..5], 16).unwrap(), &s[5..]))
        .map(|(x, s)| match s {
            "0" => [0, x],
            "1" => [x, 0],
            "2" => [0, -x],
            "3" => [-x, 0],
            _ => unreachable!(),
        })
    {
        r += i;
        c += j;
        points.push([r, c]);
    }
    // circularisation not needed
    println!("{points:?}");

    let mut shoelace_total = 0;

    for i in 0..points.len() - 1 {
        let ya = points[i][1];
        let xa = points[i][0];
        let yb = points[i + 1][1];
        let xb = points[i + 1][0];

        shoelace_total += (ya + yb) * (xa - xb);
    }

    // We're almost there. But the shoelace formula assumes zero edge width
    // Let's assume our edge coordinates are actually the centre of the cube
    // Then half of it is inside and half outside
    // at each corner, 3/4 is on one side depending on if it's concave or convex
    // however, the number of convex corners should always be 4 more than the number of concave corners
    // (i.e. +1 to total area)
    // this is because we start with a quadrilateral (4 convex corners) and every change we make adds as many convex as concave
    // tldr add 0.5*perimeter + 1

    let mut perimeter = 0;
    let mut r = 0_isize;
    let mut c = 0_isize;
    for [rr, cc] in points {
        perimeter += r.abs_diff(rr);
        perimeter += c.abs_diff(cc);
        [r, c] = [rr, cc];
    }

    (shoelace_total.unsigned_abs() + perimeter) / 2 + 1
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    /// empty top issue
    const SCANLINE_TEST: &str = "U 10
R 4
D 4
R 4
U 4
R 4
D 10
L 12";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 62);
    }
    #[test]
    fn part_1_scanline() {
        assert_eq!(part_1(SCANLINE_TEST), 131);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 952408144115);
    }
}
