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
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str) -> usize {
    // functionally equivalent to segmenting within each column
    // then summing (row, row-1, ...) for as many O as within that segment
    let rows: Vec<Vec<char>> = infile
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let width = rows.iter().map(|r| r.len()).min().unwrap_or(0);
    let height = rows.len();

    let mut total = 0;

    for c in 0..width {
        let mut score = height;
        for r in 0..height {
            match rows[r][c] {
                '#' => score = (height - r).saturating_sub(1),
                'O' => {
                    total += score;
                    score -= 1;
                }
                _ => {}
            }
            // print!("{} ({r}, {c}): {total}\t", rows[r][c]);
        }
        // println!("");
    }
    total
}

/// Now we have to do it properly.
/// We do have a trick though: this (hopefully) settles into being cyclic.
/// Suppose we had to do 25 cycles with a lead in of 3 and a period of 8.
///     ...XxxxxxxxXxxxxxxxXxxxxx
///             ^---- equiv-----^   
fn part_2(infile: &str) -> usize {
    // {grid : (cycle, score)}
    let mut cache: HashMap<String, (usize, usize)> = HashMap::new();

    let mut grid = string_to_grid(infile);

    for cycles in 1..=1_000_000_000 {
        grid = tilt_north(grid);
        grid = tilt_west(grid);
        grid = tilt_south(grid);
        grid = tilt_east(grid);
        let _nscore = score_grid2(&tilt_north(grid.clone()));
        let score = score_grid2(&grid);
        // println!(
        //     "After {cycles} cycles, score {score}\n{}\n",
        //     grid_to_string(&grid)
        // );
        if let Some((pcycle, _pscore)) = cache.get(&grid_to_string(&grid)) {
            let period = cycles - pcycle;
            let lead_in = pcycle;
            println!("period of {period}; lead-in of {lead_in}");
            return cache
                .values()
                .filter(|(c, _)| *c == lead_in + ((1_000_000_000 - cycles) % period))
                .map(|(_, s)| *s)
                .next()
                .unwrap();
        }
        cache.insert(grid_to_string(&grid), (cycles, score));
    }
    score_grid(&grid)
}

fn string_to_grid(infile: &str) -> Vec<Vec<char>> {
    infile
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn grid_to_string(grid: &Vec<Vec<char>>) -> String {
    let mut out = String::new();
    for row in grid {
        for cha in row {
            out.push(*cha);
        }
        out.push('\n')
    }
    out.pop();
    out
}

/// NOTE: the "total load on the north support beams" might need to
fn score_grid(grid: &Vec<Vec<char>>) -> usize {
    let width = grid.iter().map(|r| r.len()).min().unwrap_or(0);
    let height = grid.len();

    let mut total = 0;

    for c in 0..width {
        let mut score = height;
        for r in 0..height {
            match grid[r][c] {
                '#' => score = (height - r).saturating_sub(1),
                'O' => {
                    total += score;
                    score -= 1;
                }
                _ => {}
            }
        }
    }
    total
}

fn score_grid2(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let mut total = 0;
    for (r, row) in grid.iter().enumerate() {
        total += (height - r) * row.iter().filter(|&&c| c == 'O').count();
    }
    total
}

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = grid.iter().map(|r| r.len()).min().unwrap_or(0);
    let height = grid.len();

    /*
    if height == 0 || width == 0 {
        panic!(
            "Something is wrong with this grid:\n{}",
            grid_to_string(&grid)
        );
    }
    */

    let mut out = vec![vec!['~'; height]; width];

    for (row, line) in grid.iter().enumerate() {
        for (col, cha) in line.iter().enumerate() {
            out[col][row] = *cha;
        }
    }
    out
}

fn mirror_ew(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.into_iter()
        .map(move |mut s| {
            s.reverse();
            s
        })
        .collect()
}

fn rotate_east(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    mirror_ew(transpose(grid))
}
fn rotate_west(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(mirror_ew(grid))
}

/// Segment by # within rows, sort segments ('.' < 'O'), re-collect
fn tilt_east(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut out = vec![];
    for row in grid {
        let mut lasthash = 0;
        let mut new = vec![];
        let mut tmp = vec![];
        for (i, &c) in row.iter().enumerate() {
            if c == '#' {
                if lasthash <= i {
                    tmp.sort();
                    new.extend_from_slice(&tmp);
                    tmp.clear();
                }
                new.push(c);
                lasthash = i;
            } else {
                tmp.push(c);
            }
        }
        tmp.sort();
        new.extend_from_slice(&tmp);
        out.push(new);
    }
    out
}

