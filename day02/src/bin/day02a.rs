// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

use day02::*;

fn round_beat_score(p1: Play, p2: Play) -> u32 {
    if p1 == p2 {
        3
    } else if p1 as u32 % 3 == (p2 as u32 + 1) % 3 {
        6
    } else {
        0
    }
}

#[test]
fn test_round_score() {
    for play in ALLPLAYS {
        assert_eq!(round_beat_score(play, play), 3);
    }
    assert_eq!(round_beat_score(Play::Paper, Play::Rock), 6);
    assert_eq!(round_beat_score(Play::Rock, Play::Scissors), 6);
    assert_eq!(round_beat_score(Play::Scissors, Play::Paper), 6);
    assert_eq!(round_beat_score(Play::Rock, Play::Paper), 0);
    assert_eq!(round_beat_score(Play::Scissors, Play::Rock), 0);
    assert_eq!(round_beat_score(Play::Paper, Play::Scissors), 0);
}

fn round_score(round: &(Play, Play)) -> u32 {
    round.1 as u32 + round_beat_score(round.1, round.0)
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
