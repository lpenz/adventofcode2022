// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day08::*;

fn process<const SIDE: usize>(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let visible = visible_trees::<SIDE>(&input)?;
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
