// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

pub const EXAMPLE: &str = "0\n";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    // use nom::bytes::complete as bytes;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    pub fn num(input: &str) -> IResult<&str, u32> {
        character::u32(input)
    }

    pub fn line(input: &str) -> IResult<&str, u32> {
        let (input, num) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<u32>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?, &[0]);
    Ok(())
}
