use anyhow::{Context, Result};
use clap::Parser;
use itertools::Itertools;

use std::fs::read_to_string;
use std::ops::{Add, AddAssign, Range, RangeBounds, Sub, SubAssign};

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile)?);
    println!("Part 2:\n{}", part_2(&infile)?);

    Ok(())
}

fn part_1(infile: &str) -> Result<usize> {
    let config = config_scraper(infile)?;
    // println!("{config:?}");

    let minimum = config
        .seeds
        .iter()
        .map(|x| {
            config
                .seed_soil
                .iter()
                .find_map(|y| y.get(*x))
                .unwrap_or(*x)
        })
        .map(|x| {
            config
                .soil_fertilizer
                .iter()
                .find_map(|y| y.get(x))
                .unwrap_or(x)
        })
        .map(|x| {
            config
                .fertilizer_water
                .iter()
                .find_map(|y| y.get(x))
                .unwrap_or(x)
        })
        .map(|x| {
            config
                .water_light
                .iter()
                .find_map(|y| y.get(x))
                .unwrap_or(x)
        })
        .map(|x| {
            config
                .light_temperature
                .iter()
                .find_map(|y| y.get(x))
                .unwrap_or(x)
        })
        .map(|x| {
            config
                .temperature_humidity
                .iter()
                .find_map(|y| y.get(x))
                .unwrap_or(x)
        })
        .map(|x| {
            config
                .humidity_location
                .iter()
                .find_map(|y| y.get(x))
                .unwrap_or(x)
        })
        .min()
        .context(":shrug:")?;

    Ok(minimum)
}
fn part_2(infile: &str) -> Result<usize> {
    // sike, we actually have an infeasible-to-brute force number of seeds to consider
    // config.seeds is in (initial, range) pairs now

    // A Lookup is really saying that a certain input range has a certain (non-zero) offset
    // Otherwise the offset is zero

    // at each stage we take a set of ranges, apply some Lookups and get another set of ranges
    // the trick is that applying a Lookup might split an input range
    // also multiple Lookups might intersect with the same input range
    // we trust the input is well formed enough that two Lookups won't apply to the same value
    // for each range, for each Lookup, apply Lookup, get up to three output ranges per apply, probably don't need to coalesce

    // the before and after ranges are eligible for further lookup, the overlap range isn't

    let config = config_scraper(infile)?;
    // println!("{config:?}");

    let mut ranges: Vec<Range<usize>> = config
        .seeds
        .iter()
        .tuples()
        .map(|(base, len)| (*base)..(*base + *len))
        .collect();

    println!("seeds: {ranges:?}");
    ranges = multi_range(&config.seed_soil, &ranges);
    println!("soils: {ranges:?}");

    ranges = multi_range(&config.soil_fertilizer, &ranges);
    println!("fertilizers: {ranges:?}");

    ranges = multi_range(&config.fertilizer_water, &ranges);
    println!("waters: {ranges:?}");

    ranges = multi_range(&config.water_light, &ranges);
    println!("lights: {ranges:?}");

    ranges = multi_range(&config.light_temperature, &ranges);
    println!("temperatures: {ranges:?}");

    ranges = multi_range(&config.temperature_humidity, &ranges);
    println!("humidities: {ranges:?}");

    ranges = multi_range(&config.humidity_location, &ranges);
    println!("locations: {ranges:?}");

    Ok(ranges.iter().map(|r| r.start).min().unwrap())

    // 93839242 is too high for my input
}

#[derive(Debug, Default, PartialOrd, PartialEq, Copy, Clone)]
struct Lookup<T>
where
    T: AddAssign
        + SubAssign
        + Ord
        + Eq
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + std::default::Default,
{
    source: T,
    dest: T,
    length: T,
}

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
struct Config<T>
where
    T: AddAssign + SubAssign + Ord + Eq + Copy + Sub<Output = T> + Add<Output = T> + Default,
{
    seeds: Vec<usize>,
    seed_soil: Vec<Lookup<T>>,
    soil_fertilizer: Vec<Lookup<T>>,
    fertilizer_water: Vec<Lookup<T>>,
    water_light: Vec<Lookup<T>>,
    light_temperature: Vec<Lookup<T>>,
    temperature_humidity: Vec<Lookup<T>>,
    humidity_location: Vec<Lookup<T>>,
}

