// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day10::*;

type Sqrid = sqrid::sqrid_create!(40, 6, false);
type Qa = sqrid::qa_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, char);

fn process(bufin: impl BufRead) -> Result<Grid> {
    let input = parser::parse(bufin)?;
    let mut state = State::default();
    let mut grid = Grid::repeat('.');
    let mut qaiter = Qa::iter();
    for instr in input {
        state.load(instr);
        while state.due != 0 {
            let pos = qaiter.next().unwrap();
            let posx = pos.tuple().0 as i32;
            if posx == state.x || posx == state.x - 1 || posx == state.x + 1 {
                grid[pos] = '#';
            }
            state.tick();
        }
    }
    Ok(grid)
}

#[test]
fn test() -> Result<()> {
    let answer = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....";
    let grid = process(EXAMPLE2.as_bytes())?;
    assert_eq!(grid.iter().collect::<String>(), answer);
    println!("{}", grid);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
