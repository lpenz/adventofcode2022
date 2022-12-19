// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

use day16::*;

fn all_paths(
    valves: &HashMap<ValveId, Valve>,
    costs: &HashMap<(ValveId, ValveId), i32>,
    current: ValveId,
    closed0: &HashSet<ValveId>,
    path: &mut Vec<ValveId>,
    paths: &mut Vec<Vec<ValveId>>,
    mut left: i32,
) {
    // Open self:
    let mut closed = closed0.clone();
    closed.remove(&current);
    let valve = valves.get(&current).unwrap();
    left -= i32::from(valve.flow > 0);
    // Go to the next valves:
    for &next in &closed {
        let cost = *costs.get(&(current, next)).unwrap();
        if cost > left {
            continue;
        }
        path.push(next);
        all_paths(valves, costs, next, &closed, path, paths, left - cost);
        path.pop();
    }
    paths.push(path.clone());
}

fn path_flow(
    valves: &HashMap<ValveId, Valve>,
    costs: &HashMap<(ValveId, ValveId), i32>,
    path: &[ValveId],
    mut left: i32,
) -> i32 {
    let mut flow = 0;
    for (i, &vid) in path.iter().enumerate().skip(1) {
        let prev_vid = path[i - 1];
        left -= 1 + costs.get(&(prev_vid, vid)).unwrap();
        flow += valves.get(&vid).unwrap().flow * left;
    }
    flow
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
    let mut paths = vec![];
    let mut path = vec![initial];
    all_paths(&valves, &costs, initial, &closed, &mut path, &mut paths, 26);
    Ok(paths
        .par_iter()
        .enumerate()
        .map(|(i, path1)| {
            let mut set1 = path1.iter().collect::<HashSet<_>>();
            set1.remove(&initial);
            let flow1 = path_flow(&valves, &costs, path1, 26);
            paths[i + 1..]
                .par_iter()
                .map(|path2| {
                    if path2.iter().any(|vid| set1.contains(vid)) {
                        0
                    } else {
                        flow1 + path_flow(&valves, &costs, path2, 26)
                    }
                })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 1707);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
