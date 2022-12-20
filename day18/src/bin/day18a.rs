// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day18::*;

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let cubes = input.into_iter().collect::<HashSet<Xyz>>();
    let mut area = 0;
    for xyz in &cubes {
        for neigh in xyz.iter_neighs() {
            if !cubes.contains(&neigh) {
                area += 1;
            }
        }
    }
    Ok(area)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 64);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
