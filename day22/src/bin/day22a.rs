// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day22::*;

fn board2grid(board: Vec<Vec<Cell>>) -> Result<Grid> {
    let mut grid = Grid::default();
    for (y, line) in board.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            grid[qa] = cell;
        }
    }
    Ok(grid)
}

fn wrap_qa(qa: Qa, qr: Qr) -> Result<Qa> {
    let t = qa.tuple();
    match qr {
        Qr::N => Ok(Qa::try_from((t.0, Qa::HEIGHT - 1))?),
        Qr::E => Ok(Qa::try_from((0, t.1))?),
        Qr::S => Ok(Qa::try_from((t.0, 0))?),
        Qr::W => Ok(Qa::try_from((Qa::WIDTH - 1, t.1))?),
        _ => panic!("invalid direction {:?}", qr),
    }
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let (board0, instructions) = parser::parse(bufin)?;
    let grid = board2grid(board0)?;
    let mut qa = grid
        .iter_qa()
        .filter_map(|(qa, &c)| Some(qa).filter(|_| c == Cell::Open))
        .next()
        .ok_or_else(|| eyre!("could not find an open space"))?;
    let mut qr = Qr::E;
    for instr in instructions {
        match instr {
            Instr::Walk(steps) => {
                for _ in 0..steps {
                    let newqa = qa + qr;
                    let newqa_cell = newqa.as_ref().map(|newqa| grid[newqa]);
                    if newqa.is_err() || newqa_cell == Ok(Cell::Blank) {
                        // wrap
                        let mut newqa = wrap_qa(qa, qr)?;
                        let mut wall = false;
                        while grid[newqa] != Cell::Open {
                            if grid[newqa] == Cell::Wall {
                                wall = true;
                                break;
                            }
                            newqa = (newqa + qr)?;
                        }
                        if !wall {
                            qa = newqa;
                        }
                    } else if newqa_cell == Ok(Cell::Open) {
                        qa = newqa.unwrap_or_else(|_| panic!("error in open cell branch"));
                    } else {
                        // Wall, stop here
                        break;
                    }
                }
            }
            Instr::Turn(turnqr) => {
                qr += turnqr;
            }
        }
    }
    let t = qa.tuple();
    let facing = match qr {
        Qr::N => 3,
        Qr::E => 0,
        Qr::S => 1,
        Qr::W => 2,
        _ => panic!("unsupported direction"),
    };
    Ok(1000 * (t.1 as i32 + 1) + 4 * (t.0 as i32 + 1) + facing)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 6032);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
