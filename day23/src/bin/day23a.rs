// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use sqrid::Qr;

use day23::*;

// pub type Sqrid = sqrid::sqrid_create!(12, 12, false); // example1
// pub type Sqrid = sqrid::sqrid_create!(14, 12, false); // example
pub type Sqrid = sqrid::sqrid_create!(100, 100, false);
pub type Qa = sqrid::qa_create!(Sqrid);

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
        debug(&elves);
        // First half:
        let mut count = HashMap::<Qa, i32>::default();
        let mut active = HashMap::<Qa, Qa>::new();
        for elf in &elves {
            for qr in Qr::iter::<true>() {
                if let Ok(look) = *elf + qr {
                    if elves.contains(&look) {
                        if let Some(mv_qr) = moves.iter().copied().find(|qr| {
                            let mut qr = *qr + Qr::NW;
                            !(0..3).into_iter().any(|_| {
                                let look = (*elf + qr).unwrap();
                                qr += Qr::NE;
                                elves.contains(&look)
                            })
                        }) {
                            // elf is active, will move to dst
                            let dst = (*elf + mv_qr)?;
                            active.insert(*elf, dst);
                            *count.entry(dst).or_insert(0) += 1;
                            break;
                        }
                    }
                }
            }
        }
        // Second half, moves:
        for (elf, dst) in active {
            if count.get(&dst) == Some(&1) {
                elves.remove(&elf);
                elves.insert(dst);
            }
        }
        let qr = moves.remove(0);
        moves.push(qr);
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

pub type GridD = sqrid::grid_create!(Sqrid, char);

pub fn debug(elves: &HashSet<Qa>) {
    let mut g = GridD::repeat('.');
    for e in elves {
        g[e] = '#';
    }
    eprintln!("{}", g);
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
