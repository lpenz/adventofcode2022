// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day24::*;

fn process(bufin: impl BufRead) -> Result<Turn> {
    let input = parser::parse(bufin)?;
    let mut params = Params::new(input)?;
    // Good ol' BFS
    let t0 = params.bfs(0, params.start, params.target)?;
    let t1 = params.bfs(t0, params.target, params.start)?;
    let t2 = params.bfs(t1, params.start, params.target)?;
    Ok(t2)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 54);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
