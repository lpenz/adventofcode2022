// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use eyre::Result;

pub const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

pub type Xy = (i64, i64);

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn xy(input: &str) -> IResult<&str, Xy> {
        let (input, _) = bytes::tag("x=")(input)?;
        let (input, x) = character::i64(input)?;
        let (input, _) = bytes::tag(", y=")(input)?;
        let (input, y) = character::i64(input)?;
        Ok((input, (x, y)))
    }

    fn line(input: &str) -> IResult<&str, (Xy, Xy)> {
        let (input, _) = bytes::tag("Sensor at ")(input)?;
        let (input, sensor) = xy(input)?;
        let (input, _) = bytes::tag(": closest beacon is at ")(input)?;
        let (input, beacon) = xy(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (sensor, beacon)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Xy, Xy)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 14);
    Ok(())
}
