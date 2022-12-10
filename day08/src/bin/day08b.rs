// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::eyre;
use eyre::Result;
use std::io::{stdin, BufRead};

use day08::*;

fn iter_score(input: &[Vec<i8>], height: i8, iter: impl Iterator<Item = (usize, usize)>) -> u32 {
    1 + iter.take_while(|&xy| input[xy.1][xy.0] < height).count() as u32
}

fn score<const SIDE: usize>(input: &[Vec<i8>], xy: (usize, usize)) -> u32 {
    if xy.0 == 0 || xy.0 == SIDE - 1 || xy.1 == 0 || xy.1 == SIDE - 1 {
        return 0;
    }
    let x = xy.0;
    let y = xy.1;
    let height = input[y][x];
    let score1 = iter_score(input, height, (x + 1..SIDE - 1).map(|x| (x, y)));
    let score2 = iter_score(input, height, (1..=x - 1).rev().map(|x| (x, y)));
    let score3 = iter_score(input, height, ((y + 1)..SIDE - 1).map(|y| (x, y)));
    let score4 = iter_score(input, height, (1..=(y - 1)).rev().map(|y| (x, y)));
    score1 * score2 * score3 * score4
}

fn process<const SIDE: usize>(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let visible = visible_trees::<SIDE>(&input)?;
    let maxscore = visible
        .into_iter()
        .map(|xy| score::<SIDE>(&input, xy))
        .max()
        .ok_or_else(|| eyre!("could not calculate max score"))?;
    Ok(maxscore)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process::<5>(EXAMPLE.as_bytes())?, 8);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process::<99>(stdin().lock())?);
    Ok(())
}
