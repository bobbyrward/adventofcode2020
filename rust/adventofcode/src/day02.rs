use anyhow::{anyhow, Result};
use clap::Clap;
use lazy_static::lazy_static;
use regex::Regex;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Policy {
    min: usize,
    max: usize,
    what: u8,
}

impl Policy {
    fn new(min: usize, max: usize, what: u8) -> Self {
        Self { min, max, what }
    }

    fn is_valid_by_count(&self, password: &str) -> bool {
        let what_count = password.bytes().filter(|b| *b == self.what).count();

        what_count >= self.min && what_count <= self.max
    }

    fn is_valid_by_position(&self, password: &str) -> bool {
        let bytes = password.as_bytes();

        (bytes[self.min - 1] == self.what) ^ (bytes[self.max - 1] == self.what)
    }
}

fn parse_line(line: &str) -> Result<(Policy, &str)> {
    lazy_static! {
        static ref PARSE_RE: Regex =
            Regex::new(r"^(\d+)-(\d+) ([[:lower:]]): ([[:lower:]]+)$").unwrap();
    }

    let found = PARSE_RE
        .captures(line)
        .ok_or_else(|| anyhow!("malformed line: {}", line))?;

    Ok((
        Policy::new(
            found.get(1).unwrap().as_str().parse::<usize>()?,
            found.get(2).unwrap().as_str().parse::<usize>()?,
            found
                .get(3)
                .unwrap()
                .as_str()
                .bytes()
                .next()
                .ok_or_else(|| anyhow!("missing what???"))?,
        ),
        found.get(4).unwrap().as_str(),
    ))
}

fn part_one() -> Result<String> {
    let count = input("day02")?
        .lines()
        .map(|s| parse_line(s))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|(policy, password)| policy.is_valid_by_count(password))
        .count();

    Ok(count.to_string())
}

fn part_two() -> Result<String> {
    let count = input("day02")?
        .lines()
        .map(|s| parse_line(s))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|(policy, password)| policy.is_valid_by_position(password))
        .count();

    Ok(count.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        /*
        For example, suppose you have the following list:

        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc

        Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

        In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
        */

        let p1 = parse_line("1-3 a: abcde")?;
        let p2 = parse_line("1-3 b: cdefg")?;
        let p3 = parse_line("2-9 c: ccccccccc")?;

        assert_eq!(p1, (Policy::new(1, 3, b'a'), "abcde"));
        assert_eq!(p2, (Policy::new(1, 3, b'b'), "cdefg"));
        assert_eq!(p3, (Policy::new(2, 9, b'c'), "ccccccccc"));

        assert!(p1.0.is_valid_by_count(p1.1));
        assert!(!p2.0.is_valid_by_count(p2.1));
        assert!(p3.0.is_valid_by_count(p3.1));

        let count = input("day02")?
            .lines()
            .map(|s| parse_line(s))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .filter(|(policy, password)| policy.is_valid_by_count(password))
            .count();

        assert_eq!(count, 483);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        /*
        Given the same example list from above:

            1-3 a: abcde is valid: position 1 contains a and position 3 does not.
            1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
            2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
        */
        let p1 = parse_line("1-3 a: abcde")?;
        let p2 = parse_line("1-3 b: cdefg")?;
        let p3 = parse_line("2-9 c: ccccccccc")?;

        assert_eq!(p1, (Policy::new(1, 3, b'a'), "abcde"));
        assert_eq!(p2, (Policy::new(1, 3, b'b'), "cdefg"));
        assert_eq!(p3, (Policy::new(2, 9, b'c'), "ccccccccc"));

        assert!(p1.0.is_valid_by_position(p1.1));
        assert!(!p2.0.is_valid_by_position(p2.1));
        assert!(!p3.0.is_valid_by_position(p3.1));

        let count = input("day02")?
            .lines()
            .map(|s| parse_line(s))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .filter(|(policy, password)| policy.is_valid_by_position(password))
            .count();

        assert_eq!(count, 482);

        Ok(())
    }
}
