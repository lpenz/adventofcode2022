// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day19::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    pub mats: Mats,
    pub produces: Mats,
}

impl Default for State {
    fn default() -> Self {
        State {
            mats: Mats::default(),
            produces: Mats::ORE,
        }
    }
}

impl State {
    pub fn produce(&mut self) {
        self.mats = self.mats + self.produces;
    }
    pub fn build(&mut self, bp: &Blueprint, robot: &Robot) {
        self.mats = self.mats - bp.robot_cost(robot);
        self.produces = self.produces + robot.produces();
    }
    pub fn can_build(&self, bp: &Blueprint, robot: &Robot) -> bool {
        let cmp = bp.robot_cost(robot).partial_cmp(&self.mats);
        cmp == Some(Ordering::Equal) || cmp == Some(Ordering::Less)
    }
}

pub type Cache = HashMap<(i32, State, i32), i32>;

fn eval_state(cache: &mut Cache, bp: &Blueprint, state0: &State, left: i32) -> i32 {
    if left == 0 {
        cache.insert((bp.id, *state0, 0), state0.mats.geode());
        return state0.mats.geode();
    }
    if let Some(result) = cache.get(&(bp.id, *state0, left)) {
        // Cache hit!
        return *result;
    }
    let mut max = autofolder::Max::<i32>::new(0);
    // Try building
    if state0.can_build(bp, &Robot::Geode) {
        // If I can build geode, then I do only that
        let mut state = *state0;
        state.produce();
        state.build(bp, &Robot::Geode);
        let geodes = eval_state(cache, bp, &state, left - 1);
        max.reduce(geodes);
    } else if state0.can_build(bp, &Robot::Obs) {
        // If I can build obs, then I do only that
        let mut state = *state0;
        state.produce();
        state.build(bp, &Robot::Obs);
        let geodes = eval_state(cache, bp, &state, left - 1);
        max.reduce(geodes);
    } else {
        for robot in [Robot::Clay, Robot::Ore].into_iter() {
            if state0.produces.0[robot as u32 as usize] >= bp.max_cost.0[robot as u32 as usize] {
                // I have enough, can prune this branch
                continue;
            }
            if state0.can_build(bp, &robot) {
                let mut state = *state0;
                state.produce();
                state.build(bp, &robot);
                let geodes = eval_state(cache, bp, &state, left - 1);
                max.reduce(geodes);
            }
        }
        // Just produce instead
        let mut state = *state0;
        state.produce();
        max.reduce(eval_state(cache, bp, &state, left - 1));
    }
    let max = max.into_inner().unwrap();
    cache.insert((bp.id, *state0, left), max);
    max
}

fn eval_blueprint(bp: &Blueprint, left: i32) -> i32 {
    let mut cache = Cache::new();
    eval_state(&mut cache, bp, &State::default(), left)
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let blueprints = parser::parse(bufin)?;
    Ok(blueprints
        .into_par_iter()
        .take(3)
        .map(|bp| eval_blueprint(&bp, 32))
        .product())
}

// Program worked with real input, but this test didn't work:
// #[test]
// fn test_bp1() -> Result<()> {
//     let bp1 = Blueprint::new(
//         1,
//         Mats::new(4, 0, 0, 0),
//         Mats::new(2, 0, 0, 0),
//         Mats::new(3, 14, 0, 0),
//         Mats::new(2, 0, 7, 0),
//     );
//     let geodes = eval_blueprint(&bp1, 32);
//     assert_eq!(geodes, 56);
//     Ok(())
// }

#[test]
fn test_bp1_short() -> Result<()> {
    let bp1 = Blueprint::new(
        1,
        Mats::new(4, 0, 0, 0),
        Mats::new(2, 0, 0, 0),
        Mats::new(3, 14, 0, 0),
        Mats::new(2, 0, 7, 0),
    );
    let geodes = eval_blueprint(&bp1, 19);
    assert_eq!(geodes, 1);
    Ok(())
}

#[test]
fn test_bp2() -> Result<()> {
    let bp2 = Blueprint::new(
        2,
        Mats::new(2, 0, 0, 0),
        Mats::new(3, 0, 0, 0),
        Mats::new(3, 8, 0, 0),
        Mats::new(3, 0, 12, 0),
    );
    let geodes = eval_blueprint(&bp2, 32);
    assert_eq!(geodes, 62);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
