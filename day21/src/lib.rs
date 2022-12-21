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
}

impl fmt::Debug for MonkeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MonkeyId({})", self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Expr {
    Num(i64),
    Add(MonkeyId, MonkeyId),
    Sub(MonkeyId, MonkeyId),
    Mul(MonkeyId, MonkeyId),
    Div(MonkeyId, MonkeyId),
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

    fn expr_add(input: &str) -> IResult<&str, Expr> {
        let (input, id1) = monkeyid(input)?;
        let (input, _) = bytes::tag(" + ")(input)?;
        let (input, id2) = monkeyid(input)?;
        Ok((input, Expr::Add(id1, id2)))
    }

    fn expr_sub(input: &str) -> IResult<&str, Expr> {
        let (input, id1) = monkeyid(input)?;
        let (input, _) = bytes::tag(" - ")(input)?;
        let (input, id2) = monkeyid(input)?;
        Ok((input, Expr::Sub(id1, id2)))
    }

    fn expr_mul(input: &str) -> IResult<&str, Expr> {
        let (input, id1) = monkeyid(input)?;
        let (input, _) = bytes::tag(" * ")(input)?;
        let (input, id2) = monkeyid(input)?;
        Ok((input, Expr::Mul(id1, id2)))
    }

    fn expr_div(input: &str) -> IResult<&str, Expr> {
        let (input, id1) = monkeyid(input)?;
        let (input, _) = bytes::tag(" / ")(input)?;
        let (input, id2) = monkeyid(input)?;
        Ok((input, Expr::Div(id1, id2)))
    }

    fn expr(input: &str) -> IResult<&str, Expr> {
        branch::alt((
            expr_num,
            branch::alt((
                expr_add,
                branch::alt((expr_sub, branch::alt((expr_mul, expr_div)))),
            )),
        ))(input)
    }

    fn line(input: &str) -> IResult<&str, (MonkeyId, Expr)> {
        let (input, id) = monkeyid(input)?;
        let (input, _) = bytes::tag(": ")(input)?;
        let (input, expr) = expr(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (id, expr)))
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
