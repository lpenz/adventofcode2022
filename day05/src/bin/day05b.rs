// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

use day05::*;

pub fn do_move(state: &mut State, mv: &Move) {
    let mut crane = VecDeque::new();
    for _ in 0..mv.num {
        let c = state.0[mv.from].pop_back().unwrap();
        crane.push_front(c);
    }
    state.0[mv.to].append(&mut crane);
}

fn process(bufin: impl BufRead) -> Result<String> {
    let (mut state, moves) = parser::parse(bufin)?;
    for m in &moves {
        do_move(&mut state, m);
    }
    Ok(state
        .0
        .into_iter()
        .flat_map(|mut stack| stack.pop_back())
        .collect::<String>())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, "MCD");
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
