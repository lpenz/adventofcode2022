// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

// Play //

#[derive(PartialEq, Eq, Debug, Clone, Copy, num_derive::FromPrimitive)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn score(&self) -> u32 {
        *self as u32 + 1
    }
}

impl From<char> for Play {
    fn from(c: char) -> Self {
        match c {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            _ => panic!("could not convert {} to Play", c),
        }
    }
}

pub const ALLPLAYS: [Play; 3] = [Play::Rock, Play::Paper, Play::Scissors];

// Strat //

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Strat {
    X,
    Y,
    Z,
}

impl From<char> for Strat {
    fn from(c: char) -> Self {
        match c {
            'X' => Strat::X,
            'Y' => Strat::Y,
            'Z' => Strat::Z,
            _ => panic!("could not convert {} to Strat", c),
        }
    }
}

// Round score calculation //

pub fn round_beat_score(p1: Play, p2: Play) -> u32 {
    if p1 == p2 {
        3
    } else if p1 as u32 % 3 == (p2 as u32 + 1) % 3 {
        6
    } else {
        0
    }
}

#[test]
fn test_round_beat_score() {
    for play in ALLPLAYS {
        assert_eq!(round_beat_score(play, play), 3);
    }
    assert_eq!(round_beat_score(Play::Paper, Play::Rock), 6);
    assert_eq!(round_beat_score(Play::Rock, Play::Scissors), 6);
    assert_eq!(round_beat_score(Play::Scissors, Play::Paper), 6);
    assert_eq!(round_beat_score(Play::Rock, Play::Paper), 0);
    assert_eq!(round_beat_score(Play::Scissors, Play::Rock), 0);
    assert_eq!(round_beat_score(Play::Paper, Play::Scissors), 0);
}

// Parsing //

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn play(input: &str) -> IResult<&str, Play> {
        let (input, c) = character::one_of("ABC")(input)?;
        Ok((input, Play::from(c)))
    }

    fn strat(input: &str) -> IResult<&str, Strat> {
        let (input, c) = character::one_of("XYZ")(input)?;
        Ok((input, Strat::from(c)))
    }

    fn entry(input: &str) -> IResult<&str, (Play, Strat)> {
        let (input, play) = play(input)?;
        let (input, _) = character::space1(input)?;
        let (input, strat) = strat(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (play, strat)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Play, Strat)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(entry))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

pub const EXAMPLE: &str = "A Y
B X
C Z
";

#[test]
fn test() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        &[
            (Play::Rock, Strat::Y),
            (Play::Paper, Strat::X),
            (Play::Scissors, Strat::Z),
        ]
    );
    Ok(())
}
