// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Report, Result};
use std::collections::VecDeque;
use std::fmt;
use std::io::{stdin, BufRead};

use day25::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Snafu(pub i64);

impl Snafu {
    pub fn digit2value(c: char) -> i64 {
        match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid digit"),
        }
    }

    pub fn new(s: &str) -> Result<Snafu> {
        Ok(Snafu(
            s.chars()
                .rev()
                .fold((1_i64, 0_i64), |(weight, total), digit| {
                    (weight * 5_i64, total + weight * Snafu::digit2value(digit))
                })
                .1,
        ))
    }
}

impl std::str::FromStr for Snafu {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Snafu::new(s)
    }
}

impl From<i64> for Snafu {
    fn from(v: i64) -> Self {
        Snafu(v)
    }
}

impl From<Snafu> for i64 {
    fn from(snafu: Snafu) -> Self {
        snafu.0
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = VecDeque::new();
        let mut rest = self.0;
        // First do a direct base 5 conversion:
        while rest > 0 {
            let rem = rest % 5;
            result.push_front(rem);
            rest = (rest - rem) / 5;
        }
        // Now check overflows and balance the previous number:
        let mut run = true;
        while run {
            run = false;
            result.push_front(0);
            for i in 1..result.len() {
                let value = result[i];
                if value > 2 {
                    result[i - 1] += 1;
                    result[i] -= 5;
                    run = true;
                }
            }
        }
        // Print the values, skipping the leading zeroes:
        write!(
            f,
            "{}",
            result
                .into_iter()
                .skip_while(|&v| v == 0)
                .map(|value| {
                    match value {
                        2 => '2',
                        1 => '1',
                        0 => '0',
                        -1 => '-',
                        -2 => '=',
                        _ => panic!("bug in fmt::Display of Snafu"),
                    }
                })
                .collect::<String>()
        )
    }
}

#[test]
fn test_conversions() -> Result<()> {
    let cases = vec![
        ("1=-0-2", 1747_i64),
        ("12111", 906_i64),
        ("2=0=", 198_i64),
        ("21", 11_i64),
        ("2=01", 201_i64),
        ("111", 31_i64),
        ("20012", 1257_i64),
        ("112", 32_i64),
        ("1=-1=", 353_i64),
        ("1-12", 107_i64),
        ("12", 7_i64),
        ("1=", 3_i64),
        ("122", 37_i64),
    ];
    for case in cases {
        assert_eq!(Snafu::new(case.0)?, case.1.into());
        assert_eq!(format!("{}", Snafu::new(case.0)?), case.0);
    }
    Ok(())
}

fn process(bufin: impl BufRead) -> Result<Snafu> {
    let input = parser::parse(bufin)?;
    let snafus: Vec<Snafu> = input
        .into_iter()
        .map(|s| Snafu::new(&s))
        .collect::<Result<Vec<Snafu>>>()?;
    Ok(snafus
        .into_iter()
        .fold(0, |acc, snafu| acc + snafu.0)
        .into())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(format!("{}", process(EXAMPLE.as_bytes())?), "2=-1=0");
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
