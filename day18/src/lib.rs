// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};

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

pub type Xyz = (i32, i32, i32);

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
        Ok((input, (x, y, z)))
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
