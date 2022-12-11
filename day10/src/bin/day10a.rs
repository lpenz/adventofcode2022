// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day10::*;

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let mut state = State::default();
    let mut value = 0;
    for instr in input {
        state.load(instr);
        while state.due != 0 {
            if state.cycle == 20
                || state.cycle == 60
                || state.cycle == 100
                || state.cycle == 140
                || state.cycle == 180
                || state.cycle == 220
            {
                value += state.cycle as i32 * state.x;
            }
            state.tick();
        }
    }
    Ok(value)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 13140);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
