// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use eyre::Result;

pub const EXAMPLE: &str = "noop
addx 3
addx -5
";

pub const EXAMPLE2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

#[derive(Debug, Default, Clone, Copy)]
pub enum Instr {
    #[default]
    Noop,
    Addx(i32),
}

impl Instr {
    pub fn cost(&self) -> u32 {
        match self {
            Instr::Noop => 1,
            Instr::Addx(_) => 2,
        }
    }
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

    fn instr_noop(input: &str) -> IResult<&str, Instr> {
        let (input, _) = bytes::tag("noop")(input)?;
        Ok((input, Instr::Noop))
    }

    fn instr_addx(input: &str) -> IResult<&str, Instr> {
        let (input, _) = bytes::tag("addx ")(input)?;
        let (input, value) = character::i32(input)?;
        Ok((input, Instr::Addx(value)))
    }

    fn instr(input: &str) -> IResult<&str, Instr> {
        let (input, instr) = branch::alt((instr_noop, instr_addx))(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, instr))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Instr>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(instr))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 3);
    Ok(())
}

// State and processing

#[derive(Debug)]
pub struct State {
    pub x: i32,
    pub cycle: u32,
    pub current: Instr,
    pub due: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            x: 1,
            cycle: 1,
            current: Default::default(),
            due: 0,
        }
    }
}

impl State {
    pub fn load(&mut self, instr: Instr) {
        assert_eq!(self.due, 0);
        self.due = instr.cost();
        self.current = instr;
    }
    pub fn tick(&mut self) {
        self.cycle += 1;
        self.due -= 1;
        if self.due == 0 {
            match self.current {
                Instr::Noop => {}
                Instr::Addx(v) => self.x += v,
            }
        }
    }
}