impl<T> Lookup<T>
where
    T: Default + AddAssign + SubAssign + Ord + Eq + Copy + Sub<Output = T> + Add<Output = T>,
{
    fn get(&self, input: T) -> Option<T> {
        if input >= self.source && (input - self.source <= self.length) {
            let offset = input - self.source;
            return Some(self.dest + offset);
        }
        None
    }

    fn get_range(
        &self,
        input: &std::ops::Range<T>,
    ) -> (
        Option<std::ops::Range<T>>,
        Option<std::ops::Range<T>>,
        Option<std::ops::Range<T>>,
    ) {
        let source_range = self.source..(self.source + self.length);

        let (before, overlap, after) = range_partition::<T>(input, &source_range);

        if let Some(o) = overlap {
            // this is the part which gets offset
            // we're working with usizes here which means no negative offsets

            let tx = if self.source > self.dest {
                let offset = self.source - self.dest;
                (o.start - offset)..(o.end - offset)
            } else {
                let offset = self.dest - self.source;
                (o.start + offset)..(o.end + offset)
            };

            return (before, Some(tx), after);
        }

        (before, overlap, after)
    }
}
/// partition `base` by `part`
/// (before, overlap, after)
/// `.0`: the section of `base` < `part`
/// `.1`: the overlap
/// `.2`: the section of `base` > `part`
fn range_partition<T>(
    base: &Range<T>,
    part: &Range<T>,
) -> (Option<Range<T>>, Option<Range<T>>, Option<Range<T>>)
where
    T: Default + AddAssign + SubAssign + Ord + Eq + Copy + Sub<Output = T> + Add<Output = T>,
{
    let mut before = None;
    let mut during = None;
    let mut after = None;

    if base.start < part.start {
        // we have a before
        before = Some(base.start..base.end.min(part.start));
        if part.start < base.end && base.end <= part.end {
            // we also have a during
            during = Some(part.start..base.end)
        }
        if part.end < base.end {
            // we have a before, during and after
            during = Some(part.clone());
            after = Some(part.end..base.end);
        }
    } else if part.contains(&base.start) {
        // we have an no before, but we do have a during
        during = Some(base.start..base.end.min(part.end));
        if part.end < base.end {
            // we also have an after
            after = Some(part.end..base.end);
        }
    } else {
        // we only have an after
        after = Some(base.clone());
    }

    (before, during, after)
}

fn multi_range<T>(lookups: &[Lookup<T>], ranges: &[Range<T>]) -> Vec<Range<T>>
where
    T: Default
        + AddAssign
        + SubAssign
        + Ord
        + Eq
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + std::fmt::Debug,
{
    let mut out = vec![];
    let mut todo = vec![];
    for r in ranges {
        todo.push(r.clone());
    }

    for lookup in lookups {
        let mut new = vec![];
        for r in &todo {
            let (b, o, a) = lookup.get_range(r);
            // println!("{lookup:?} of {r:?} -> ({b:?} ; {o:?} ; {a:?})");
            if let Some(before) = b {
                new.push(before);
            }
            if let Some(overlap) = o {
                out.push(overlap);
                // must be converted at most once
            }
            if let Some(after) = a {
                new.push(after);
            }
        }
        // println!("after {lookup:?}, converted {out:?}, still todo {new:?}");
        todo = new;
    }

    out.extend_from_slice(&todo);
    out
}

fn config_scraper(infile: &str) -> Result<Config<usize>> {
    // we can split by colon, then by whitespace
    // first is a special case
    // remaining we chunk numbers by 3

    let mut config = Config::default();

    let (_, seeds, soil, fertilizer, water, light, temperature, humidity, location) = infile
        .splitn(9, ':')
        .collect_tuple()
        .context("too few sections")?;

    config.seeds = seeds
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    config.seed_soil = soil
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .tuples()
        .map(|(dest, source, length)| Lookup {
            source,
            dest,
            length,
        })
        .collect();

    config.soil_fertilizer = fertilizer
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .tuples()
        .map(|(dest, source, length)| Lookup {
            source,
            dest,
            length,
        })
        .collect();
    config.fertilizer_water = water
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .tuples()
        .map(|(dest, source, length)| Lookup {
            source,
            dest,
            length,
        })
        .collect();
    config.water_light = light
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .tuples()
        .map(|(dest, source, length)| Lookup {
            source,
            dest,
            length,
        })
        .collect();
    config.light_temperature = temperature
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .tuples()
        .map(|(dest, source, length)| Lookup {
            source,
            dest,
            length,
        })
        .collect();
    config.temperature_humidity = humidity
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .tuples()
        .map(|(dest, source, length)| Lookup {
            source,
            dest,
            length,
        })
        .collect();
    config.humidity_location = location
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .tuples()
        .map(|(dest, source, length)| Lookup {
            source,
            dest,
            length,
        })
        .collect();
    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1).unwrap(), 35);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1).unwrap(), 46);
    }

    #[test]
    fn test_range_partition() {
        let a = 0..5;
        let b = 4..10;
        let c = 6..8;

        assert_eq!(range_partition(&a, &b), (Some(0..4), Some(4..5), None));
        assert_eq!(range_partition(&a, &c), (Some(0..5), None, None));
        assert_eq!(range_partition(&c, &b), (None, Some(6..8), None));
        assert_eq!(range_partition(&c, &a), (None, None, Some(6..8)));
        assert_eq!(
            range_partition(&b, &c),
            (Some(4..6), Some(6..8), Some(8..10))
        );
        assert_eq!(range_partition(&b, &a), (None, Some(4..5), Some(5..10)))
    }
}
