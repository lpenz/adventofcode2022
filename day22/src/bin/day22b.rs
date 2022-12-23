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

#[derive(Debug, Clone, Copy, Default)]
pub struct Transition {
    pub id: usize,
    pub qr_new: Qr,
}

impl Transition {
    pub const fn new(id: usize, qr_new: Qr) -> Transition {
        Transition { id, qr_new }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Side {
    pub topleft: Qa,
    pub n: Transition,
    pub e: Transition,
    pub s: Transition,
    pub w: Transition,
}

pub struct Cube {
    pub side: u16,
    pub sides: [Side; 6],
}

impl Cube {
    pub fn eval(&self, qa0: Qa, qr0: Qr) -> Result<(Qa, Qr)> {
        let t = qa0.tuple();
        let side_src = self
            .sides
            .iter()
            .find(|side| {
                let tl = side.topleft.tuple();
                t.0 >= tl.0 && t.0 < tl.0 + self.side && t.1 >= tl.1 && t.1 < tl.1 + self.side
            })
            .cloned()
            .ok_or_else(|| eyre!("could not find original side of {:?}", qa0))?;
        let t_tl = side_src.topleft.tuple();
        let t_br = (t_tl.0 + self.side - 1, t_tl.1 + self.side - 1);
        if qr0 == Qr::N && t.1 > t_tl.1
            || qr0 == Qr::S && t.1 < t_br.1
            || qr0 == Qr::W && t.0 > t_tl.0
            || qr0 == Qr::E && t.0 < t_br.0
        {
            return Ok(((qa0 + qr0)?, qr0));
        }
        let rel = (
            t.0 - side_src.topleft.tuple().0,
            t.1 - side_src.topleft.tuple().1,
        );
        let trans = match qr0 {
            Qr::N => side_src.n,
            Qr::E => side_src.e,
            Qr::S => side_src.s,
            Qr::W => side_src.w,
            _ => panic!("unsupported Qr {}", qr0),
        };
        let side_dst = self.sides[trans.id];
        let qr_new = trans.qr_new;
        let x = match (qr0, qr_new) {
            (_, Qr::E) => side_dst.topleft.tuple().0,
            (_, Qr::W) => side_dst.topleft.tuple().0 + self.side - 1,
            (Qr::N, Qr::N) => side_dst.topleft.tuple().0 + rel.0,
            (Qr::S, Qr::S) => side_dst.topleft.tuple().0 + rel.0,
            (Qr::N, Qr::S) => side_dst.topleft.tuple().0 + self.side - 1 - rel.0,
            (Qr::S, Qr::N) => side_dst.topleft.tuple().0 + self.side - 1 - rel.0,
            (Qr::E, Qr::N) => side_dst.topleft.tuple().0 + rel.1,
            (Qr::E, Qr::S) => side_dst.topleft.tuple().0 + self.side - 1 - rel.1,
            (Qr::W, Qr::N) => side_dst.topleft.tuple().0 + self.side - 1 - rel.1,
            (Qr::W, Qr::S) => side_dst.topleft.tuple().0 + rel.1,
            _ => unimplemented!(),
        };
        let y = match (qr0, qr_new) {
            (_, Qr::S) => side_dst.topleft.tuple().1,
            (_, Qr::N) => side_dst.topleft.tuple().1 + self.side - 1,
            (Qr::E, Qr::E) => side_dst.topleft.tuple().1 + rel.1,
            (Qr::W, Qr::W) => side_dst.topleft.tuple().1 + rel.1,
            (Qr::E, Qr::W) => side_dst.topleft.tuple().1 + self.side - 1 - rel.1,
            (Qr::W, Qr::E) => side_dst.topleft.tuple().1 + self.side - 1 - rel.1,
            (Qr::N, Qr::E) => side_dst.topleft.tuple().1 + rel.0,
            (Qr::N, Qr::W) => side_dst.topleft.tuple().1 + self.side - 1 - rel.0,
            (Qr::S, Qr::E) => side_dst.topleft.tuple().1 + self.side - 1 - rel.0,
            (Qr::S, Qr::W) => side_dst.topleft.tuple().1 + rel.0,
            _ => unimplemented!(),
        };
        Ok((Qa::try_from((x, y))?, qr_new))
    }
}

pub const CUBE_EXAMPLE: Cube = Cube {
    side: 4,
    sides: [
        Side {
            // 0
            topleft: Qa::new::<8, 0>(),
            n: Transition::new(1, Qr::S),
            e: Transition::new(5, Qr::W),
            s: Transition::new(3, Qr::S),
            w: Transition::new(2, Qr::S),
        },
        Side {
            // 1
            topleft: Qa::new::<0, 4>(),
            n: Transition::new(0, Qr::S),
            e: Transition::new(2, Qr::E),
            s: Transition::new(4, Qr::N),
            w: Transition::new(5, Qr::N),
        },
        Side {
            // 2
            topleft: Qa::new::<4, 4>(),
            n: Transition::new(0, Qr::E),
            e: Transition::new(3, Qr::E),
            s: Transition::new(4, Qr::E),
            w: Transition::new(1, Qr::W),
        },
        Side {
            // 3
            topleft: Qa::new::<8, 4>(),
            n: Transition::new(0, Qr::N),
            e: Transition::new(5, Qr::S),
            s: Transition::new(4, Qr::S),
            w: Transition::new(2, Qr::W),
        },
        Side {
            // 4
            topleft: Qa::new::<8, 8>(),
            n: Transition::new(3, Qr::N),
            e: Transition::new(5, Qr::E),
            s: Transition::new(1, Qr::N),
            w: Transition::new(2, Qr::N),
        },
        Side {
            // 5
            topleft: Qa::new::<12, 8>(),
            n: Transition::new(3, Qr::W),
            e: Transition::new(0, Qr::W),
            s: Transition::new(1, Qr::E),
            w: Transition::new(4, Qr::W),
        },
    ],
};

// Translated form my input
const CUBE_INPUT: Cube = Cube {
    side: 50,
    sides: [
        Side {
            // 0
            topleft: Qa::new::<50, 0>(),
            n: Transition::new(5, Qr::E),
            e: Transition::new(1, Qr::E),
            s: Transition::new(2, Qr::S),
            w: Transition::new(3, Qr::E),
        },
        Side {
            // 1
            topleft: Qa::new::<100, 0>(),
            n: Transition::new(5, Qr::N),
            e: Transition::new(4, Qr::W),
            s: Transition::new(2, Qr::W),
            w: Transition::new(0, Qr::W),
        },
        Side {
            // 2
            topleft: Qa::new::<50, 50>(),
            n: Transition::new(0, Qr::N),
            e: Transition::new(1, Qr::N),
            s: Transition::new(4, Qr::S),
            w: Transition::new(3, Qr::S),
        },
        Side {
            // 3
            topleft: Qa::new::<0, 100>(),
            n: Transition::new(2, Qr::E),
            e: Transition::new(4, Qr::E),
            s: Transition::new(5, Qr::S),
            w: Transition::new(0, Qr::E),
        },
        Side {
            // 4
            topleft: Qa::new::<50, 100>(),
            n: Transition::new(2, Qr::N),
            e: Transition::new(1, Qr::W),
            s: Transition::new(5, Qr::W),
            w: Transition::new(3, Qr::W),
        },
        Side {
            // 5
            topleft: Qa::new::<0, 150>(),
            n: Transition::new(3, Qr::N),
            e: Transition::new(4, Qr::N),
            s: Transition::new(1, Qr::S),
            w: Transition::new(0, Qr::S),
        },
    ],
};

#[test]
fn test_eval() -> Result<()> {
    let cube = CUBE_EXAMPLE;
    let qa = |x, y| Qa::try_from((x, y)).unwrap();
    // 1 top-left N
    assert_eq!(
        cube.eval(qa(2 * cube.side, 0), Qr::N)?,
        (qa(cube.side - 1, cube.side), Qr::S)
    );
    // 1 top-right N
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 0), Qr::N)?,
        (qa(0, cube.side), Qr::S)
    );
    // 1 top-right E
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 0), Qr::E)?,
        (qa(4 * cube.side - 1, 3 * cube.side - 1), Qr::W)
    );
    // 1 bottom-right E
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, cube.side - 1), Qr::E)?,
        (qa(4 * cube.side - 1, 2 * cube.side), Qr::W)
    );
    // 1 bottom-left S
    assert_eq!(
        cube.eval(qa(2 * cube.side, cube.side - 1), Qr::S)?,
        (qa(2 * cube.side, cube.side), Qr::S)
    );
    // 1 bottom-right S
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, cube.side - 1), Qr::S)?,
        (qa(3 * cube.side - 1, cube.side), Qr::S)
    );
    // 1 top-left W
    assert_eq!(
        cube.eval(qa(2 * cube.side, 0), Qr::W)?,
        (qa(cube.side, cube.side), Qr::S)
    );
    // 1 bottom-left W
    assert_eq!(
        cube.eval(qa(2 * cube.side, cube.side - 1), Qr::W)?,
        (qa(2 * cube.side - 1, cube.side), Qr::S)
    );
    // 2 top-left N
    assert_eq!(
        cube.eval(qa(0, cube.side), Qr::N)?,
        (qa(3 * cube.side - 1, 0), Qr::S)
    );
    // 2 top-right N
    assert_eq!(
        cube.eval(qa(cube.side - 1, cube.side), Qr::N)?,
        (qa(2 * cube.side, 0), Qr::S)
    );
    // 2 top-right E
    assert_eq!(
        cube.eval(qa(cube.side - 1, cube.side), Qr::E)?,
        (qa(cube.side, cube.side), Qr::E)
    );
    // 2 bottom-right E
    assert_eq!(
        cube.eval(qa(cube.side - 1, 2 * cube.side - 1), Qr::E)?,
        (qa(cube.side, 2 * cube.side - 1), Qr::E)
    );
    // 2 bottom-left S
    assert_eq!(
        cube.eval(qa(0, 2 * cube.side - 1), Qr::S)?,
        (qa(3 * cube.side - 1, 3 * cube.side - 1), Qr::N)
    );
    // 2 bottom-right S
    assert_eq!(
        cube.eval(qa(cube.side - 1, 2 * cube.side - 1), Qr::S)?,
        (qa(2 * cube.side, 3 * cube.side - 1), Qr::N)
    );
    // 2 top-left W
    assert_eq!(
        cube.eval(qa(0, cube.side), Qr::W)?,
        (qa(4 * cube.side - 1, 3 * cube.side - 1), Qr::N)
    );
    // 2 bottom-left W
    assert_eq!(
        cube.eval(qa(0, 2 * cube.side - 1), Qr::W)?,
        (qa(3 * cube.side, 3 * cube.side - 1), Qr::N)
    );
    // 3 top-left N
    assert_eq!(
        cube.eval(qa(cube.side, cube.side), Qr::N)?,
        (qa(2 * cube.side, 0), Qr::E)
    );
    // 3 top-right N
    assert_eq!(
        cube.eval(qa(2 * cube.side - 1, cube.side), Qr::N)?,
        (qa(2 * cube.side, cube.side - 1), Qr::E)
    );
    // 3 top-right E
    assert_eq!(
        cube.eval(qa(2 * cube.side - 1, cube.side), Qr::E)?,
        (qa(2 * cube.side, cube.side), Qr::E)
    );
    // 3 bottom-right E
    assert_eq!(
        cube.eval(qa(2 * cube.side - 1, 2 * cube.side - 1), Qr::E)?,
        (qa(2 * cube.side, 2 * cube.side - 1), Qr::E)
    );
    // 3 bottom-left S
    assert_eq!(
        cube.eval(qa(cube.side, 2 * cube.side - 1), Qr::S)?,
        (qa(2 * cube.side, 3 * cube.side - 1), Qr::E)
    );
    // 3 bottom-right S
    assert_eq!(
        cube.eval(qa(2 * cube.side - 1, 2 * cube.side - 1), Qr::S)?,
        (qa(2 * cube.side, 2 * cube.side), Qr::E)
    );
    // 3 top-left W
    assert_eq!(
        cube.eval(qa(cube.side, cube.side), Qr::W)?,
        (qa(cube.side - 1, cube.side), Qr::W)
    );
    // 3 bottom-left W
    assert_eq!(
        cube.eval(qa(cube.side, 2 * cube.side - 1), Qr::W)?,
        (qa(cube.side - 1, 2 * cube.side - 1), Qr::W)
    );
    // 4 top-left N
    assert_eq!(
        cube.eval(qa(2 * cube.side, cube.side), Qr::N)?,
        (qa(2 * cube.side, cube.side - 1), Qr::N)
    );
    // 4 top-right N
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, cube.side), Qr::N)?,
        (qa(3 * cube.side - 1, cube.side - 1), Qr::N)
    );
    // 4 top-right E
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, cube.side), Qr::E)?,
        (qa(4 * cube.side - 1, 2 * cube.side), Qr::S)
    );
    // 4 bottom-right E
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 2 * cube.side - 1), Qr::E)?,
        (qa(3 * cube.side, 2 * cube.side), Qr::S)
    );
    // 4 bottom-left S
    assert_eq!(
        cube.eval(qa(2 * cube.side, 2 * cube.side - 1), Qr::S)?,
        (qa(2 * cube.side, 2 * cube.side), Qr::S)
    );
    // 4 bottom-right S
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 2 * cube.side - 1), Qr::S)?,
        (qa(3 * cube.side - 1, 2 * cube.side), Qr::S)
    );
    // 4 top-left W
    assert_eq!(
        cube.eval(qa(2 * cube.side, cube.side), Qr::W)?,
        (qa(2 * cube.side - 1, cube.side), Qr::W)
    );
    // 4 bottom-left W
    assert_eq!(
        cube.eval(qa(2 * cube.side, 2 * cube.side - 1), Qr::W)?,
        (qa(2 * cube.side - 1, 2 * cube.side - 1), Qr::W)
    );
    // 5 top-left N
    assert_eq!(
        cube.eval(qa(2 * cube.side, 2 * cube.side), Qr::N)?,
        (qa(2 * cube.side, 2 * cube.side - 1), Qr::N)
    );
    // 5 top-right N
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 2 * cube.side), Qr::N)?,
        (qa(3 * cube.side - 1, 2 * cube.side - 1), Qr::N)
    );
    // 5 top-right E
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 2 * cube.side), Qr::E)?,
        (qa(3 * cube.side, 2 * cube.side), Qr::E)
    );
    // 5 bottom-right E
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 3 * cube.side - 1), Qr::E)?,
        (qa(3 * cube.side, 3 * cube.side - 1), Qr::E)
    );
    // 5 bottom-left S
    assert_eq!(
        cube.eval(qa(2 * cube.side, 3 * cube.side - 1), Qr::S)?,
        (qa(cube.side - 1, 2 * cube.side - 1), Qr::N)
    );
    // 5 bottom-right S
    assert_eq!(
        cube.eval(qa(3 * cube.side - 1, 3 * cube.side - 1), Qr::S)?,
        (qa(0, 2 * cube.side - 1), Qr::N)
    );
    // 5 top-left W
    assert_eq!(
        cube.eval(qa(2 * cube.side, 2 * cube.side), Qr::W)?,
        (qa(2 * cube.side - 1, 2 * cube.side - 1), Qr::N)
    );
    // 5 bottom-left W
    assert_eq!(
        cube.eval(qa(2 * cube.side, 3 * cube.side - 1), Qr::W)?,
        (qa(cube.side, 2 * cube.side - 1), Qr::N)
    );
    // 6 top-left N
    assert_eq!(
        cube.eval(qa(3 * cube.side, 2 * cube.side), Qr::N)?,
        (qa(3 * cube.side - 1, 2 * cube.side - 1), Qr::W)
    );
    // 6 top-right N
    assert_eq!(
        cube.eval(qa(4 * cube.side - 1, 2 * cube.side), Qr::N)?,
        (qa(3 * cube.side - 1, cube.side), Qr::W)
    );
    // 6 top-right E
    assert_eq!(
        cube.eval(qa(4 * cube.side - 1, 2 * cube.side), Qr::E)?,
        (qa(3 * cube.side - 1, cube.side - 1), Qr::W)
    );
    // 6 bottom-right E
    assert_eq!(
        cube.eval(qa(4 * cube.side - 1, 3 * cube.side - 1), Qr::E)?,
        (qa(3 * cube.side - 1, 0), Qr::W)
    );
    // 6 bottom-left S
    assert_eq!(
        cube.eval(qa(3 * cube.side, 3 * cube.side - 1), Qr::S)?,
        (qa(0, 2 * cube.side - 1), Qr::E)
    );
    // 6 bottom-right S
    assert_eq!(
        cube.eval(qa(4 * cube.side - 1, 3 * cube.side - 1), Qr::S)?,
        (qa(0, cube.side), Qr::E)
    );
    // 6 top-left W
    assert_eq!(
        cube.eval(qa(3 * cube.side, 2 * cube.side), Qr::W)?,
        (qa(3 * cube.side - 1, 2 * cube.side), Qr::W)
    );
    // 6 bottom-left W
    assert_eq!(
        cube.eval(qa(3 * cube.side, 3 * cube.side - 1), Qr::W)?,
        (qa(3 * cube.side - 1, 3 * cube.side - 1), Qr::W)
    );
    Ok(())
}

