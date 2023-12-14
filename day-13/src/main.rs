/*!
Find a reflection point within columns (i.e. a horizontal reflection).

Process: a reflection point (between columns) exists when the columns
equidistant from the point are equal.

A reflection is anchored by one or both edges of the grid:

    bd..........
    bbdd........
    bbbddd......
    bbbbdddd....
    bbbbbddddd..
    bbbbbbdddddd

So to find a reflection, we:
- set the rightmost edge of "b" as each of 0..len()-1
- set d = b+1
- if !=, break
- if equal, b-- and d++ until one hits an edge

What happens if different columns are locally reflective?
Only the largest reflection (in the sense of distance from the edge) is correct?

No.

We need to take an intersection:

       12 34 56 78
    A: xo ox xx xx    local reflections at 2|3 (left), 6|7 (right) and 7|8 (right)
    B: xo ox xo ox    local reflections at 2|3 (left), 4|5 (both) and 6|7 (right)
    C: ox xo xx xx    local reflections at 2|3 (left), 6|7 (right) and 7|8 (right)

In the above example, the only shared reflection point in all three rows is at 2|3

*/

use anyhow::Result;
use clap::Parser;

use std::fs::read_to_string;

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
    /*!
    We're looking for reflections in patterns in a grid.

    Reflection can be on the horizontal or the vertical axis
    **/

    infile
        .split("\n\n")
        .map(|pat| {
            let vert = find_reflections_col(pat)
                .iter()
                .map(|x| x + 1)
                .next()
                .unwrap_or(0);
            let horz = find_reflections_row(pat)
                .iter()
                .map(|x| x + 1)
                .next()
                .unwrap_or(0);

            // println!("{pat}\n{horz}\t{vert}");

            100 * horz + vert
        })
        .sum()
}

/// Find a reflection by columns ("vertical")
/// newline-delimited strings are inherently row-major
fn find_reflections_col(input: &str) -> Vec<usize> {
    let rows: Vec<Vec<char>> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let width = rows.iter().map(|r| r.len()).min().unwrap_or(0);
    let _height = rows.len();

    let mut out: Vec<usize> = vec![];

    for start in 0..(width - 1) {
        let mut b = start;
        let mut d = start + 1;

        loop {
            // Test for column equality
            if rows.iter().any(|r| r[b] != r[d]) {
                break;
            }
            // Have b or d hit the edge?
            if b == 0 || d >= width - 1 {
                // println!("Hit the edges at {b} {d}")
                // if out.is_none() || (start > out.unwrap() && start < width - out.unwrap()) {
                // println!("Adding vertical reflection at {start}   ({b} {d})");
                // out = Some(start);
                // }
                out.push(start);
                break;
            }
            b -= 1;
            d += 1;
        }
    }

    out
}

/// Find a reflection by rows ("horizontal")
fn find_reflections_row(input: &str) -> Vec<usize> {
    // we can compare rows at a time, lol

    let rows: Vec<&str> = input.lines().filter(|s| !s.is_empty()).collect();

    let mut out: Vec<usize> = vec![];
    for start in 0..(rows.len() - 1) {
        let mut b = start;
        let mut d = start + 1;
        loop {
            if rows[b] != rows[d] {
                break;
            }
            if b == 0 || d >= rows.len() - 1 {
                // if out.is_none() || (start > out.unwrap() && start < rows.len() - out.unwrap()) {
                // out = Some(start);
                // }
                out.push(start);
                break;
            }
            b -= 1;
            d += 1;
        }
    }
    out
}

/// Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one . or # should be the opposite type.
///
/// In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid.
/// (The old reflection line won't necessarily continue being valid after the smudge is fixed.)
fn part_2(infile: &str) -> usize {
    // I think we can probably just do this exhaustively
    infile
        .split("\n\n")
        .map(|pat| {
            let vert_orig = find_reflections_col(pat).iter().map(|x| x + 1).next();
            let horz_orig = find_reflections_row(pat).iter().map(|x| x + 1).next();

            exh_unsmudge(pat, vert_orig, horz_orig)
        })
        .sum()
}
fn exh_unsmudge(pat: &str, vert_orig: Option<usize>, horz_orig: Option<usize>) -> usize {
    //! There should be exactly one unsmudge op that will result in a different line becoming valid.
    //!     (vert_new, horz_new) != (vert_orig, horz_orig) && (vert_new, horz_new) != (None, None)
    //! For OUTPUT we only care about the new line (either added or modified)
    //!
    //! Now, if there are multiple available reflections, one must take priority.
    //! In part 1 we dealt with this (but didn't need to) in find_reflections_*
    //! Now we must deal with it here.
    //! Actually, it's not so much that one must take priority, it's that we need the new one
    //! TODO: we still might need to filter for priority if there are two possible new ones

    let rows: Vec<Vec<char>> = pat
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let width = rows.iter().map(|r| r.len()).min().unwrap_or(0);
    let height = rows.len();

    for h in 0..height {
        for w in 0..width {
            let mut testy = rows.clone();
            testy[h][w] = match testy[h][w] {
                '#' => '.',
                '.' => '#',
                _ => unreachable!(),
            };
            let test_pat: String = testy
                .into_iter()
                .map(|v| v.iter().collect::<String>())
                .map(|mut x| {
                    x.push('\n');
                    x
                })
                .collect();

            // Apply priority
            let vert_new = find_reflections_col(&test_pat)
                .iter()
                .map(|x| x + 1)
                .find(|&x| Some(x) != vert_orig);
            let horz_new = find_reflections_row(&test_pat)
                .iter()
                .map(|x| x + 1)
                .find(|&x| Some(x) != horz_orig);
            // println!(
            // "({w:02}, {h:02})\tV: {vert_orig:?} -> {vert_new:?}\tH: {horz_orig:?} -> {horz_new:?}"
            // );

            // if (vert_new, horz_new) != (vert_orig, horz_orig) &&
            if (vert_new, horz_new) != (None, None) {
                // println!("Old pattern:\n{pat}");
                // println!("h {:?} v {:?}", horz_orig, vert_orig);
                // println!(
                //     "\nNew pattern (changed ({w}, {h})):\n{}",
                //     test_pat.trim_end()
                // );
                // println!("h {:?} v {:?}\n", horz_new, vert_new);
                let mut out = 0;
                if vert_new != vert_orig {
                    out += vert_new.unwrap_or(0);
                }
                if horz_new != horz_orig {
                    out += 100 * horz_new.unwrap_or(0);
                }
                return out;
            }
        }
    }
    panic!("no new reflection found for \n{pat}");
}

