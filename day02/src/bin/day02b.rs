// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use num_traits::cast::FromPrimitive;
use std::io::{stdin, BufRead};

use day02::*;

pub fn myplay_calc(strat: Strat, hisplay: Play) -> Play {
    let hisval = hisplay as u32;
    Play::from_u32(match strat {
        Strat::X => (hisval + 2) % 3, // lose
        Strat::Y => hisval,           // draw
        Strat::Z => (hisval + 1) % 3, // win
    })
    .unwrap()
}

#[test]
fn test_myplay_calc() {
    assert_eq!(myplay_calc(Strat::Y, Play::Rock), Play::Rock);
    assert_eq!(myplay_calc(Strat::X, Play::Paper), Play::Rock);
    assert_eq!(myplay_calc(Strat::Z, Play::Scissors), Play::Rock);
}

fn eval(entry: (Play, Strat)) -> u32 {
    let hisplay = entry.0;
    let strat = entry.1;
    let myplay = myplay_calc(strat, hisplay);
    myplay.score() + round_beat_score(myplay, hisplay)
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    Ok(input.into_iter().map(eval).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 12);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
