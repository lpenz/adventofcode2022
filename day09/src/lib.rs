// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::HashSet;

use sqrid::qaqr::qaqr_resolve;

// pub type Sqrid = sqrid::sqrid_create!(6, 5, true);
// pub type Sqrid = sqrid::sqrid_create!(26, 21, true);
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

pub const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
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
    assert_eq!(parser::parse(EXAMPLE2.as_bytes())?.len(), 8);
    Ok(())
}

pub fn process_moves<const KNOTS: usize>(input: &[Mv]) -> Result<usize> {
    let mut knots = [Qa::try_from((Sqrid::WIDTH / 2, Sqrid::HEIGHT / 2))?; KNOTS];
    let mut visited = HashSet::<Qa>::default();
    visited.insert(knots[0]);
    for mv in input {
        for _ in 0..mv.dist {
            knots[0] = qaqr_resolve(knots[0], mv.qr)?;
            for i in 1..KNOTS {
                if knots[i - 1] != knots[i]
                    && !Qr::iter::<true>().any(|d| knots[i - 1] + d == Ok(knots[i]))
                {
                    let target = knots[i - 1].tuple();
                    let me = knots[i].tuple();
                    // Rules:
                    let qr = if me.0 == target.0 {
                        if me.1 > target.1 {
                            Qr::N
                        } else {
                            Qr::S
                        }
                    } else if me.1 == target.1 {
                        if me.0 > target.0 {
                            Qr::W
                        } else {
                            Qr::E
                        }
                    } else if me.0 < target.0 {
                        if me.1 < target.1 {
                            Qr::SE
                        } else {
                            Qr::NE
                        }
                    } else if me.1 < target.1 {
                        Qr::SW
                    } else {
                        Qr::NW
                    };
                    knots[i] = (knots[i] + qr).unwrap();
                }
                visited.insert(knots[KNOTS - 1]);
            }
        }
    }
    Ok(visited.len())
}
