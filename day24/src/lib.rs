// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use std::collections::HashMap;
use std::collections::HashSet;

pub const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

pub type Sqrid = sqrid::sqrid_create!(122, 27, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub type GridDebug = sqrid::grid_create!(Sqrid, char);
pub type Gridbool = sqrid::gridbool_create!(Sqrid);
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

pub type Turn = i32;

#[derive(Default, Debug, Clone)]
pub struct Bliz {
    pub id: usize,
    pub qa0: Qa,
    pub qr: Qr,
}

impl Bliz {
    pub fn qa_at(&self, botright: &Qa, turn: Turn) -> Result<Qa> {
        let t = self.qa0.tuple();
        let x = t.0 as Turn;
        let y = t.1 as Turn;
        let tmax = botright.tuple();
        let (max, pos) = if self.qr.is_horizontal() {
            (tmax.0 as Turn - 1, x - 1)
        } else {
            (tmax.1 as Turn - 1, y - 1)
        };
        let turn = turn % max;
        let newpos = if self.qr.is_positive() {
            ((pos + turn) % max) + 1
        } else {
            ((pos + 2 * max - turn) % max) + 1
        };
        Qa::try_from(match self.qr {
            Qr::N => (x, newpos),
            Qr::E => (newpos, y),
            Qr::S => (x, newpos),
            Qr::W => (newpos, y),
            _ => panic!("invalid direction"),
        })
        .map_err(|e| eyre!(e))
    }
}

#[test]
fn test_bliz() -> Result<()> {
    let botright = Qa::new(3, 3)?;
    let bliz = |x, y, qr| Bliz {
        id: 0,
        qa0: Qa::new(x, y).unwrap(),
        qr: qr,
    };
    // Direct
    assert_eq!(bliz(1, 1, Qr::E).qa_at(&botright, 1)?, Qa::new(2, 1)?);
    assert_eq!(bliz(2, 1, Qr::W).qa_at(&botright, 1)?, Qa::new(1, 1)?);
    assert_eq!(bliz(1, 2, Qr::N).qa_at(&botright, 1)?, Qa::new(1, 1)?);
    assert_eq!(bliz(1, 1, Qr::S).qa_at(&botright, 1)?, Qa::new(1, 2)?);
    // Wrap
    assert_eq!(bliz(1, 1, Qr::W).qa_at(&botright, 1)?, Qa::new(2, 1)?);
    assert_eq!(bliz(1, 1, Qr::N).qa_at(&botright, 1)?, Qa::new(1, 2)?);
    assert_eq!(bliz(2, 1, Qr::E).qa_at(&botright, 1)?, Qa::new(1, 1)?);
    assert_eq!(bliz(1, 2, Qr::S).qa_at(&botright, 1)?, Qa::new(1, 1)?);
    Ok(())
}

impl From<(usize, Qa, Qr)> for Bliz {
    fn from(t: (usize, Qa, Qr)) -> Self {
        Bliz {
            id: t.0,
            qa0: t.1,
            qr: t.2,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Params {
    pub botright: Qa,
    pub blizs: Vec<Bliz>,
    pub walls: HashSet<Qa>,
    pub cache: HashMap<(Turn, Qa), bool>,
    pub start: Qa,
    pub target: Qa,
}

impl Params {
    pub fn new(input: Vec<Vec<Cell>>) -> Result<Params> {
        let mut params = Params::default();
        let mut bliz_id = 0;
        for (y, line) in input.into_iter().enumerate() {
            for (x, cell) in line.into_iter().enumerate() {
                let qa = Qa::new(x as u16, y as u16)?;
                if cell == Cell::Wall {
                    params.walls.insert(qa);
                }
                if y == 0 && cell == Cell::Empty {
                    params.start = qa;
                } else if y != 0 && cell == Cell::Empty {
                    params.target = qa;
                } else if cell == Cell::Wall {
                    params.botright = qa;
                }
                if let Cell::Bliz(qr) = cell {
                    params.blizs.push((bliz_id, qa, qr).into());
                    bliz_id += 1;
                }
            }
        }
        Ok(params)
    }

    pub fn any_at(&mut self, turn: Turn, qa: Qa) -> bool {
        if let Some(cached) = self.cache.get(&(turn, qa)) {
            return *cached;
        }
        let result = self
            .blizs
            .iter()
            .filter(|b| {
                ((b.qr == Qr::N || b.qr == Qr::S) && b.qa0.tuple().0 == qa.tuple().0)
                    || ((b.qr == Qr::E || b.qr == Qr::W) && b.qa0.tuple().1 == qa.tuple().1)
            })
            .any(|b| b.qa_at(&self.botright, turn).ok() == Some(qa));
        self.cache.insert((turn, qa), result);
        result
    }

    pub fn iter(&mut self, turn: Turn, me: Qa) -> impl Iterator<Item = (Qa, Option<Qr>)> + '_ {
        let mut v = vec![];
        if !self.any_at(turn, me) {
            v.push((me, None));
        }
        v.extend(Qr::iter::<false>().filter_map(move |qr| {
            if let Ok(qa) = me + qr {
                if self.walls.contains(&qa) {
                    None
                } else if !self.any_at(turn, qa) {
                    Some((qa, Some(qr)))
                } else {
                    None
                }
            } else {
                None
            }
        }));
        v.into_iter()
    }

    pub fn debug(&self, turn: Turn, me: Qa) {
        let blizs = self
            .blizs
            .iter()
            .map(|b| {
                (
                    b.qa_at(&self.botright, turn).unwrap(),
                    match b.qr {
                        Qr::N => '^',
                        Qr::E => '>',
                        Qr::S => 'v',
                        Qr::W => '<',
                        _ => panic!("invalid qr"),
                    },
                )
            })
            .collect::<HashMap<_, _>>();
        let g = Qa::iter()
            .map(|qa| {
                if self.walls.contains(&qa) {
                    '#'
                } else if let Some(b) = blizs.get(&qa) {
                    *b
                } else if qa == me {
                    '@'
                } else {
                    ' '
                }
            })
            .collect::<GridDebug>();
        eprintln!("{}", g);
    }

    pub fn bfs(&mut self, mut turn: Turn, start: Qa, target: Qa) -> Result<Turn> {
        let mut nextfront = HashSet::<Qa>::default();
        nextfront.insert(start);
        let mut found = false;
        while !found {
            let front = std::mem::take(&mut nextfront);
            turn += 1;
            for me in &front {
                if *me == target {
                    found = true;
                    break;
                }
                for (qa, _qropt) in self.iter(turn, *me) {
                    nextfront.insert(qa);
                }
            }
        }
        Ok(turn - 1)
    }
}
