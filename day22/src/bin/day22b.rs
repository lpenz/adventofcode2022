// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day22::*;

fn board2grid(board: Vec<Vec<Cell>>) -> Result<(Grid, Qa, u16)> {
    let mut grid = Grid::default();
    let mut start = None;
    let mut side = 0;
    for (y, line) in board.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            grid[qa] = cell;
            if start.is_none() && cell != Cell::Blank {
                start = Some(qa);
                side = x as u16 / 2;
            }
        }
    }
    Ok((grid, start.unwrap(), side))
}

fn step(side: u16, qa: Qa, qr: Qr) -> Result<(Qa, Qr)> {
    let t = qa.tuple();
    if t.1 == 0 && qr == Qr::N {
        // 1 N, go N:
        Ok((Qa::try_from((3 * side - t.0 - 1, side))?, Qr::S))
    } else if t.0 == 2 * side && t.1 < side && qr == Qr::W {
        // 1 W, go W:
        Ok((Qa::try_from((side + t.1, side))?, Qr::S))
    } else if t.0 == 3 * side - 1 && t.1 < side && qr == Qr::E {
        // 1 E, go E:
        Ok((Qa::try_from((4 * side - 1, 3 * side - 1 - t.1))?, Qr::W))
    } else if t.0 < side && t.1 == side && qr == Qr::N {
        // 2 N, go N:
        Ok((Qa::try_from((3 * side - 1 - t.0, 0))?, Qr::S))
    } else if t.0 == 0 && qr == Qr::W {
        // 2 W, go W:
        Ok((
            Qa::try_from((4 * side - 1 - (t.1 - side), 3 * side - 1))?,
            Qr::N,
        ))
    } else if t.0 < side && t.1 == 2 * side - 1 && qr == Qr::S {
        // 2 S, go S:
        Ok((Qa::try_from((3 * side - 1 - t.0, 3 * side - 1))?, Qr::N))
    } else if side <= t.0 && t.0 < 2 * side && t.1 == side && qr == Qr::N {
        // 3 N, go N:
        Ok((Qa::try_from((side * 2, t.0 - side))?, Qr::E))
    } else if side <= t.0 && t.0 < 2 * side && t.1 == 2 * side - 1 && qr == Qr::S {
        // 3 S, go S:
        Ok((
            Qa::try_from((side * 2, 3 * side - 1 - (t.0 - side)))?,
            Qr::E,
        ))
    } else if t.0 == 3 * side - 1 && t.1 >= side && t.1 < 2 * side && qr == Qr::E {
        // 4 E, go E:
        Ok((Qa::try_from((side * 5 - 1 - t.1, 2 * side))?, Qr::S))
    } else if t.0 == 2 * side && t.1 >= 2 * side && qr == Qr::W {
        // 5 W, go W:
        Ok((Qa::try_from((4 * side - 1 - t.1, 2 * side - 1))?, Qr::N))
    } else if t.0 < 3 * side && t.1 == 3 * side - 1 && qr == Qr::S {
        // 5 S, go S:
        Ok((
            Qa::try_from((side - 1 - (t.0 - 2 * side), 2 * side - 1))?,
            Qr::N,
        ))
    } else if t.0 >= 3 * side && t.1 == 2 * side && qr == Qr::N {
        // 6 N, go N:
        Ok((
            Qa::try_from((3 * side - 1, 2 * side - 1 - (t.0 - 3 * side)))?,
            Qr::W,
        ))
    } else if t.0 == 4 * side - 1 && qr == Qr::E {
        // 6 E, go E:
        Ok((
            Qa::try_from((3 * side - 1, side - 1 - (t.1 - 2 * side)))?,
            Qr::W,
        ))
    } else if t.0 >= 3 * side && t.1 == 3 * side - 1 && qr == Qr::S {
        // 6 S, go S:
        Ok((Qa::try_from((0, 2 * side - 1 - (t.0 - 3 * side)))?, Qr::E))
    } else {
        // panic!("not implemented: {:?} {:?}", qa, qr);
        sqrid::qaqr_resolve(qa, qr)
            .map(|qa| (qa, qr))
            .map_err(|e| eyre!(e))
    }
}

