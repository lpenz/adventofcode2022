// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
pub use sqrid::Qr;

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
