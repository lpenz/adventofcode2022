// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use sqrid;

pub const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

pub type Sqrid = sqrid::sqrid_create!(120, 27, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Qr = sqrid::Qr;

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Bliz(Qr),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            '^' => Cell::Bliz(Qr::N),
            '>' => Cell::Bliz(Qr::E),
            'v' => Cell::Bliz(Qr::S),
            '<' => Cell::Bliz(Qr::W),
            _ => panic!("could not convert {} to Play", c),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(".#^>v<")(input)?;
        Ok((input, Cell::from(c)))
    }

    fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, l) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, l))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Cell>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let parsed = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(parsed.len(), 6);
    assert_eq!(parsed[0].len(), 8);
    Ok(())
}
