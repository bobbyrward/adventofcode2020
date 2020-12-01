use anyhow::{anyhow, Result};
use clap::Clap;
use im::Vector;
use tracing::debug;

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

fn find_paired_expense(current: i64, expenses: &Vector<i64>) -> Option<i64> {
    if expenses.is_empty() {
        return None;
    }

    let first = expenses.front().unwrap();
    if current + first > 2020 {
        debug!("{} + first({}) > 2020", current, first);
        return None;
    }

    let last = expenses.back().unwrap();
    if current + last < 2020 {
        debug!("{} + last({}) < 2020", current, last);
        return None;
    }

    expenses
        .binary_search_by(|n| (current + n).cmp(&2020))
        .ok()
        .map(|n| expenses[n])
}

fn find_matching_expenses(expenses: &[i64]) -> Option<(i64, i64)> {
    let mut known = Vector::new();

    known.push_back(expenses[0]);

    for expense in expenses.iter().skip(1) {
        if let Some(found_match) = find_paired_expense(*expense, &known) {
            return Some((*expense, found_match));
        }

        known.insert_ord(*expense);
    }

    None
}

fn part_one() -> Result<String> {
    let found = find_matching_expenses(
        &input("day01")?
            .lines()
            .map(|s| s.trim().parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?,
    )
    .ok_or_else(|| anyhow!("No matches found"))?;

    Ok(format!("{:?}", found.0 * found.1))
}

fn part_two() -> Result<String> {
    Ok(String::from("Solution 2"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_find_paired() -> Result<()> {
        let expenses = vec![299, 366, 675, 979, 1456, 1721];

        let test_match = |n: i64, expected: Option<i64>| {
            let filtered = &expenses
                .iter()
                .copied()
                .filter(|ln| *ln != n)
                .collect::<Vec<_>>();

            assert_eq!(find_paired_expense(n, &Vector::from(filtered)), expected);
        };

        test_match(1721, Some(299));
        test_match(299, Some(1721));

        test_match(366, None);
        test_match(675, None);
        test_match(979, None);
        test_match(1456, None);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_find_matching() -> Result<()> {
        let expenses = vec![299, 366, 675, 979, 1456, 1721];
        assert_eq!(find_matching_expenses(&expenses), Some((1721, 299)));

        let expenses = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_matching_expenses(&expenses), Some((299, 1721)));

        let expenses = vec![299, 979, 366, 299, 675, 1456, 1721];
        assert_eq!(find_matching_expenses(&expenses), Some((1721, 299)));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = input("day01")?
            .lines()
            .map(|s| s.trim().parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        // assert_eq(find_matching_expenses(&input)

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        Ok(())
    }
}
