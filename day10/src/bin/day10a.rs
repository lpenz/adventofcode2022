// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day10::*;

#[derive(Debug)]
pub struct State {
    x: i32,
    cycle: u32,
    current: Instr,
    due: u32,
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
    fn load(&mut self, instr: Instr) {
        assert_eq!(self.due, 0);
        self.due = instr.cost();
        self.current = instr;
    }
    fn tick(&mut self) {
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
