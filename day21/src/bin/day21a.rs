// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day21::*;

pub fn solve_monkey(monkeys: &HashMap<MonkeyId, Expr>, m: &MonkeyId) -> i64 {
    let expr = monkeys.get(m).unwrap();
    match expr {
        Expr::Num(num) => *num,
        Expr::Add(m1, m2) => solve_monkey(monkeys, m1) + solve_monkey(monkeys, m2),
        Expr::Sub(m1, m2) => solve_monkey(monkeys, m1) - solve_monkey(monkeys, m2),
        Expr::Mul(m1, m2) => solve_monkey(monkeys, m1) * solve_monkey(monkeys, m2),
        Expr::Div(m1, m2) => solve_monkey(monkeys, m1) / solve_monkey(monkeys, m2),
    }
}

fn process(bufin: impl BufRead) -> Result<i64> {
    let monkeys = parser::parse(bufin)?;
    Ok(solve_monkey(&monkeys, &MonkeyId::new("root")?))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 152);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
