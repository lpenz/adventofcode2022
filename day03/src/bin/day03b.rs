// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day03::*;

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let sets = input
        .into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<Item>>())
        .collect::<Vec<_>>();
    Ok(num::range_step(0, sets.len(), 3)
        .map(|i| {
            sets[i]
                .intersection(&sets[i + 1])
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&sets[i + 2])
                .map(|item| item.priority() as u32)
                .sum::<u32>()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 70);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
