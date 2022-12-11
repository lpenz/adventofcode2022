// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day11::*;

fn process(bufin: impl BufRead) -> Result<u32> {
    let mut monkeys = parser::parse(bufin)?;
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspected += 1;
                let worry = monkeys[i].op.apply(item) / 3;
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
    Ok(inspections.into_iter().rev().take(2).product())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 10605);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
