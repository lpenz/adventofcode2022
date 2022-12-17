// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

use day15::*;

fn process<const YEVAL: i64>(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut cantbe = BTreeSet::<i64>::default();
    for (sensor, beacon) in &input {
        let beacon_dist = manhattan_distance(sensor, beacon);
        let yeval_dist = (YEVAL - sensor.1).abs();
        if yeval_dist > beacon_dist {
            continue;
        }
        for i in 0..=beacon_dist - yeval_dist {
            cantbe.insert(sensor.0 + i);
            cantbe.insert(sensor.0 - i);
        }
    }
    for (_, beacon) in &input {
        if beacon.1 == YEVAL {
            cantbe.remove(&beacon.0);
        }
    }
    Ok(cantbe.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process::<10>(EXAMPLE.as_bytes())?, 26);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process::<2000000>(stdin().lock())?);
    Ok(())
}
