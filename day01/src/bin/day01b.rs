// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::BinaryHeap;
use std::io::{stdin, BufRead};

use day01::*;

fn process(bufin: impl BufRead) -> Result<Calories> {
    let input = parser::parse(bufin)?;
    let mut perelf = input
        .into_iter()
        .map(|elf| elf.into_iter().sum::<Calories>())
        .collect::<BinaryHeap<Calories>>();
    // BinaryHeap's iter is random, and into_iter_sorted is only in nightly atm
    Ok(perelf.pop().unwrap() + perelf.pop().unwrap() + perelf.pop().unwrap())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 45000);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