fn rowdiff(pat: &str) {
    println!("linediff:");
    for (i, (a, b)) in pat.lines().zip(pat.lines().skip(1)).enumerate() {
        let count = a.chars().zip(b.chars()).filter(|(x, y)| x != y).count();
        if count == 0 {
            println!("\n{i}\t{a}\n{}\t{b}\n\n", i + 1);
        } else {
            print!("{i}: {count}\t");
        }
    }
    println!();
}

fn coldiff(pat: &str) {
    println!("coldiff");

    let rows: Vec<Vec<char>> = pat
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let width = rows.iter().map(|r| r.len()).min().unwrap_or(0);

    for i in 0..(width - 1) {
        let a: String = rows.iter().map(|r| r[i]).collect();
        let b: String = rows.iter().map(|r| r[i + 1]).collect();
        let count = a.chars().zip(b.chars()).filter(|(x, y)| x != y).count();
        if count == 0 {
            println!("\n{i}\t{a}\n{}\t{b}\n\n", i + 1);
        } else {
            print!("{i}: {count}\t");
        }
    }
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Vertical reflection between cols 5 & 6 (1-indexed)
    const EXAMPLE_1_A: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    /// Horizontal reflection between rows 4 & 5 (1-indexed)
    const EXAMPLE_1_B: &str = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_row_find() {
        assert_eq!(find_reflections_row(EXAMPLE_1_B)[0] + 1, 4);
        assert!(find_reflections_row(EXAMPLE_1_A).is_empty());
    }

    #[test]
    fn test_col() {
        assert_eq!(find_reflections_col(EXAMPLE_1_A)[0] + 1, 5);
        assert!(find_reflections_col(EXAMPLE_1_B).is_empty());
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(&format!("{EXAMPLE_1_A}\n\n{EXAMPLE_1_B}")), 405);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(&format!("{EXAMPLE_1_A}\n\n{EXAMPLE_1_B}")), 400);
    }
    #[test]
    fn part_1_real() {
        assert_eq!(part_1(include_str!("../input.txt")), 33975);
    }

    /// Apparently, there are no smudge changes possible on this one?
    /// What *should* happen is that there's a # at row 8 col 16 which if deleted
    /// makes a larger reflection available
    /// but that's not getting picked up?
    const HMM_1: &str = r"..##...#....####.
##.....###..#..#.
####.#.###.######
####...#.#.######
..####.#.###....#
..##.#..##...##..
###..######......
...####.###....#.
##...####.#.####.
####..####.##..##
##.#..#.#.#.####.
..........##.##.#
..#...####.#.##.#";
    #[test]
    fn dual_reflection() {
        assert_eq!(part_1(HMM_1), 1);
    }

    /// One # to .
    const HMM_1_FIX: &str = r"..##...#....####.
##.....###..#..#.
####.#.###.######
####...#.#.######
..####.#.###....#
..##.#..##...##..
###..######......
...####.###......
##...####.#.####.
####..####.##..##
##.#..#.#.#.####.
..........##.##.#
..#...####.#.##.#";
    #[test]
    fn hmm1fix() {
        assert_eq!(find_reflections_col(HMM_1_FIX), vec![0, 13]);
    }

    const HMM_2: &str = r"#.#.#.##.
...##...#
...##...#
#.#.#.#..
#..######
.#.##..##
.###.#.##
#...####.
#..##....
....#.#.#
####.#.#.
###...#..
.#.##...#
##.####..
##.####..
.#.##...#
###...#..";

    #[test]
    fn wtf2() {
        println!("v: {:?}", find_reflections_col(HMM_2));
        println!("h: {:?}", find_reflections_row(HMM_2));
        coldiff(HMM_2);
        rowdiff(HMM_2);
        assert_eq!(exh_unsmudge(HMM_2, None, Some(14)), 200);
    }
}

// Pt 2: 28336 is too low for my input
