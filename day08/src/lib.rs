// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use eyre::Result;

pub const EXAMPLE: &str = "30373
25512
65332
33549
35390
";

pub mod parser {
    use eyre::eyre;
    use eyre::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    fn tree(input: &str) -> IResult<&str, i8> {
        combinator::map_res(character::satisfy(|c| c.is_ascii_digit()), |d| {
            d.to_digit(10)
                .ok_or_else(|| eyre!("could not parse tree {}", d))
                .map(|d| d as i8)
        })(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<i8>> {
        let (input, line) = multi::many1(tree)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, line))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<i8>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(line))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 5);
    Ok(())
}
