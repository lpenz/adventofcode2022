// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

pub const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

pub type Section = u32;

pub type Assignment = (Section, Section);

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    // use nom::bytes::complete as bytes;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn section(input: &str) -> IResult<&str, u32> {
        character::u32(input)
    }

    fn assignment(input: &str) -> IResult<&str, Assignment> {
        let (input, min) = section(input)?;
        let (input, _) = character::char('-')(input)?;
        let (input, max) = section(input)?;
        Ok((input, (min, max)))
    }

    fn assnpair(input: &str) -> IResult<&str, (Assignment, Assignment)> {
        let (input, a1) = assignment(input)?;
        let (input, _) = character::char(',')(input)?;
        let (input, a2) = assignment(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (a1, a2)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Assignment, Assignment)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(assnpair))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        &[
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ]
    );
    Ok(())
}
