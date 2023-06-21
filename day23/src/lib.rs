// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, eyre::Error, Result};
use sqrid::Qr;
use std::collections::HashMap;
use std::collections::HashSet;

pub const EXAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

pub const EXAMPLE1: &str = ".....
..##.
..#..
.....
..##.
.....
";

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Elf,
}

impl TryFrom<char> for Cell {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Elf),
            _ => Err(eyre!("could not parse tile {}", c)),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        combinator::map_res(character::one_of(".#"), |c| c.try_into())(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cells))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Cell>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 7);
    Ok(())
}

// Solving:

// pub type Sqrid = sqrid::sqrid_create!(12, 12, false); // example1
// pub type Sqrid = sqrid::sqrid_create!(14, 12, false); // example
pub type Sqrid = sqrid::sqrid_create!(u16::MAX, u16::MAX, false);
pub type Qa = sqrid::qa_create!(Sqrid);

pub fn evaluate_round(elves: &mut HashSet<Qa>, moves: &mut Vec<Qr>) -> Result<bool> {
    // First half:
    let mut count = HashMap::<Qa, i32>::default();
    let mut active = HashMap::<Qa, Qa>::new();
    let mut moved = false;
    for elf in elves.iter() {
        for qr in Qr::iter::<true>() {
            if let Ok(look) = *elf + qr {
                if elves.contains(&look) {
                    if let Some(mv_qr) = moves.iter().copied().find(|qr| {
                        let mut qr = *qr + Qr::NW;
                        !(0..3).any(|_| {
                            let look = (*elf + qr).unwrap();
                            qr += Qr::NE;
                            elves.contains(&look)
                        })
                    }) {
                        // elf is active, will move to dst
                        let dst = (*elf + mv_qr)?;
                        active.insert(*elf, dst);
                        *count.entry(dst).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }
    }
    // Second half, moves:
    for (elf, dst) in active {
        if count.get(&dst) == Some(&1) {
            elves.remove(&elf);
            elves.insert(dst);
            moved = true;
        }
    }
    let qr = moves.remove(0);
    moves.push(qr);
    Ok(moved)
}

pub type GridD = sqrid::grid_create!(Sqrid, char);

pub fn debug(elves: &HashSet<Qa>) {
    let mut g = GridD::repeat('.');
    for e in elves {
        g[e] = '#';
    }
    eprintln!("{}", g);
}
