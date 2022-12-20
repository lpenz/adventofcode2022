// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day18::*;

pub fn minmax_calc(cubes: &HashSet<Xyz>) -> (Xyz, Xyz) {
    (
        Xyz(
            cubes
                .iter()
                .map(|xyz| xyz.0)
                .collect::<autofolder::Min<_>>()
                .into_inner()
                .unwrap(),
            cubes
                .iter()
                .map(|xyz| xyz.1)
                .collect::<autofolder::Min<_>>()
                .into_inner()
                .unwrap(),
            cubes
                .iter()
                .map(|xyz| xyz.2)
                .collect::<autofolder::Min<_>>()
                .into_inner()
                .unwrap(),
        ),
        Xyz(
            cubes
                .iter()
                .map(|xyz| xyz.0)
                .collect::<autofolder::Max<_>>()
                .into_inner()
                .unwrap(),
            cubes
                .iter()
                .map(|xyz| xyz.1)
                .collect::<autofolder::Max<_>>()
                .into_inner()
                .unwrap(),
            cubes
                .iter()
                .map(|xyz| xyz.2)
                .collect::<autofolder::Max<_>>()
                .into_inner()
                .unwrap(),
        ),
    )
}

pub fn is_inside(minmax: &(Xyz, Xyz), current: Xyz) -> bool {
    current.0 >= minmax.0 .0
        && current.0 <= minmax.1 .0
        && current.1 >= minmax.0 .1
        && current.1 <= minmax.1 .1
        && current.2 >= minmax.0 .2
        && current.2 <= minmax.1 .2
}

pub fn dirs_escape(cubes: &HashSet<Xyz>, minmax: &(Xyz, Xyz), current: &Xyz) -> bool {
    for d in Xyz::iter_dirs() {
        let mut xyz = *current;
        while !cubes.contains(&xyz) {
            xyz = xyz + d;
            if !is_inside(minmax, xyz) {
                return true;
            }
        }
    }
    false
}

pub fn bf_escape(cubes: &HashSet<Xyz>, whitelist: &mut HashSet<Xyz>, current: &Xyz) -> bool {
    let mut nextfront = vec![current];
    let mut visited = HashSet::<Xyz>::new();
    visited.insert(*current);
    while !nextfront.is_empty() {
        let front = std::mem::take(&mut nextfront);
        for xyz in front {
            for neigh in xyz.iter_neighs() {
                if visited.contains(&neigh) {
                    continue;
                }
                if whitelist.contains(&neigh) {
                    return true;
                }
                if !cubes.contains(xyz) {
                    nextfront.push(xyz);
                }
                visited.insert(neigh);
            }
        }
    }
    false
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let cubes = input.into_iter().collect::<HashSet<Xyz>>();
    let minmax = minmax_calc(&cubes);
    // Join neighbors in a map with the area weight
    let values = cubes
        .iter()
        .fold(HashMap::<Xyz, i32>::new(), |mut values, &current| {
            for d in Xyz::iter_dirs() {
                let xyz = current + d;
                if !cubes.contains(&xyz) {
                    values.entry(xyz).and_modify(|v| *v += 1).or_insert(1);
                }
            }
            values
        });
    let mut area = 0;
    let mut spare = HashSet::<Xyz>::new();
    let mut whitelist = HashSet::<Xyz>::new();
    // First pass: simple directions
    for (xyz, value) in &values {
        if dirs_escape(&cubes, &minmax, xyz) {
            area += value;
            whitelist.insert(*xyz);
        } else {
            spare.insert(*xyz);
        }
    }
    // Second pass: BFS what is left
    for xyz in &spare {
        if bf_escape(&cubes, &mut whitelist, xyz) {
            area += values.get(xyz).unwrap();
        }
    }
    Ok(area)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 58);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
