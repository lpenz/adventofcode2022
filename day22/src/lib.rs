// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, eyre::Error, Result};
pub use sqrid::Qr;
use std::fmt;

pub const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

// pub type Sqrid = sqrid::sqrid_create!(16, 12, false);
pub type Sqrid = sqrid::sqrid_create!(150, 200, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    #[default]
    Blank,
    Open,
    Wall,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<&Cell> for char {
    fn from(cell: &Cell) -> char {
        match cell {
            Cell::Blank => ' ',
            Cell::Open => '.',
            Cell::Wall => '#',
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Cell::Blank),
            '.' => Ok(Cell::Open),
            '#' => Ok(Cell::Wall),
            _ => Err(eyre!("could not parse tile {}", c)),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Walk(i32),
    Turn(Qr),
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        combinator::map_res(character::one_of(" .#"), |c| c.try_into())(input)
    }

    fn board_line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, line) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, line))
    }

    fn instr_walk(input: &str) -> IResult<&str, Instr> {
        let (input, num) = character::i32(input)?;
        Ok((input, Instr::Walk(num)))
    }

    fn instr_turn(input: &str) -> IResult<&str, Instr> {
        let (input, lr) = character::one_of("LR")(input)?;
        let qr = match lr {
            'L' => Qr::W,
            'R' => Qr::E,
            _ => panic!("unexpected char {}", lr),
        };
        Ok((input, Instr::Turn(qr)))
    }

    fn instr(input: &str) -> IResult<&str, Instr> {
        let (input, instr) = branch::alt((instr_walk, instr_turn))(input)?;
        Ok((input, instr))
    }

    fn all(input: &str) -> IResult<&str, (Vec<Vec<Cell>>, Vec<Instr>)> {
        let (input, board) = multi::many1(board_line)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, path) = multi::many1(instr)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (board, path)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Vec<Cell>>, Vec<Instr>)> {
        aoc::parse_with!(all, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.0.len(), 12);
    assert_eq!(input.1.len(), 13);
    Ok(())
}
