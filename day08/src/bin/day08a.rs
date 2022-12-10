// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day08::*;

fn check_visible(
    input: &[Vec<i8>],
    visible: &mut HashSet<(usize, usize)>,
    xy: (usize, usize),
    mut tallest: i8,
) -> i8 {
    let height = input[xy.1][xy.0];
    if height > tallest {
        visible.insert(xy);
        tallest = height;
    }
    tallest
}

fn process<const SIDE: usize>(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut visible = HashSet::<(usize, usize)>::default();
    // Process columns
    for x in 0..SIDE {
        (0..SIDE).fold(-1, |tallest, y| {
            check_visible(&input, &mut visible, (x, y), tallest)
        });
        (0..SIDE).rev().fold(-1, |tallest, y| {
            check_visible(&input, &mut visible, (x, y), tallest)
        });
    }
    // Process lines
    for y in 0..SIDE {
        (0..SIDE).fold(-1, |tallest, x| {
            check_visible(&input, &mut visible, (x, y), tallest)
        });
        (0..SIDE).rev().fold(-1, |tallest, x| {
            check_visible(&input, &mut visible, (x, y), tallest)
        });
    }
    Ok(visible.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process::<5>(EXAMPLE.as_bytes())?, 21);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process::<99>(stdin().lock())?);
    Ok(())
}
