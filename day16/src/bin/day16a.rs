// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

use day16::*;

fn dfs(
    valves: &HashMap<ValveId, Valve>,
    costs: &HashMap<(ValveId, ValveId), i32>,
    current: ValveId,
    closed0: &HashSet<ValveId>,
    mut left: i32,
) -> i32 {
    if left <= 1 || closed0.is_empty() {
        return 0;
    }
    // Open self:
    let mut closed = closed0.clone();
    closed.remove(&current);
    let valve = valves.get(&current).unwrap();
    left -= i32::from(valve.flow > 0);
    let flow = valve.flow * left;
    // Go to the next valves:
    let mut best = autofolder::Max::new(flow);
    for &next in &closed {
        let cost = *costs.get(&(current, next)).unwrap();
        if cost > left {
            continue;
        }
        best.reduce(flow + dfs(valves, costs, next, &closed, left - cost));
    }
    best.into_inner().unwrap()
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let initial = ValveId::from("AA");
    let valves = input
        .into_iter()
        .map(|v| (v.id, v))
        .collect::<HashMap<ValveId, Valve>>();
    let costs = move_costs_calc(&valves)?;
    let closed = valves
        .iter()
        .filter_map(|(&vid, v)| Some(vid).filter(|_| v.flow > 0))
        .collect::<HashSet<_>>();
    Ok(dfs(&valves, &costs, initial, &closed, 30))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 1651);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
