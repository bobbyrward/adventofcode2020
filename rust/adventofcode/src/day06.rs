use anyhow::Result;
use clap::Clap;
use std::collections::HashSet;

use crate::{input, Command};

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

fn part_one() -> Result<String> {
    let count = input(crate::Day::day06)
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(str::chars)
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>();

    Ok(count.to_string())
}

fn part_two() -> Result<String> {
    let count = input(crate::Day::day06)
        .split("\n\n")
        .map(|group| {
            group
                .trim_end()
                .split('\n')
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(None, |state: Option<HashSet<_>>, line| {
                    if let Some(state) = state {
                        Some(
                            state
                                .intersection(&line)
                                .copied()
                                .collect::<HashSet<char>>(),
                        )
                    } else {
                        Some(line)
                    }
                })
                .map(|s| s.len())
                .unwrap_or(0)
        })
        .sum::<usize>();

    Ok(count.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        Ok(())
    }
}
