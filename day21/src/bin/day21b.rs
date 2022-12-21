// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day21::*;

pub fn find_humn(
    monkeys: &HashMap<MonkeyId, Expr>,
    m: &MonkeyId,
    humn_branch: &mut HashSet<MonkeyId>,
) -> bool {
    if humn_branch.contains(m) {
        true
    } else {
        let expr = *monkeys.get(m).unwrap();
        match expr {
            Expr::Num(_) => false,
            Expr::Op(_, m1, m2) => {
                let b1 = find_humn(monkeys, &m1, humn_branch);
                if b1 {
                    humn_branch.insert(m1);
                }
                let b2 = find_humn(monkeys, &m2, humn_branch);
                if b2 {
                    humn_branch.insert(m2);
                }
                b1 || b2
            }
        }
    }
}

pub fn expr_humn01(humn_branch: &HashSet<MonkeyId>, expr: Expr) -> Option<(MonkeyId, MonkeyId)> {
    let (m0, m1) = expr.monkeys()?;
    Some(if humn_branch.contains(&m1) {
        assert!(!humn_branch.contains(&m0));
        (m0, m1)
    } else {
        (m1, m0)
    })
}

pub fn solve_humn(
    monkeys: &HashMap<MonkeyId, Expr>,
    humn_branch: &HashSet<MonkeyId>,
    m: MonkeyId,
    value: i64,
) -> i64 {
    if m == MonkeyId::humn() {
        value
    } else {
        let expr = *monkeys.get(&m).unwrap();
        let (op, m0, _m1) = match expr {
            Expr::Num(_) => panic!("called with non-humn parent {:?}", m),
            Expr::Op(op, m0, m1) => (op, m0, m1),
        };
        let (humn0, humn1) = expr_humn01(humn_branch, expr).unwrap();
        let humn0_value = solve_monkey(monkeys, &humn0);
        match op {
            Op::Add => solve_humn(monkeys, humn_branch, humn1, value - humn0_value),
            Op::Sub => {
                if humn0 == m0 {
                    solve_humn(monkeys, humn_branch, humn1, humn0_value - value)
                } else {
                    solve_humn(monkeys, humn_branch, humn1, value + humn0_value)
                }
            }
            Op::Mul => solve_humn(monkeys, humn_branch, humn1, value / humn0_value),
            Op::Div => {
                if humn0 == m0 {
                    solve_humn(monkeys, humn_branch, humn1, humn0_value / value)
                } else {
                    solve_humn(monkeys, humn_branch, humn1, value * humn0_value)
                }
            }
        }
    }
}

fn process(bufin: impl BufRead) -> Result<i64> {
    let monkeys = parser::parse(bufin)?;
    let root_expr = *monkeys.get(&MonkeyId::root()).unwrap();
    let mut humn_branch = HashSet::<MonkeyId>::new();
    humn_branch.insert(MonkeyId::humn());
    find_humn(&monkeys, &MonkeyId::root(), &mut humn_branch);
    let (humn0, humn1) = expr_humn01(&humn_branch, root_expr).unwrap();
    let value = solve_monkey(&monkeys, &humn0);
    Ok(solve_humn(&monkeys, &humn_branch, humn1, value))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 301);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
