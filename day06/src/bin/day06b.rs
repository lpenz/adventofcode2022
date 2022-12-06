// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day06::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    solve::<14>(input)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 19);
    assert_eq!(process(EXAMPLE2.as_bytes())?, 23);
    assert_eq!(process(EXAMPLE3.as_bytes())?, 23);
    assert_eq!(process(EXAMPLE4.as_bytes())?, 29);
    assert_eq!(process(EXAMPLE5.as_bytes())?, 26);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
