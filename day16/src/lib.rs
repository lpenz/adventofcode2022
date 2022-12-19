// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::eyre::eyre;
pub use color_eyre::Result;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::mem;

use copstr::Str;

pub const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ValveId(pub Str<2>);

impl fmt::Debug for ValveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ValveId({})", self.0)
    }
}

impl From<Vec<char>> for ValveId {
    fn from(vc: Vec<char>) -> Self {
        ValveId(Str::new_trunc(vc.into_iter().collect::<String>()))
    }
}

impl From<&str> for ValveId {
    fn from(s: &str) -> Self {
        ValveId(s.try_into().unwrap())
    }
}

#[derive(Debug)]
pub struct Valve {
    pub id: ValveId,
    pub flow: i32,
    pub to: Vec<ValveId>,
}

impl Valve {
    pub fn new(id: ValveId, flow: i32, to: Vec<ValveId>) -> Valve {
        Valve { id, flow, to }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn valve(input: &str) -> IResult<&str, ValveId> {
        let (input, vstr) = multi::count(character::satisfy(|c| c.is_ascii_uppercase()), 2)(input)?;
        Ok((input, vstr.into()))
    }

    fn line(input: &str) -> IResult<&str, Valve> {
        let (input, _) = bytes::tag("Valve ")(input)?;
        let (input, id) = valve(input)?;
        let (input, _) = bytes::tag(" has flow rate=")(input)?;
        let (input, flow) = character::i32(input)?;
        let (input, _) = branch::alt((
            bytes::tag("; tunnels lead to valves "),
            bytes::tag("; tunnel leads to valve "),
        ))(input)?;
        let (input, to) = multi::separated_list1(bytes::tag(", "), valve)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Valve::new(id, flow, to)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Valve>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 10);
    Ok(())
}

pub fn move_costs_calc(
    valves: &HashMap<ValveId, Valve>,
) -> Result<HashMap<(ValveId, ValveId), i32>> {
    let mut costs = HashMap::<(ValveId, ValveId), i32>::new();
    for (&vid0, valve0) in valves.iter() {
        let mut nextfront = valve0.to.clone();
        let mut cost = 1;
        let mut visited = HashSet::<ValveId>::new();
        while !nextfront.is_empty() {
            let front = mem::take(&mut nextfront);
            for vid in front {
                if visited.contains(&vid) {
                    continue;
                }
                costs.insert((vid0, vid), cost);
                visited.insert(vid);
                nextfront.extend(valves.get(&vid).unwrap().to.iter());
            }
            cost += 1;
        }
    }
    Ok(costs)
}
