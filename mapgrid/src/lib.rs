extern crate derive_more;
use derive_more::{Add, Sub, Display, From, Into}
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ops::Add,
};
use std::cmp::{PartialOrd, PartialEq, Ord, Eq};

// #[derive(From, Into, PartialOrd, PartialEq, Ord, Eq)]
pub type Coord = [isize; 2];

pub trait Grid<V> {
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> Self;
}

impl<V> Grid<V> for BTreeMap<Coord, V> {
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> BTreeMap<Coord, V> {
        let mut out = BTreeMap::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if let Some(v) = f(cha) {
                    out.insert([row as isize, col as isize], v);
                }
            }
        }
        out
    }
}

impl<V> Grid<V> for BTreeSet<Coord> {
    /// Note: the value of V is disregarded
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> BTreeSet<Coord> {
        let mut out = BTreeSet::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if f(cha).is_some() {
                    out.insert([row as isize, col as isize]);
                }
            }
        }
        out
    }
}

impl<V> Grid<V> for HashMap<Coord, V> {
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> HashMap<Coord, V> {
        let mut out = HashMap::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if let Some(v) = f(cha) {
                    out.insert([row as isize, col as isize], v);
                }
            }
        }
        out
    }
}

impl<V> Grid<V> for HashSet<Coord> {
    /// Note: the value of V is disregarded
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> HashSet<Coord> {
        let mut out = HashSet::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if f(cha).is_some() {
                    out.insert([row as isize, col as isize]);
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
}
