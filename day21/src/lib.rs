// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
pub use std::collections::HashMap;
use std::fmt;

pub const EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonkeyId(pub copstr::Str<4>);

impl MonkeyId {
    pub fn new(id: &str) -> Result<MonkeyId> {
        Ok(MonkeyId(copstr::Str::<4>::new(id)?))
    }
    pub fn root() -> MonkeyId {
        MonkeyId::new("root").unwrap()
    }
    pub fn humn() -> MonkeyId {
        MonkeyId::new("humn").unwrap()
    }
}

impl fmt::Debug for MonkeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MonkeyId({})", self.0.as_str())
    }
}

impl fmt::Display for MonkeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn apply(&self, v1: i64, v2: i64) -> i64 {
        match self {
            Op::Add => v1 + v2,
            Op::Sub => v1 - v2,
            Op::Mul => v1 * v2,
            Op::Div => v1 / v2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Expr {
    Num(i64),
    Op(Op, MonkeyId, MonkeyId),
}

impl Expr {
    pub fn monkeys(&self) -> Option<(MonkeyId, MonkeyId)> {
        match self {
            Expr::Num(_) => None,
            Expr::Op(_, m1, m2) => Some((*m1, *m2)),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn monkeyid(input: &str) -> IResult<&str, MonkeyId> {
        let (input, id) = multi::count(character::anychar, 4)(input)?;
        Ok((input, MonkeyId(id.into_iter().collect())))
    }

    fn expr_num(input: &str) -> IResult<&str, Expr> {
        let (input, num) = character::i64(input)?;
        Ok((input, Expr::Num(num)))
    }

    fn op_add(input: &str) -> IResult<&str, Op> {
        let (input, _) = bytes::tag(" + ")(input)?;
        Ok((input, Op::Add))
    }

    fn op_sub(input: &str) -> IResult<&str, Op> {
        let (input, _) = bytes::tag(" - ")(input)?;
        Ok((input, Op::Sub))
    }

    fn op_mul(input: &str) -> IResult<&str, Op> {
        let (input, _) = bytes::tag(" * ")(input)?;
        Ok((input, Op::Mul))
    }

    fn op_div(input: &str) -> IResult<&str, Op> {
        let (input, _) = bytes::tag(" / ")(input)?;
        Ok((input, Op::Div))
    }

    fn op(input: &str) -> IResult<&str, Op> {
        branch::alt((op_add, branch::alt((op_sub, branch::alt((op_mul, op_div))))))(input)
    }

    fn expr_op(input: &str) -> IResult<&str, Expr> {
        let (input, m1) = monkeyid(input)?;
        let (input, op) = op(input)?;
        let (input, m2) = monkeyid(input)?;
        Ok((input, Expr::Op(op, m1, m2)))
    }

    fn expr(input: &str) -> IResult<&str, Expr> {
        branch::alt((expr_num, expr_op))(input)
    }

    fn line(input: &str) -> IResult<&str, (MonkeyId, Expr)> {
        let (input, m) = monkeyid(input)?;
        let (input, _) = bytes::tag(": ")(input)?;
        let (input, expr) = expr(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (m, expr)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<HashMap<MonkeyId, Expr>> {
        aoc::parse_with!(multi::many1(line), bufin).map(|v| v.into_iter().collect())
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 15);
    Ok(())
}

pub fn solve_monkey(monkeys: &HashMap<MonkeyId, Expr>, m: &MonkeyId) -> i64 {
    let expr = monkeys.get(m).unwrap();
    match expr {
        Expr::Num(num) => *num,
        Expr::Op(op, m1, m2) => op.apply(solve_monkey(monkeys, m1), solve_monkey(monkeys, m2)),
    }
}
