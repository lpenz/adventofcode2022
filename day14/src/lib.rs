// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::eyre;
use eyre::Result;
use std::cmp::Ordering::{Equal, Greater, Less};

pub use sqrid::Qr;

pub const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

pub type Sqrid = sqrid::sqrid_create!(1000, 500, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    #[default]
    Empty,
    Rock,
    Sand,
}

impl From<Cell> for char {
    fn from(cell: Cell) -> char {
        match cell {
            Cell::Empty => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn qa(input: &str) -> IResult<&str, Qa> {
        let (input, x) = character::u16(input)?;
        let (input, _) = bytes::tag(",")(input)?;
        let (input, y) = character::u16(input)?;
        let qa = Qa::try_from((x, y)).unwrap();
        Ok((input, qa))
    }

    fn path(input: &str) -> IResult<&str, Vec<Qa>> {
        let (input, qas) = multi::separated_list1(bytes::tag(" -> "), qa)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, qas))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Qa>>> {
        aoc::parse_with!(multi::many1(path), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 2);
    Ok(())
}

pub fn direction(src: &Qa, dst: &Qa) -> Option<Qr> {
    let tsrc = src.tuple();
    let tdst = dst.tuple();
    match (tsrc.0.cmp(&tdst.0), tsrc.1.cmp(&tdst.1)) {
        (Equal, Equal) => None,
        (Equal, Greater) => Some(Qr::N),
        (Less, Greater) => Some(Qr::NE),
        (Less, Equal) => Some(Qr::E),
        (Less, Less) => Some(Qr::SE),
        (Equal, Less) => Some(Qr::S),
        (Greater, Less) => Some(Qr::SW),
        (Greater, Equal) => Some(Qr::W),
        (Greater, Greater) => Some(Qr::NW),
    }
}

pub fn lay_rock(g: &mut Grid, src: &Qa, dst: &Qa) -> Result<()> {
    let mut qa = *src;
    g[qa] = Cell::Rock;
    while let Some(qr) = direction(&qa, dst) {
        qa = (qa + qr)?;
        g[qa] = Cell::Rock;
    }
    Ok(())
}

pub fn grid_from_paths(paths: Vec<Vec<Qa>>) -> Result<Box<Grid>> {
    let mut g = Box::<sqrid::Grid<Cell, 1000, 500, 500000>>::default();
    for path in paths {
        for (i, dst) in path.iter().enumerate().skip(1) {
            lay_rock(&mut g, &path[i - 1], dst)?;
        }
    }
    Ok(g)
}

pub fn grid_y_max_rock(g: &Grid) -> Result<u16> {
    g.iter_qa()
        .filter_map(|(qa, &cell)| {
            if cell == Cell::Rock {
                Some(qa.tuple().1)
            } else {
                None
            }
        })
        .max()
        .ok_or_else(|| eyre!("could not find max y"))
}

pub fn sand_fall(grid: &mut Grid, qa: &mut Qa) -> bool {
    if let Some(newqa) = [Qr::S, Qr::SW, Qr::SE]
        .iter()
        .filter_map(|qr| {
            (*qa + qr)
                .ok()
                .and_then(|qa| Some(qa).filter(|qa| grid[qa] == Cell::Empty))
        })
        .next()
    {
        grid[*qa] = Cell::Empty;
        *qa = newqa;
        grid[*qa] = Cell::Sand;
        true
    } else {
        false
    }
}
