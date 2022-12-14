// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day06::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    solve::<4>(input)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 7);
    assert_eq!(process(EXAMPLE2.as_bytes())?, 5);
    assert_eq!(process(EXAMPLE3.as_bytes())?, 6);
    assert_eq!(process(EXAMPLE4.as_bytes())?, 10);
    assert_eq!(process(EXAMPLE5.as_bytes())?, 11);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
