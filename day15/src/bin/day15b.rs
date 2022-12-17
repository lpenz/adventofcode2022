// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::io::{stdin, BufRead};

use day15::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    pub center: Xy,
    pub range: i64,
}

impl Sensor {
    pub fn new(center: Xy, range: i64) -> Sensor {
        Sensor { center, range }
    }
    pub fn covered(&self, xy: &(i64, i64)) -> bool {
        manhattan_distance(&self.center, xy) <= self.range
    }
    pub fn iter_outerlimits(&self) -> impl Iterator<Item = Xy> + '_ {
        (0..=self.range)
            .map(move |d| {
                // from N to E
                (self.center.0 + d, self.center.1 - self.range - 1 + d)
            })
            .chain((0..=self.range).map(move |d| {
                // from E to S
                (self.center.0 + self.range + 1 - d, self.center.1 + d)
            }))
            .chain((0..=self.range).map(move |d| {
                // from S to W
                (self.center.0 - d, self.center.1 + self.range + 1 - d)
            }))
            .chain((0..=self.range).map(move |d| {
                // from W to N
                (self.center.0 - self.range - 1 + d, self.center.1 - d)
            }))
    }
}

#[test]
fn test_iter_outerlimits() {
    let sensor = Sensor::new((0, 0), 0);
    let limits_exc = sensor.iter_outerlimits().collect::<Vec<_>>();
    for xy in &limits_exc {
        assert!(!sensor.covered(&xy));
        assert_eq!(manhattan_distance(&sensor.center, &xy), sensor.range + 1);
    }
    assert_eq!(limits_exc, vec![(0, -1), (1, 0), (0, 1), (-1, 0)]);
    let sensor = Sensor::new((0, 0), 1);
    let limits_exc = sensor.iter_outerlimits().collect::<Vec<_>>();
    for xy in &limits_exc {
        assert!(!sensor.covered(&xy));
        assert_eq!(manhattan_distance(&sensor.center, &xy), sensor.range + 1);
    }
    assert_eq!(
        limits_exc,
        vec![
            (0, -2),
            (1, -1),
            (2, 0),
            (1, 1),
            (0, 2),
            (-1, 1),
            (-2, 0),
            (-1, -1)
        ]
    );
}

impl From<((i64, i64), (i64, i64))> for Sensor {
    fn from(input: ((i64, i64), (i64, i64))) -> Self {
        Sensor::new(input.0, manhattan_distance(&input.0, &input.1))
    }
}

fn check<const MAX: i64>(sensors: &[Sensor], xy: &(i64, i64)) -> bool {
    0 <= xy.0 && xy.0 <= MAX && 0 <= xy.1 && xy.1 <= MAX && sensors.iter().all(|s| !s.covered(xy))
}

fn process<const MAX: i64>(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    let sensors = input.into_iter().map(Sensor::from).collect::<Vec<_>>();
    if let Some(xy) = sensors
        .iter()
        .flat_map(|s| s.iter_outerlimits())
        .find(|xy| check::<MAX>(&sensors, xy))
    {
        return Ok(xy.0 * 4000000 + xy.1);
    }
    Err(eyre!("could not find a solution"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process::<20>(EXAMPLE.as_bytes())?, 56000011);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process::<4000000>(stdin().lock())?);
    Ok(())
}
