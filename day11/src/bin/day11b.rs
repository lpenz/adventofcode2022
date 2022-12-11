// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day11::*;

fn process(bufin: impl BufRead) -> Result<u64> {
    let monkeys = parser::parse(bufin)?;
    monkey_business::<10000, 1>(monkeys)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 2713310158);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
