// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day01::*;

fn process(bufin: impl BufRead) -> Result<Calories> {
    let input = parser::parse(bufin)?;
    let top = itertools::max(
        input
            .into_iter()
            .map(|elf| elf.into_iter().sum::<Calories>()),
    )
    .unwrap();
    Ok(top)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 24000);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
