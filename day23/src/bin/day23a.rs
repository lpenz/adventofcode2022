// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

use sqrid::Qr;

use day23::*;

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let mut elves = HashSet::<Qa>::new();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            if cell == Cell::Elf {
                let qa = Qa::try_from((x as u16 + 20, y as u16 + 20))?;
                elves.insert(qa);
            }
        }
    }
    let mut moves = vec![Qr::N, Qr::S, Qr::W, Qr::E];
    for _round in 0..=10 {
        evaluate_round(&mut elves, &mut moves)?;
    }
    let (xmin, xmax) = elves
        .iter()
        .map(|qa| qa.tuple().0 as i32)
        .collect::<autofolder::MinMax<_>>()
        .into_inner_unwrap();
    let (ymin, ymax) = elves
        .iter()
        .map(|qa| qa.tuple().1 as i32)
        .collect::<autofolder::MinMax<_>>()
        .into_inner_unwrap();
    Ok((xmax - xmin + 1) * (ymax - ymin + 1) - elves.len() as i32)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 110);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
