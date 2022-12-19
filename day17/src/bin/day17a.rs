// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

type Qa = sqrid::Qa<7, { u16::MAX }>;
type Grid = HashSet<Qa>;

#[derive(Debug, Clone)]
pub struct Shape(pub Vec<Qa>);

impl Shape {
    pub fn new_dash() -> Shape {
        Shape(vec![
            Qa::new::<0, 0>(),
            Qa::new::<1, 0>(),
            Qa::new::<2, 0>(),
            Qa::new::<3, 0>(),
        ])
    }
    pub fn new_plus() -> Shape {
        Shape(vec![
            Qa::new::<1, 0>(),
            Qa::new::<0, 1>(),
            Qa::new::<1, 1>(),
            Qa::new::<2, 1>(),
            Qa::new::<1, 2>(),
        ])
    }
    pub fn new_l() -> Shape {
        Shape(vec![
            Qa::new::<0, 0>(),
            Qa::new::<1, 0>(),
            Qa::new::<2, 0>(),
            Qa::new::<2, 1>(),
            Qa::new::<2, 2>(),
        ])
    }
    pub fn new_i() -> Shape {
        Shape(vec![
            Qa::new::<0, 0>(),
            Qa::new::<0, 1>(),
            Qa::new::<0, 2>(),
            Qa::new::<0, 3>(),
        ])
    }
    pub fn new_o() -> Shape {
        Shape(vec![
            Qa::new::<0, 0>(),
            Qa::new::<0, 1>(),
            Qa::new::<1, 0>(),
            Qa::new::<1, 1>(),
        ])
    }
    pub fn place(&mut self, qa: Qa) {
        let qat = qa.tuple();
        for point in &mut self.0 {
            let t = point.tuple();
            *point = Qa::try_from((qat.0 + t.0, qat.1 + t.1)).unwrap();
        }
    }
    pub fn mv(&mut self, grid: &Grid, qr: Qr) -> bool {
        let rock_result: Result<Vec<_>, _> = self
            .0
            .iter()
            .map(|&qa| {
                if let Some(qa) = qa + qr {
                    if !grid.contains(&qa) {
                        Ok(qa)
                    } else {
                        Err(eyre!("overlap with rock"))
                    }
                } else {
                    Err(eyre!("out of bounds"))
                }
            })
            .collect();
        if let Ok(rocks) = rock_result {
            self.0 = rocks;
            true
        } else {
            false
        }
    }
    pub fn rested(&self, grid: &Grid) -> bool {
        self.0
            .iter()
            .any(|&qa| grid.contains(&(qa + Qr::N).unwrap()))
    }
}

use day17::*;

fn process(bufin: impl BufRead) -> Result<u32> {
    let jets = parser::parse(bufin)?;
    let mut grid = Grid::new();
    for x in 0..7 {
        grid.insert(Qa::try_from((x, 0)).unwrap());
    }
    let mut jetiter = jets.iter().cycle();
    for irocks in 0..2022 {
        let mut shape = match irocks % 5 {
            0 => Shape::new_dash(),
            1 => Shape::new_plus(),
            2 => Shape::new_l(),
            3 => Shape::new_i(),
            4 => Shape::new_o(),
            _ => panic!(),
        };
        let ymax = grid.iter().map(|qa| qa.tuple().1).max().unwrap();
        let spawner = Qa::try_from((2, ymax + 4)).unwrap();
        shape.place(spawner);
        // debug(&grid, &shape);
        loop {
            let jet = *jetiter.next().unwrap();
            // eprintln!("jet {}", jet.name_utf8());
            shape.mv(&grid, jet);
            // debug(&grid, &shape);
            if shape.rested(&grid) {
                for qa in shape.0.iter() {
                    grid.insert(*qa);
                }
                // debug(&grid, &shape);
                break;
            } else {
                shape.mv(&grid, Qr::N);
                // debug(&grid, &shape);
            }
        }
    }
    let height = grid.iter().map(|qa| qa.tuple().1 as u32).max().unwrap();
    Ok(height)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 3068);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}

pub type SqridD = sqrid::sqrid_create!(7, 30, false);
pub type QaD = sqrid::qa_create!(SqridD);
pub type GridD = sqrid::grid_create!(SqridD, char);

pub fn debug(grid: &HashSet<Qa>, shape: &Shape) {
    let mut g = GridD::repeat('.');
    for rock in grid {
        let mut t = rock.tuple();
        t.1 = 29 - t.1;
        let qad = QaD::try_from(t).unwrap();
        g[qad] = '#';
    }
    for rock in &shape.0 {
        let mut t = rock.tuple();
        t.1 = 29 - t.1;
        g[QaD::try_from(t).unwrap()] = '@';
    }
    eprintln!("{}", g);
}
