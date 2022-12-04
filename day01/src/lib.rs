// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use eyre::Result;

pub type Calories = u32;

pub const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

pub mod parser {
    use eyre::eyre;
    use eyre::Result;
    use nom::character::complete as character;
    use nom::character::complete::newline;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn calories(input: &str) -> IResult<&str, Calories> {
        let (input, calories) = character::u32(input)?;
        let (input, _) = newline(input)?;
        Ok((input, calories))
    }

    fn elf(input: &str) -> IResult<&str, Vec<Calories>> {
        let (input, elf) = multi::many1(calories)(input)?;
        Ok((input, elf))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Calories>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::separated_list1(newline, elf))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        &[
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    );
    Ok(())
}
