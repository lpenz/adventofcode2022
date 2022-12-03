// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day03::*;

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|rucksack| {
            // Compartment size:
            let csize = rucksack.len() / 2;
            let c1 = rucksack[0..csize]
                .iter()
                .copied()
                .collect::<HashSet<Item>>();
            let c2 = rucksack[csize..].iter().copied().collect::<HashSet<Item>>();
            (c1.intersection(&c2))
                .map(|item| item.priority() as u32)
                .sum::<u32>()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 157);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
