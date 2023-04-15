// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
pub use sqrid::Qr;
use std::collections::HashMap;
use std::io::BufRead;

// At turn start:
// y = 0 shape
// y = 1 shape
// y = 2 shape
// y = 3 shape
// y = 4
// y = 5
// y = 6
// y = 7 grid
// y = 8 grid
// y = 9 grid
// y =10 grid

pub const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn w(input: &str) -> IResult<&str, Qr> {
        let (input, _) = bytes::tag("<")(input)?;
        Ok((input, Qr::W))
    }

    fn e(input: &str) -> IResult<&str, Qr> {
        let (input, _) = bytes::tag(">")(input)?;
        Ok((input, Qr::E))
    }

    fn dir(input: &str) -> IResult<&str, Qr> {
        branch::alt((w, e))(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<Qr>> {
        let (input, dirs) = multi::many1(dir)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, dirs))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Qr>> {
        aoc::parse_with!(line, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 40);
    Ok(())
}

const GRIDTOP: u16 = 7;

type Qa = sqrid::Qa<7, { u16::MAX }>;

#[derive(Debug, Clone)]
pub struct Shape(pub Vec<Qa>);

impl Shape {
    pub fn new_dash() -> Shape {
        Shape(vec![
            Qa::new::<0, 3>(),
            Qa::new::<1, 3>(),
            Qa::new::<2, 3>(),
            Qa::new::<3, 3>(),
        ])
    }
    pub fn new_plus() -> Shape {
        Shape(vec![
            Qa::new::<1, 1>(),
            Qa::new::<0, 2>(),
            Qa::new::<1, 2>(),
            Qa::new::<2, 2>(),
            Qa::new::<1, 3>(),
        ])
    }
    pub fn new_l() -> Shape {
        Shape(vec![
            Qa::new::<0, 3>(),
            Qa::new::<1, 3>(),
            Qa::new::<2, 3>(),
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
            Qa::new::<0, 2>(),
            Qa::new::<0, 3>(),
            Qa::new::<1, 2>(),
            Qa::new::<1, 3>(),
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
                if let Ok(qa) = qa + qr {
                    if !grid.is_blocked(&qa) {
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
            .any(|&qa| grid.is_blocked(&(qa + Qr::S).unwrap()))
    }
}

#[derive(Debug)]
pub struct Grid {
    s: Vec<u8>,
    ymin: autofolder::Min<u16>,
    ymax: autofolder::Max<u16>,
}

impl Grid {
    fn qa2idx(&self, qa: &Qa) -> Option<usize> {
        let t = qa.tuple();
        if self.s.len() > t.1 as usize {
            Some(self.s.len() - t.1 as usize - 1)
        } else {
            None
        }
    }
    fn qa2mask(&self, qa: &Qa) -> u8 {
        let t = qa.tuple();
        0x80 >> (t.0 as u8)
    }
    pub fn is_blocked(&self, qa: &Qa) -> bool {
        if let Some(i) = self.qa2idx(qa) {
            let c = self.s[i];
            let mask = self.qa2mask(qa);
            (c & mask) != 0
        } else {
            false
        }
    }
    pub fn add_block(&mut self, qa: &Qa) {
        self.ymax.eval(qa.tuple().1);
        self.ymin.eval(qa.tuple().1);
        if let Some(i) = self.qa2idx(qa) {
            let mask = self.qa2mask(qa);
            let c = &mut self.s[i];
            *c |= mask;
        }
    }
    pub fn height(&self) -> u64 {
        let zeroes = self.s.iter().rev().take_while(|&c| *c == 0).count();
        (self.s.len() - zeroes - 1) as u64
    }
    pub fn eval(&mut self) {
        while self.s[self.s.len() - 7] != 0 {
            self.s.push(0);
        }
        self.ymin = autofolder::Min::<u16>::new(GRIDTOP);
    }
    pub fn get_top<const L: usize>(&self) -> [u8; L] {
        let len = self.s.len();
        let mut top = [0_u8; L];
        top.copy_from_slice(&self.s[len - 7 - L..len - 7]);
        top
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            s: vec![0xFF, 0, 0, 0, 0, 0, 0, 0],
            ymin: autofolder::Min::<u16>::new(GRIDTOP),
            ymax: autofolder::Max::<u16>::new(0),
        }
    }
}

pub fn process(turns: u64, bufin: impl BufRead) -> Result<u64> {
    let jets = parser::parse(bufin)?;
    let mut grid = Grid::default();
    let mut jetiter = jets.iter().cycle();
    let mut turn = 0_u64;
    let mut height_offset = 0_u64;
    let mut cache = HashMap::<([u8; 23], u64), (u64, u64)>::default();
    let mut found = false;
    while turn < turns {
        // Put current state in cache:
        if grid.height() > 23 && !found {
            let toprows = grid.get_top::<23>();
            let key = (toprows, turn % (jets.len() as u64 * 5));
            if let Some((oldturn, oldheight)) = cache.get(&key) {
                let turn_incr = turn - oldturn;
                let height_incr = grid.height() - oldheight;
                let turns_left = turns - turn;
                let num_incr = turns_left / turn_incr;
                turn += num_incr * turn_incr;
                height_offset += num_incr * height_incr;
                found = true;
            } else {
                cache.insert(key, (turn, grid.height()));
            }
        }
        // Drop next shape:
        let ishape = turn % 5;
        let mut shape = match ishape {
            0 => Shape::new_dash(),
            1 => Shape::new_plus(),
            2 => Shape::new_l(),
            3 => Shape::new_i(),
            4 => Shape::new_o(),
            _ => panic!(),
        };
        let spawner = Qa::try_from((2, 0)).unwrap();
        shape.place(spawner);
        loop {
            let &jet = jetiter.next().unwrap();
            shape.mv(&grid, jet);
            if shape.rested(&grid) {
                for qa in shape.0.iter() {
                    grid.add_block(qa);
                }
                break;
            } else {
                shape.mv(&grid, Qr::S);
            }
        }
        grid.eval();
        turn += 1;
    }
    let height = grid.height() + height_offset;
    Ok(height)
}

pub type SqridD = sqrid::sqrid_create!(7, 30, false);
pub type QaD = sqrid::qa_create!(SqridD);
pub type GridD = sqrid::grid_create!(SqridD, char);

pub fn debug(grid: &Grid, shape0: Option<&Shape>) {
    let mut g = GridD::repeat('.');
    for qad in QaD::iter() {
        let qa = Qa::try_from(qad.tuple()).unwrap();
        if grid.is_blocked(&qa) {
            g[qad] = '#';
        }
    }
    if let Some(shape) = shape0 {
        for rock in &shape.0 {
            if let Ok(qad) = QaD::try_from(rock.tuple()) {
                g[qad] = '@';
            }
        }
    }
    eprintln!("{}", g);
}
