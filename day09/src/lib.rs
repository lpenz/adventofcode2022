// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use eyre::Result;

// pub type Sqrid = sqrid::sqrid_create!(6, 5, true);
pub type Sqrid = sqrid::sqrid_create!(1000, 1000, true);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Qr = sqrid::qr::Qr;
pub type Grid = sqrid::grid_create!(Sqrid, char);
pub type Gridbool = sqrid::gridbool_create!(Sqrid);

#[derive(Debug)]
pub struct Mv {
    pub qr: Qr,
    pub dist: u32,
}

pub const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

pub mod parser {
    use eyre::eyre;
    use eyre::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn qr(input: &str) -> IResult<&str, Qr> {
        let (input, dir) = character::one_of("UDLR")(input)?;
        Ok((
            input,
            match dir {
                'U' => Ok(Qr::N),
                'R' => Ok(Qr::E),
                'D' => Ok(Qr::S),
                'L' => Ok(Qr::W),
                _ => panic!("could not convert direction"),
            }?,
        ))
    }

    fn mv(input: &str) -> IResult<&str, Mv> {
        let (input, qr) = qr(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, dist) = character::u32(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Mv { qr, dist }))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Mv>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(mv))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 8);
    Ok(())
}
