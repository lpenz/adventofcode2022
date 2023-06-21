// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use std::cmp;
use std::cmp::Ordering;
use std::ops;

pub const EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mats(pub [i32; 4]);

impl Mats {
    pub const ORE: Mats = Mats::const_mats::<1, 0, 0, 0>();
    pub const CLAY: Mats = Mats::const_mats::<0, 1, 0, 0>();
    pub const OBS: Mats = Mats::const_mats::<0, 0, 1, 0>();
    pub const GEODE: Mats = Mats::const_mats::<0, 0, 0, 1>();
    pub fn new(ore: i32, clay: i32, obs: i32, geode: i32) -> Mats {
        Mats([ore, clay, obs, geode])
    }
    pub const fn const_mats<const ORE: i32, const CLAY: i32, const OBS: i32, const GEODE: i32>(
    ) -> Mats {
        Mats([ORE, CLAY, OBS, GEODE])
    }
    pub fn geode(&self) -> i32 {
        self.0[3]
    }
}

impl ops::Add for Mats {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Mats([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
        ])
    }
}

impl ops::Sub for Mats {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Mats([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
            self.0[3] - other.0[3],
        ])
    }
}

impl cmp::PartialOrd for Mats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.partial_cmp(b))
            .reduce(|prev, current| {
                if current == Some(Ordering::Equal) || current == prev {
                    prev
                } else if prev == Some(Ordering::Equal) {
                    current
                } else {
                    None
                }
            })
            .unwrap()
    }
}

#[test]
fn test_ord() {
    assert_eq!(Mats::ORE.partial_cmp(&Mats::CLAY), None);
    assert_eq!(
        Mats::ORE.partial_cmp(&(Mats::ORE + Mats::ORE)),
        Some(Ordering::Less)
    );
    assert_eq!(
        (Mats::ORE + Mats::ORE + Mats::CLAY).partial_cmp(&Mats::ORE),
        Some(Ordering::Greater)
    );
    assert_eq!(Mats::GEODE.partial_cmp(&Mats::GEODE), Some(Ordering::Equal));
}

#[derive(Debug)]
pub struct Blueprint {
    pub id: i32,
    pub ore_cost: Mats,
    pub clay_cost: Mats,
    pub obs_cost: Mats,
    pub geode_cost: Mats,
    pub max_cost: Mats,
}

impl Blueprint {
    pub fn new(
        id: i32,
        ore_cost: Mats,
        clay_cost: Mats,
        obs_cost: Mats,
        geode_cost: Mats,
    ) -> Blueprint {
        let mut bp = Blueprint {
            id,
            ore_cost,
            clay_cost,
            obs_cost,
            geode_cost,
            max_cost: Mats::default(),
        };
        bp.max_cost = bp._max_cost();
        bp
    }
    fn _max_cost(&self) -> Mats {
        let costs = [
            self.ore_cost,
            self.clay_cost,
            self.obs_cost,
            self.geode_cost,
        ];
        Mats::new(
            costs.map(|c| c.0[0]).into_iter().max().unwrap(),
            costs.map(|c| c.0[1]).into_iter().max().unwrap(),
            costs.map(|c| c.0[2]).into_iter().max().unwrap(),
            costs.map(|c| c.0[3]).into_iter().max().unwrap(),
        )
    }
    pub fn robot_cost(&self, robot: &Robot) -> Mats {
        match robot {
            Robot::Ore => self.ore_cost,
            Robot::Clay => self.clay_cost,
            Robot::Obs => self.obs_cost,
            Robot::Geode => self.geode_cost,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Robot {
    Ore = 0,
    Clay = 1,
    Obs = 2,
    Geode = 3,
}

impl Robot {
    pub const ROBOTS: [Robot; 4] = [Robot::Ore, Robot::Clay, Robot::Obs, Robot::Geode];
    pub fn produces(&self) -> Mats {
        match self {
            Robot::Ore => Mats::ORE,
            Robot::Clay => Mats::CLAY,
            Robot::Obs => Mats::OBS,
            Robot::Geode => Mats::GEODE,
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn line(input: &str) -> IResult<&str, Blueprint> {
        let (input, _) = bytes::tag("Blueprint ")(input)?;
        let (input, id) = character::i32(input)?;
        let (input, _) = bytes::tag(": Each ore robot costs ")(input)?;
        let (input, ore_ore) = character::i32(input)?;
        let ore_cost = Mats::new(ore_ore, 0, 0, 0);
        let (input, _) = bytes::tag(" ore. Each clay robot costs ")(input)?;
        let (input, clay_ore) = character::i32(input)?;
        let clay_cost = Mats::new(clay_ore, 0, 0, 0);
        let (input, _) = bytes::tag(" ore. Each obsidian robot costs ")(input)?;
        let (input, obs_ore) = character::i32(input)?;
        let (input, _) = bytes::tag(" ore and ")(input)?;
        let (input, obs_clay) = character::i32(input)?;
        let obs_cost = Mats::new(obs_ore, obs_clay, 0, 0);
        let (input, _) = bytes::tag(" clay. Each geode robot costs ")(input)?;
        let (input, geode_ore) = character::i32(input)?;
        let (input, _) = bytes::tag(" ore and ")(input)?;
        let (input, geode_obs) = character::i32(input)?;
        let geode_cost = Mats::new(geode_ore, 0, geode_obs, 0);
        let (input, _) = bytes::tag(" obsidian.\n")(input)?;
        Ok((
            input,
            Blueprint::new(id, ore_cost, clay_cost, obs_cost, geode_cost),
        ))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Blueprint>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 2);
    Ok(())
}
