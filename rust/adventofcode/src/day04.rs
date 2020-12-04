use anyhow::Result;
use clap::Clap;
use std::collections::HashSet;

use crate::Command;

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
    let mut current: Vec<&str> = Vec::new();
    let mut count = 0;

    fn check_passport(passport: &[&str]) -> bool {
        let tags: HashSet<&str> = passport
            .iter()
            .filter_map(|s| {
                if *s == "" {
                    return None;
                }

                let s = s.split(':').take(1).next().unwrap();
                Some(s)
            })
            .collect();

        let has = |s| tags.contains(s);

        has("byr")
            && has("iyr")
            && has("eyr")
            && has("hgt")
            && has("hcl")
            && has("ecl")
            && has("pid")
    }

    for line in crate::input(crate::Day::day04).lines() {
        if line == "" {
            if check_passport(&current) {
                count += 1;
            }
            current = Vec::new();
        }

        current.extend(line.split(' '));
    }

    if check_passport(&current) {
        count += 1;
    }

    Ok(count.to_string())
}

fn part_two() -> Result<String> {
    let mut current: Vec<&str> = Vec::new();
    let mut count = 0;

    fn check_passport(passport: &[&str]) -> bool {
        let tags: HashSet<&str> = passport
            .iter()
            .filter_map(|s| {
                if *s == "" {
                    return None;
                }

                let mut i = s.split(':');
                let k = i.next().unwrap();
                let v = i.next().unwrap();

                match k {
                    "byr" => {
                        let i = v.parse::<i64>().unwrap();

                        if i >= 1920 && i <= 2002 {
                            Some(k)
                        } else {
                            None
                        }
                    }
                    "iyr" => {
                        let i = v.parse::<i64>().unwrap();

                        if i >= 2010 && i <= 2020 {
                            Some(k)
                        } else {
                            None
                        }
                    }
                    "eyr" => {
                        let i = v.parse::<i64>().unwrap();

                        if i >= 2020 && i <= 2030 {
                            Some(k)
                        } else {
                            None
                        }
                    }
                    "hgt" => {
                        let unit = &v[v.len() - 2..];

                        match unit {
                            "cm" => {
                                let i = v[..v.len() - 2].parse::<i64>().unwrap();
                                if i >= 150 && i <= 193 {
                                    Some(k)
                                } else {
                                    None
                                }
                            }
                            "in" => {
                                let i = v[..v.len() - 2].parse::<i64>().unwrap();
                                if i >= 59 && i <= 76 {
                                    Some(k)
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    }
                    "hcl" => {
                        if &v[0..1] != "#" {
                            return None;
                        }

                        if v.len() != 7 {
                            return None;
                        }

                        if usize::from_str_radix(&v[1..], 16).is_ok() {
                            Some(k)
                        } else {
                            None
                        }
                    }
                    "ecl" => match v {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(k),
                        _ => None,
                    },
                    "pid" => {
                        if v.len() != 9 {
                            return None;
                        }

                        if v.parse::<i64>().is_ok() {
                            Some(k)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .collect();

        let has = |s| tags.contains(s);

        has("byr")
            && has("iyr")
            && has("eyr")
            && has("hgt")
            && has("hcl")
            && has("ecl")
            && has("pid")
    }

    for line in crate::input(crate::Day::day04).lines() {
        // for line in s.lines() {
        if line == "" {
            if check_passport(&current) {
                count += 1;
            }
            current = Vec::new();
        }

        current.extend(line.split(' '));
    }

    if check_passport(&current) {
        count += 1;
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let _s = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                 byr:1937 iyr:2017 cid:147 hgt:183cm\n\
\n\
                 iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                 hcl:#cfa07d byr:1929\n\
\n\
                 hcl:#ae17e1 iyr:2013\n\
                 eyr:2024\n\
                 ecl:brn pid:760753108 byr:1931\n\
                 hgt:179cm\n\
\n\
                 hcl:#cfa07d eyr:2025 pid:166559648\n\
                 iyr:2011 ecl:brn hgt:59in";

        let mut current: Vec<&str> = Vec::new();
        let mut count = 0;

        fn check_passport(passport: &[&str]) -> bool {
            let tags: HashSet<&str> = passport
                .iter()
                .filter_map(|s| {
                    if *s == "" {
                        return None;
                    }

                    let s = s.split(':').take(1).next().unwrap();
                    Some(s)
                })
                .collect();

            let has = |s| tags.contains(s);

            has("byr")
                && has("iyr")
                && has("eyr")
                && has("hgt")
                && has("hcl")
                && has("ecl")
                && has("pid")
        }

        for line in crate::input(crate::Day::day04).lines() {
            if line == "" {
                if check_passport(&current) {
                    count += 1;
                }
                current = Vec::new();
            }

            current.extend(line.split(' '));
        }

        if check_passport(&current) {
            count += 1;
        }

        assert_eq!(count, 254);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let mut current: Vec<&str> = Vec::new();
        let mut count = 0;

        fn check_passport(passport: &[&str]) -> bool {
            let tags: HashSet<&str> = passport
                .iter()
                .filter_map(|s| {
                    if *s == "" {
                        return None;
                    }

                    let mut i = s.split(':');
                    let k = i.next().unwrap();
                    let v = i.next().unwrap();

                    match k {
                        "byr" => {
                            let i = v.parse::<i64>().unwrap();

                            if i >= 1920 && i <= 2002 {
                                Some(k)
                            } else {
                                None
                            }
                        }
                        "iyr" => {
                            let i = v.parse::<i64>().unwrap();

                            if i >= 2010 && i <= 2020 {
                                Some(k)
                            } else {
                                None
                            }
                        }
                        "eyr" => {
                            let i = v.parse::<i64>().unwrap();

                            if i >= 2020 && i <= 2030 {
                                Some(k)
                            } else {
                                None
                            }
                        }
                        "hgt" => {
                            let unit = &v[v.len() - 2..];

                            match unit {
                                "cm" => {
                                    let i = v[..v.len() - 2].parse::<i64>().unwrap();
                                    if i >= 150 && i <= 193 {
                                        Some(k)
                                    } else {
                                        None
                                    }
                                }
                                "in" => {
                                    let i = v[..v.len() - 2].parse::<i64>().unwrap();
                                    if i >= 59 && i <= 76 {
                                        Some(k)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        }
                        "hcl" => {
                            if &v[0..1] != "#" {
                                return None;
                            }

                            if v.len() != 7 {
                                return None;
                            }

                            if usize::from_str_radix(&v[1..], 16).is_ok() {
                                Some(k)
                            } else {
                                None
                            }
                        }
                        "ecl" => match v {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(k),
                            _ => None,
                        },
                        "pid" => {
                            if v.len() != 9 {
                                return None;
                            }

                            if v.parse::<i64>().is_ok() {
                                Some(k)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
                .collect();

            let has = |s| tags.contains(s);

            has("byr")
                && has("iyr")
                && has("eyr")
                && has("hgt")
                && has("hcl")
                && has("ecl")
                && has("pid")
        }

        for line in crate::input(crate::Day::day04).lines() {
            // for line in s.lines() {
            if line == "" {
                if check_passport(&current) {
                    count += 1;
                }
                current = Vec::new();
            }

            current.extend(line.split(' '));
        }

        if check_passport(&current) {
            count += 1;
        }

        assert_eq!(count, 184);

        Ok(())
    }
}
