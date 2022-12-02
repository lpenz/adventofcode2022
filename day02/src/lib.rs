// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for Play {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => panic!("could not convert {} to Play", c),
        }
    }
}

pub const ALLPLAYS: [Play; 3] = [Play::Rock, Play::Paper, Play::Scissors];

pub const EXAMPLE: &str = "A Y
B X
C Z
";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    pub fn play(input: &str) -> IResult<&str, Play> {
        let (input, c) = character::one_of("ABCXYZ")(input)?;
        Ok((input, Play::from(c)))
    }

    pub fn entry(input: &str) -> IResult<&str, (Play, Play)> {
        let (input, play1) = play(input)?;
        let (input, _) = character::space1(input)?;
        let (input, play2) = play(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (play1, play2)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Play, Play)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(entry))(&input);
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
            (Play::Rock, Play::Paper),
            (Play::Paper, Play::Rock),
            (Play::Scissors, Play::Scissors),
        ]
    );
    Ok(())
}
