// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::VecDeque;

pub const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

pub type Worry = u64;

#[derive(Debug)]
pub enum Op {
    Add(Worry),
    Mul(Worry),
    Square,
}

impl Op {
    pub fn apply(&self, arg: Worry) -> Worry {
        match self {
            Op::Add(val) => arg + val,
            Op::Mul(val) => arg * val,
            Op::Square => arg * arg,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<Worry>,
    pub op: Op,
    pub test: Worry,
    pub monkey_true: usize,
    pub monkey_false: usize,
    pub inspected: u32,
}

pub mod parser {
    use eyre::eyre;
    use eyre::Result;
    use nom::branch;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn op_add(input: &str) -> IResult<&str, Op> {
        let (input, _) = bytes::tag("+ ")(input)?;
        let (input, value) = character::u32(input)?;
        Ok((input, Op::Add(Worry::from(value))))
    }

    fn op_mul(input: &str) -> IResult<&str, Op> {
        let (input, _) = bytes::tag("* ")(input)?;
        let (input, value) = character::u32(input)?;
        Ok((input, Op::Mul(Worry::from(value))))
    }

    fn op_square(input: &str) -> IResult<&str, Op> {
        let (input, _) = bytes::tag("* old")(input)?;
        Ok((input, Op::Square))
    }

    fn op(input: &str) -> IResult<&str, Op> {
        branch::alt((op_add, op_square, op_mul))(input)
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let (input, _) = bytes::tag("Monkey ")(input)?;
        let (input, id) = character::u32(input)?;
        let (input, _) = bytes::tag(":\n  Starting items: ")(input)?;
        let (input, items) = multi::separated_list1(bytes::tag(", "), character::u32)(input)?;
        let (input, _) = bytes::tag("\n  Operation: new = old ")(input)?;
        let (input, op) = op(input)?;
        let (input, _) = bytes::tag("\n  Test: divisible by ")(input)?;
        let (input, test) = character::u32(input)?;
        let (input, _) = bytes::tag("\n    If true: throw to monkey ")(input)?;
        let (input, monkey_true) = character::u32(input)?;
        let (input, _) = bytes::tag("\n    If false: throw to monkey ")(input)?;
        let (input, monkey_false) = character::u32(input)?;
        let (input, _) = character::newline(input)?;
        Ok((
            input,
            Monkey {
                id: id as usize,
                items: items.into_iter().map(Worry::from).collect(),
                op,
                test: Worry::from(test),
                monkey_true: monkey_true as usize,
                monkey_false: monkey_false as usize,
                inspected: 0,
            },
        ))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Monkey>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result =
            combinator::all_consuming(multi::separated_list1(character::newline, monkey))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 4);
    Ok(())
}

pub fn monkey_business<const ROUNDS: usize, const DIVIDER: Worry>(
    mut monkeys: Vec<Monkey>,
) -> Result<u64> {
    let module: Worry = monkeys.iter().map(|m| m.test).product();
    for _round in 0..ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspected += 1;
                let worry = (monkeys[i].op.apply(item) / DIVIDER) % module;
                let target_monkey = if worry % monkeys[i].test == 0 {
                    monkeys[i].monkey_true
                } else {
                    monkeys[i].monkey_false
                };
                monkeys[target_monkey].items.push_back(worry);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspections.sort();
    Ok(inspections
        .into_iter()
        .rev()
        .take(2)
        .map(|v| v as u64)
        .product())
}