#[test]
fn test_step() -> Result<()> {
    const SIDE: u16 = 4;
    let qa = |x, y| Qa::try_from((x, y)).unwrap();
    // 1 top-left N
    assert_eq!(
        step(SIDE, qa(2 * SIDE, 0), Qr::N)?,
        (qa(SIDE - 1, SIDE), Qr::S)
    );
    // 1 top-right N
    assert_eq!(
        step(SIDE, qa(3 * SIDE - 1, 0), Qr::N)?,
        (qa(0, SIDE), Qr::S)
    );
    // 1 top-left W
    assert_eq!(step(SIDE, qa(2 * SIDE, 0), Qr::W)?, (qa(SIDE, SIDE), Qr::S));
    // 1 bottom-left W
    assert_eq!(
        step(SIDE, qa(2 * SIDE, SIDE - 1), Qr::W)?,
        (qa(2 * SIDE - 1, SIDE), Qr::S)
    );
    // 1 top-right E
    assert_eq!(
        step(SIDE, qa(3 * SIDE - 1, 0), Qr::E)?,
        (qa(4 * SIDE - 1, 3 * SIDE - 1), Qr::W)
    );
    // 1 bottom-right E
    assert_eq!(
        step(SIDE, qa(3 * SIDE - 1, SIDE - 1), Qr::E)?,
        (qa(4 * SIDE - 1, 2 * SIDE), Qr::W)
    );
    // 2 top-left N
    assert_eq!(
        step(SIDE, qa(0, SIDE), Qr::N)?,
        (qa(3 * SIDE - 1, 0), Qr::S)
    );
    // 2 top-right N
    assert_eq!(
        step(SIDE, qa(SIDE - 1, SIDE), Qr::N)?,
        (qa(2 * SIDE, 0), Qr::S)
    );
    // 2 top-left W
    assert_eq!(
        step(SIDE, qa(0, SIDE), Qr::W)?,
        (qa(4 * SIDE - 1, 3 * SIDE - 1), Qr::N)
    );
    // 2 bottom-left W
    assert_eq!(
        step(SIDE, qa(0, 2 * SIDE - 1), Qr::W)?,
        (qa(3 * SIDE, 3 * SIDE - 1), Qr::N)
    );
    // 2 bottom-left S
    assert_eq!(
        step(SIDE, qa(0, 2 * SIDE - 1), Qr::S)?,
        (qa(3 * SIDE - 1, 3 * SIDE - 1), Qr::N)
    );
    // 2 bottom-right S
    assert_eq!(
        step(SIDE, qa(SIDE - 1, 2 * SIDE - 1), Qr::S)?,
        (qa(2 * SIDE, 3 * SIDE - 1), Qr::N)
    );
    // 3 top-left N
    assert_eq!(step(SIDE, qa(SIDE, SIDE), Qr::N)?, (qa(2 * SIDE, 0), Qr::E));
    // 3 top-right N
    assert_eq!(
        step(SIDE, qa(2 * SIDE - 1, SIDE), Qr::N)?,
        (qa(2 * SIDE, SIDE - 1), Qr::E)
    );
    // 3 bottom-left S
    assert_eq!(
        step(SIDE, qa(SIDE, 2 * SIDE - 1), Qr::S)?,
        (qa(2 * SIDE, 3 * SIDE - 1), Qr::E)
    );
    // 3 bottom-right S
    assert_eq!(
        step(SIDE, qa(2 * SIDE - 1, 2 * SIDE - 1), Qr::S)?,
        (qa(2 * SIDE, 2 * SIDE), Qr::E)
    );
    // 4 top-right E
    assert_eq!(
        step(SIDE, qa(3 * SIDE - 1, SIDE), Qr::E)?,
        (qa(4 * SIDE - 1, 2 * SIDE), Qr::S)
    );
    // 4 bottom-right E
    assert_eq!(
        step(SIDE, qa(3 * SIDE - 1, 2 * SIDE - 1), Qr::E)?,
        (qa(3 * SIDE, 2 * SIDE), Qr::S)
    );
    // 5 top-left W
    assert_eq!(
        step(SIDE, qa(2 * SIDE, 2 * SIDE), Qr::W)?,
        (qa(2 * SIDE - 1, 2 * SIDE - 1), Qr::N)
    );
    // 5 bottom-left W
    assert_eq!(
        step(SIDE, qa(2 * SIDE, 3 * SIDE - 1), Qr::W)?,
        (qa(SIDE, 2 * SIDE - 1), Qr::N)
    );
    // 5 bottom-left S
    assert_eq!(
        step(SIDE, qa(2 * SIDE, 3 * SIDE - 1), Qr::S)?,
        (qa(SIDE - 1, 2 * SIDE - 1), Qr::N)
    );
    // 5 bottom-right S
    assert_eq!(
        step(SIDE, qa(3 * SIDE - 1, 3 * SIDE - 1), Qr::S)?,
        (qa(0, 2 * SIDE - 1), Qr::N)
    );
    // 6 top-left N
    assert_eq!(
        step(SIDE, qa(3 * SIDE, 2 * SIDE), Qr::N)?,
        (qa(3 * SIDE - 1, 2 * SIDE - 1), Qr::W)
    );
    // 6 top-right N
    assert_eq!(
        step(SIDE, qa(4 * SIDE - 1, 2 * SIDE), Qr::N)?,
        (qa(3 * SIDE - 1, SIDE), Qr::W)
    );
    // 6 top-right E
    assert_eq!(
        step(SIDE, qa(4 * SIDE - 1, 2 * SIDE), Qr::E)?,
        (qa(3 * SIDE - 1, SIDE - 1), Qr::W)
    );
    // 6 bottom-right E
    assert_eq!(
        step(SIDE, qa(4 * SIDE - 1, 3 * SIDE - 1), Qr::E)?,
        (qa(3 * SIDE - 1, 0), Qr::W)
    );
    // 6 bottom-left S
    assert_eq!(
        step(SIDE, qa(3 * SIDE, 3 * SIDE - 1), Qr::S)?,
        (qa(0, 2 * SIDE - 1), Qr::E)
    );
    // 6 bottom-right S
    assert_eq!(
        step(SIDE, qa(4 * SIDE - 1, 3 * SIDE - 1), Qr::S)?,
        (qa(0, SIDE), Qr::E)
    );
    Ok(())
}

#[test]
fn test_cycles() -> Result<()> {
    let (board0, _) = parser::parse(EXAMPLE.as_bytes())?;
    let (grid, _, side) = board2grid(board0)?;
    for qa0 in Qa::iter().filter(|qa| grid[qa] != Cell::Blank) {
        for qr0 in Qr::iter::<false>() {
            let (mut qa, mut qr) = step(side, qa0, qr0)?;
            let mut steps = 1;
            while qa != qa0 {
                (qa, qr) = step(side, qa, qr)?;
                steps += 1;
            }
            assert_eq!(steps, 16);
        }
    }
    Ok(())
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let (board0, instructions) = parser::parse(bufin)?;
    let (grid, mut qa, side) = board2grid(board0)?;
    let mut qr = Qr::E;
    for instr in instructions {
        match instr {
            Instr::Walk(steps) => {
                for _ in 0..steps {
                    let (newqa, newqr) = step(side, qa, qr)?;
                    assert!(grid[newqa] != Cell::Blank);
                    if grid[newqa] != Cell::Wall {
                        qa = newqa;
                        qr = newqr;
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
    assert_eq!(process(EXAMPLE.as_bytes())?, 5031);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
