//! Day 3: Toboggan Trajectory
//!
//! Since they haven't gotten complicated yet, let's make it artifically so.
//!
use std::convert::TryFrom;
use std::ops::Index;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Clap;

use crate::Command;
use crate::Point;

#[derive(Debug, Clap)]
pub enum Args {
    Part1,
    Part2,
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

/// A trait for doing wrap around indexing (of positive indexes)
trait IndexLoop<T> {
    fn index_loop(&self, index: usize) -> &T;
}

/// Implement it on Vec so Vec now has an index_loop method
impl<T> IndexLoop<T> for Vec<T> {
    fn index_loop(&self, index: usize) -> &T {
        &self[index % self.len()]
    }
}

/// Type alias for Point<usize> to save typing
type MapIndex = Point<usize>;

/// Instead of requiring a MapIndex in parameters, we can instead
/// require a type that implements IntoMapIndex yet still just work
/// with MapIndex's
trait IntoMapIndex {
    fn into_map_index(self) -> MapIndex;
}

// Auto convert appropriate tuples
impl IntoMapIndex for (usize, usize) {
    fn into_map_index(self) -> MapIndex {
        MapIndex::new(self.0, self.1)
    }
}

// Identity impl
impl IntoMapIndex for MapIndex {
    fn into_map_index(self) -> MapIndex {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapEntry {
    Empty,
    Tree,
}

impl MapEntry {
    fn is_tree(&self) -> bool {
        matches!(self, MapEntry::Tree)
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        matches!(self, MapEntry::Empty)
    }
}

impl Default for MapEntry {
    fn default() -> Self {
        Self::Empty
    }
}

impl TryFrom<u8> for MapEntry {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'#' => Self::Tree,
            b'.' => Self::Empty,
            _ => return Err(anyhow!("Invalid MapEntry: {}", value)),
        })
    }
}

#[derive(Debug, Default)]
struct TobogganMap {
    map: Vec<Vec<MapEntry>>,
}

impl TobogganMap {
    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn count_trees_in_path<I>(&self, path: I) -> usize
    where
        I: IntoMapIndex,
    {
        let path = path.into_map_index();

        // There's this monstrosity
        (1..)
            .map(|i| path * i)
            .take_while(|idx| idx.y < self.len())
            .filter(|idx| self[*idx].is_tree())
            .count()

        /* Or this
        let mut tree_count = 0;
        let mut index = MapIndex::new(0, 0);

        loop {
            index += MapIndex::new(3, 1);

            if index.y >= map.len() {
                break;
            }

            if map[index].is_tree() {
                tree_count += 1;
            }
        }

        tree_count
        */
    }
}

impl FromStr for TobogganMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .copied()
                    .map(MapEntry::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|map| Self { map })
    }
}

/// Implement the [] operator using MapIndex
impl<T> Index<T> for TobogganMap
where
    T: IntoMapIndex,
{
    type Output = MapEntry;

    fn index(&self, index: T) -> &Self::Output {
        let index = index.into_map_index();
        self.map[index.y].index_loop(index.x)
    }
}

fn part_one() -> Result<String> {
    Ok(crate::input(crate::Day::day03)
        .parse::<TobogganMap>()?
        .count_trees_in_path((3, 1))
        .to_string())
}

fn part_two() -> Result<String> {
    let map: TobogganMap = crate::input(crate::Day::day03).parse()?;
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    Ok(slopes
        .iter()
        .map(|idx| map.count_trees_in_path(*idx))
        .product::<usize>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "..##.......\n\
                         #...#...#..\n\
                         .#....#..#.\n\
                         ..#.#...#.#\n\
                         .#...##..#.\n\
                         ..#.##.....\n\
                         .#.#.#....#\n\
                         .#........#\n\
                         #.##...#...\n\
                         #...##....#\n\
                         .#..#...#.#";

    #[tracing_test::traced_test]
    #[test]

    fn test_part_one() -> Result<()> {
        let map: TobogganMap = INPUT.parse()?;

        assert_eq!(map[(0, 0)], MapEntry::Empty);
        assert_eq!(map[(1, 0)], MapEntry::Empty);
        assert_eq!(map[(2, 0)], MapEntry::Tree);
        assert_eq!(map[(3, 0)], MapEntry::Tree);

        assert_eq!(map[(11, 0)], MapEntry::Empty);
        assert_eq!(map[(12, 0)], MapEntry::Empty);
        assert_eq!(map[(13, 0)], MapEntry::Tree);
        assert_eq!(map[(14, 0)], MapEntry::Tree);

        assert_eq!(map.count_trees_in_path((3, 1)), 7);

        let map: TobogganMap = crate::input(crate::Day::day03).parse()?;
        assert_eq!(map.count_trees_in_path((3, 1)), 286);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        /*
        Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

        Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

            Right 1, down 1.
            Right 3, down 1. (This is the slope you already checked.)
            Right 5, down 1.
            Right 7, down 1.
            Right 1, down 2.

        In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.
        */
        let map: TobogganMap = INPUT.parse()?;
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let product = slopes
            .iter()
            .map(|idx| map.count_trees_in_path(*idx))
            .product::<usize>();

        assert_eq!(product, 336);

        let map: TobogganMap = crate::input(crate::Day::day03).parse()?;

        let product = slopes
            .iter()
            .map(|idx| map.count_trees_in_path(*idx))
            .product::<usize>();

        assert_eq!(product, 3638606400);

        Ok(())
    }
}
