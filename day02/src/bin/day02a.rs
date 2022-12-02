// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

use day02::*;

fn strat2play(strat: Strat) -> Play {
    match strat {
        Strat::X => Play::Rock,
        Strat::Y => Play::Paper,
        Strat::Z => Play::Scissors,
    }
}

fn round_score(round: &(Play, Strat)) -> u32 {
    let p1 = round.0;
    let p2 = strat2play(round.1);
    p2.score() + round_beat_score(p2, p1)
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    Ok(input.iter().map(round_score).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 15);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
