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

    let input = parse_input_1(&infile);

    println!("{input:?}");

    println!("Part 1:\n{}", part_1(&infile)?);
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str) -> Result<usize> {
    let input = parse_input_1(infile);

    Ok(input
        .iter()
        .map(|r| {
            strategise(r.time)
                .iter()
                .filter(|d| **d > r.distance)
                .count()
        })
        .product())
}
fn part_2(infile: &str) -> u64 {
    // hard coding values because why not

    // basic calculus says that (A - t) * t has a derivative
    // A - 2t, which has a zero at t = A/2
    // we have an odd number but that'll do for a search space

    // We need -t**2 + At - D = 0

    let (time, dist) = parse_input_2(infile);

    let quadratic_p = (-time + i_sqrt(time.pow(2) - (4 * dist))) / (-2);
    let quadratic_m = (-time - i_sqrt(time.pow(2) - (4 * dist))) / (-2);

    println!("This will be very close to the actual number, but may not be it exactly. I ended up just using `bc`:");

    println!("\nbc -e '((-{time} - sqrt( ({time}^2) - (4 * {dist}))) / -2) - ((-{time} + sqrt( ({time}^2) - (4 * {dist}))) / -2) + 1'\n");

    quadratic_m.abs_diff(quadratic_p) - 1
}

fn i_sqrt(number: i64) -> i64 {
    (number as f64).sqrt() as i64 + 1
}

fn parse_input_1(infile: &str) -> Vec<Race> {
    // two lines, columns are associative
    let lines: Vec<&str> = infile.lines().collect();

    let times: Vec<usize> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let dists: Vec<usize> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    times
        .iter()
        .zip(dists.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect()
}
fn parse_input_2(infile: &str) -> (i64, i64) {
    let lines: Vec<&str> = infile.lines().collect();

    let time: i64 = lines[0]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    let dist: i64 = lines[1]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    (time, dist)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Race {
    time: usize,
    distance: usize,
}

/// For each whole ms you spend holding the button,
/// the boat's speed increases by 1 ms/s
/// but you only have `total_time` to hold and go
/// returns a vec of distances where the index is the # of ms spent holding
fn strategise(total_time: usize) -> Vec<usize> {
    (0..=total_time).map(|t| (total_time - t) * t).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1).unwrap(), 288);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 71503);
    }
}

/*





*/
