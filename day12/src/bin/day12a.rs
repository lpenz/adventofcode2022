// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use sqrid::Qr;

use day12::*;

type Sqrid = sqrid::sqrid_create!(143, 41, false);
type Qa = sqrid::qa_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, char);

fn mvok(grid: &Grid, src: Qa, qr: Qr) -> Option<Qa> {
    let dst = (src + qr)?;
    if (grid[dst] as u8) <= (grid[src] as u8) + 1 {
        Some(dst)
    } else {
        None
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::repeat(char::from(b'~'));
    let mut src = Qa::default();
    let mut dst = Qa::default();
    for (y, line) in input.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            if line[x] == 'S' {
                src = qa;
                grid[qa] = 'a';
            } else if line[x] == 'E' {
                dst = qa;
                grid[qa] = 'z';
            } else {
                grid[qa] = *cell;
            }
        }
    }
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
