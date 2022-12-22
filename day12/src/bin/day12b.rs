// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day12::*;

pub fn back_mvok(grid: &Grid, src: Qa, qr: Qr) -> Option<Qa> {
    let dst = (src + qr).ok()?;
    if (grid[src] as u8) <= (grid[dst] as u8) + 1 {
        Some(dst)
    } else {
        None
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let (_, dst, grid) = vecs2grid(input)?;
    let path = Sqrid::bfs_path(
        |qa, qr| back_mvok(&grid, qa, qr),
        &dst,
        |qa| grid[qa] == 'a',
    )?;
    Ok(path.1.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 29);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
