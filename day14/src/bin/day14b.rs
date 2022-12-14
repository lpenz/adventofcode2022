// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day14::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = grid_from_paths(input)?;
    let y_max = grid_y_max_rock(&grid)?;
    lay_rock(
        &mut grid,
        &Qa::try_from((0, y_max + 2)).unwrap(),
        &Qa::try_from((Qa::WIDTH - 1, y_max + 2)).unwrap(),
    )?;
    const SRC: Qa = Qa::new::<500, 0>();
    loop {
        let mut qa = SRC;
        let mut moved = true;
        while moved {
            if grid[SRC] == Cell::Sand {
                return Ok(grid.iter().filter(|&c| c == &Cell::Sand).count());
            }
            grid[qa] = Cell::Sand;
            moved = sand_fall(&mut grid, &mut qa);
        }
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 93);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
