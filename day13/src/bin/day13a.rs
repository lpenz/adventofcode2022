// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::cmp::Ord;
use std::cmp::Ordering::Less;
use std::io::{stdin, BufRead};

use day13::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair.0.cmp(&pair.1) == Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 13);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
