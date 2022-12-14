// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::eyre;
use eyre::Result;
use std::io::{stdin, BufRead};

use day14::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = grid_from_paths(input)?;
    let y_max = grid
        .iter_qa()
        .filter_map(|(qa, &cell)| {
            if cell == Cell::Rock {
                Some(qa.tuple().1)
            } else {
                None
            }
        })
        .max()
        .ok_or_else(|| eyre!("could not find max y"))?;
    loop {
        let mut qa = Qa::new::<500, 0>();
        let mut moved = true;
        while moved {
            if qa.tuple().1 > y_max {
                grid[qa] = Cell::Empty;
                return Ok(grid.iter().filter(|&c| c == &Cell::Sand).count());
            }
            if let Some(newqa) = [Qr::S, Qr::SW, Qr::SE]
                .iter()
                .filter_map(|qr| {
                    (qa + qr).and_then(|qa| Some(qa).filter(|qa| grid[qa] == Cell::Empty))
                })
                .next()
            {
                grid[qa] = Cell::Empty;
                qa = newqa;
                grid[qa] = Cell::Sand;
                moved = true;
            } else {
                moved = false;
            }
        }
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 24);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
