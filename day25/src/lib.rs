// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};

pub const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

pub type Snafu = String;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn snafudigit(input: &str) -> IResult<&str, char> {
        let (input, digit) = character::one_of("210-=")(input)?;
        Ok((input, digit))
    }

    fn line(input: &str) -> IResult<&str, Snafu> {
        let (input, digits) = multi::many1(snafudigit)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, digits.into_iter().collect()))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Snafu>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 13);
    Ok(())
}