pub fn test_cycles(cube: &Cube) -> Result<()> {
    let max_steps = cube.side * 4;
    for side in cube.sides {
        let tl = side.topleft.tuple();
        for dy in 0..cube.side {
            for dx in 0..cube.side {
                let qa0 = Qa::try_from((tl.0 + dx, tl.1 + dy))?;
                for qr0 in Qr::iter::<false>() {
                    let (mut qa, mut qr) = cube.eval(qa0, qr0)?;
                    let mut steps = 1;
                    while qa != qa0 {
                        (qa, qr) = cube.eval(qa, qr)?;
                        steps += 1;
                        if steps > max_steps {
                            return Err(eyre!("error evaluating from {:?} to {}", qa0, qr0));
                        }
                    }
                    assert_eq!(steps, max_steps);
                }
            }
        }
    }
    Ok(())
}

#[test]
fn test_cycles_example() -> Result<()> {
    test_cycles(&CUBE_EXAMPLE)
}

#[test]
fn test_cycles_input() -> Result<()> {
    test_cycles(&CUBE_INPUT)
}

fn process(cube: &Cube, bufin: impl BufRead) -> Result<i32> {
    let (board0, instructions) = parser::parse(bufin)?;
    let (grid, mut qa, _) = board2grid(board0)?;
    let mut qr = Qr::E;
    for instr in instructions {
        match instr {
            Instr::Walk(steps) => {
                for _ in 0..steps {
                    let (newqa, newqr) = cube.eval(qa, qr)?;
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
    assert_eq!(process(&CUBE_EXAMPLE, EXAMPLE.as_bytes())?, 5031);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(&CUBE_INPUT, stdin().lock())?);
    Ok(())
}