fn tilt_west(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut out = vec![];
    for row in grid {
        let mut lasthash = 0;
        let mut new = vec![];
        let mut tmp = vec![];
        for (i, &c) in row.iter().enumerate() {
            if c == '#' {
                if lasthash <= i {
                    tmp.sort();
                    tmp.reverse();
                    new.extend_from_slice(&tmp);
                    tmp.clear();
                }
                new.push('#');
                lasthash = i;
            } else {
                tmp.push(c);
            }
        }
        tmp.sort();
        tmp.reverse();
        new.extend_from_slice(&tmp);
        out.push(new);
    }
    out
}

fn tilt_north(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(tilt_west(transpose(grid)))
}
fn tilt_south(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(tilt_east(transpose(grid)))
}

#[cfg(test)]
mod test {
    use super::*;

    /// O can roll in any of the four cardinal directions, # are fixed, . are empty
    const EXAMPLE_1: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 136);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 64);
    }

    #[test]
    fn transforms() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];

        assert_eq!(input.clone(), transpose(transpose(input.clone())));
        assert_eq!(input.clone(), mirror_ew(mirror_ew(input.clone())));
    }

    #[test]
    fn deser() {
        assert_eq!(EXAMPLE_1, &grid_to_string(&string_to_grid(EXAMPLE_1)));
    }

    #[test]
    fn score() {
        let n_tilt = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        assert_eq!(score_grid(&string_to_grid(n_tilt)), 136);
    }
    #[test]
    fn score_2() {
        let n_tilt = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        assert_eq!(score_grid2(&string_to_grid(n_tilt)), 136);
    }

    #[test]
    fn what_do() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];

        println!("I\n{}", grid_to_string(&input.clone()));
        println!("M\n{}", grid_to_string(&mirror_ew(input.clone())));
        println!("T\n{}", grid_to_string(&transpose(input.clone())));

        println!(
            "MT = rotate east\n{}",
            grid_to_string(&mirror_ew(transpose(input.clone())))
        );

        println!(
            "TMT = flip NS\n{}",
            grid_to_string(&transpose(mirror_ew(transpose(input.clone()))))
        );

        println!(
            "MTMT = rotate south \n{}",
            grid_to_string(&mirror_ew(transpose(mirror_ew(transpose(input.clone())))))
        );
        // intermediate stage
        println!(
            "TMTMT\n{}",
            grid_to_string(&transpose(mirror_ew(transpose(mirror_ew(transpose(
                input.clone()
            ))))))
        );
        println!(
            "MTMTMT = rotate west\n{}",
            grid_to_string(&mirror_ew(transpose(mirror_ew(transpose(mirror_ew(
                transpose(input.clone())
            ))))))
        );
        println!(
            "MTMTMTMT = I\n{}",
            grid_to_string(&mirror_ew(transpose(mirror_ew(transpose(mirror_ew(
                transpose(mirror_ew(transpose(input.clone())))
            ))))))
        );
        println!(
            "TM = rotate west?\n{}",
            grid_to_string(&transpose(mirror_ew(input.clone())))
        );
    }

    #[test]
    fn one_cycle() {
        let mut grid = string_to_grid(EXAMPLE_1);
        println!("{EXAMPLE_1}");

        grid = tilt_north(grid);
        println!("-> N\n{}", grid_to_string(&grid));

        grid = tilt_west(grid);
        println!("-> W\n{}", grid_to_string(&grid));

        grid = tilt_south(grid);
        println!("-> S\n{}", grid_to_string(&grid));

        grid = tilt_east(grid);
        println!("-> E\n{}", grid_to_string(&grid));

        let one = r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

        assert_eq!(grid_to_string(&grid), one);
    }

    #[test]
    fn three_cycle() {
        let three = r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

        let mut grid = string_to_grid(EXAMPLE_1);
        for _ in 0..3 {
            grid = tilt_north(grid);
            grid = tilt_west(grid);
            grid = tilt_south(grid);
            grid = tilt_east(grid);
        }

        assert_eq!(grid_to_string(&grid), three);
    }
}
