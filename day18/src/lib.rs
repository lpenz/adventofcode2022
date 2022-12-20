// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use std::ops;

pub const EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Xyz(pub i32, pub i32, pub i32);

impl Xyz {
    pub fn iter_dirs() -> impl Iterator<Item = Xyz> {
        vec![
            Xyz(1, 0, 0),
            Xyz(0, 1, 0),
            Xyz(0, 0, 1),
            Xyz(-1, 0, 0),
            Xyz(0, -1, 0),
            Xyz(0, 0, -1),
        ]
        .into_iter()
    }
    pub fn iter_neighs(&self) -> impl Iterator<Item = Xyz> + '_ {
        Xyz::iter_dirs().map(|d| *self + d)
    }
}

impl ops::Add for Xyz {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Xyz(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn xyz(input: &str) -> IResult<&str, Xyz> {
        let (input, x) = character::i32(input)?;
        let (input, _) = bytes::tag(",")(input)?;
        let (input, y) = character::i32(input)?;
        let (input, _) = bytes::tag(",")(input)?;
        let (input, z) = character::i32(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Xyz(x, y, z)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Xyz>> {
        aoc::parse_with!(multi::many1(xyz), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 13);
    Ok(())
}
