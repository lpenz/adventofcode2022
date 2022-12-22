// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day12::*;

pub fn mvok(grid: &Grid, src: Qa, qr: Qr) -> Option<Qa> {
    let dst = (src + qr).ok()?;
    if (grid[dst] as u8) <= (grid[src] as u8) + 1 {
        Some(dst)
    } else {
        None
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let (src, dst, grid) = vecs2grid(input)?;
    let path = Sqrid::bfs_path(|qa, qr| mvok(&grid, qa, qr), &src, |qa| qa == dst)?;
    Ok(path.1.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 31);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
