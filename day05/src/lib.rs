// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::VecDeque;

#[cfg(test)]
use eyre::Result;

pub const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

pub type Crate = char;

#[derive(Debug)]
pub struct State(pub Vec<VecDeque<Crate>>);

impl State {
    pub fn from_lines(lines: Vec<Vec<Option<Crate>>>) -> Self {
        let numlines = lines.len();
        let numcols = lines[0].len();
        State(
            (0..numcols)
                .map(|col| {
                    (0..numlines)
                        .flat_map(|line| lines[line][col])
                        .rev()
                        .collect::<VecDeque<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub num: u32,
}

pub mod parser {
    use eyre::eyre;
    use eyre::Result;
    use nom::branch;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn cell_crate(input: &str) -> IResult<&str, Option<Crate>> {
        let (input, _) = character::char('[')(input)?;
        let (input, c) = character::satisfy(|c| c.is_ascii_uppercase())(input)?;
        let (input, _) = character::char(']')(input)?;
        Ok((input, Some(c)))
    }

    fn cell_empty(input: &str) -> IResult<&str, Option<Crate>> {
        let (input, _) = bytes::tag("   ")(input)?;
        Ok((input, None))
    }

    fn cell(input: &str) -> IResult<&str, Option<Crate>> {
        branch::alt((cell_crate, cell_empty))(input)
    }

    fn cell_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
        let (input, line) = multi::separated_list0(bytes::tag(" "), cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, line))
    }

    fn mv(input: &str) -> IResult<&str, Move> {
        let (input, _) = bytes::tag("move ")(input)?;
        let (input, num) = character::u32(input)?;
        let (input, _) = bytes::tag(" from ")(input)?;
        let (input, from) = character::u32(input)?;
        let (input, _) = bytes::tag(" to ")(input)?;
        let (input, to) = character::u32(input)?;
        let (input, _) = character::newline(input)?;
        Ok((
            input,
            Move {
                from: from as usize - 1,
                to: to as usize - 1,
                num,
            },
        ))
    }

    fn all(input: &str) -> IResult<&str, (State, Vec<Move>)> {
        let (input, cell_lines) = multi::many1(cell_line)(input)?;
        let (input, _) = multi::many1(character::one_of("0123456789 "))(input)?;
        let (input, _) = character::newline(input)?;
        let (input, _) = character::newline(input)?;
        let (input, moves) = multi::many1(mv)(input)?;
        Ok((input, (State::from_lines(cell_lines), moves)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(State, Vec<Move>)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    let (state, moves) = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(state.0.len(), 3);
    assert_eq!(moves.len(), 4);
    Ok(())
}
