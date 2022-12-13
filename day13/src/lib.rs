// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::VecDeque;

use eyre::eyre;
use eyre::Result;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

pub const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Int(u32),
    List(VecDeque<Cell>),
}

impl Cell {
    fn wrap(cell: &Cell) -> Cell {
        Cell::List(VecDeque::from_iter([cell.clone()]))
    }

    fn tail(cell: &Cell) -> Result<Cell> {
        if let Cell::List(list) = cell {
            Ok(Cell::List(list.iter().skip(1).cloned().collect()))
        } else {
            Err(eyre!("can't get tail of Cell::Int"))
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Cell::Int(int1), Cell::Int(int2)) => int1.cmp(int2),
            (Cell::List(list1), Cell::List(list2)) => match (list1.is_empty(), list2.is_empty()) {
                (true, true) => Equal,
                (true, _) => Less,
                (_, true) => Greater,
                _ => {
                    let cmp = list1[0].cmp(&list2[0]);
                    if cmp != Equal {
                        cmp
                    } else {
                        Cell::tail(self).unwrap().cmp(&Cell::tail(other).unwrap())
                    }
                }
            },
            (Cell::List(_), Cell::Int(_)) => self.cmp(&Cell::wrap(other)),
            (Cell::Int(_), Cell::List(_)) => Cell::wrap(self).cmp(other),
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

    fn cell_int(input: &str) -> IResult<&str, Cell> {
        let (input, int) = character::u32(input)?;
        Ok((input, Cell::Int(int)))
    }

    fn cell_list(input: &str) -> IResult<&str, Cell> {
        let (input, _) = bytes::tag("[")(input)?;
        let (input, children) = multi::separated_list0(bytes::tag(","), cell)(input)?;
        let (input, _) = bytes::tag("]")(input)?;
        Ok((input, Cell::List(children.into_iter().collect())))
    }

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, cell) = branch::alt((cell_int, cell_list))(input)?;
        Ok((input, cell))
    }

    fn pair(input: &str) -> IResult<&str, (Cell, Cell)> {
        let (input, cell1) = cell(input)?;
        let (input, _) = character::newline(input)?;
        let (input, cell2) = cell(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (cell1, cell2)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Cell, Cell)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result =
            combinator::all_consuming(multi::separated_list1(character::newline, pair))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }

    pub fn parse_cell(mut bufin: impl BufRead) -> Result<Cell> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(cell)(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 8);
    Ok(())
}

#[test]
fn test_each() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input[0].0.cmp(&input[0].1), Less);
    assert_eq!(input[1].0.cmp(&input[1].1), Less);
    assert_eq!(input[2].0.cmp(&input[2].1), Greater);
    assert_eq!(input[3].0.cmp(&input[3].1), Less);
    assert_eq!(input[4].0.cmp(&input[4].1), Greater);
    assert_eq!(input[5].0.cmp(&input[5].1), Less);
    assert_eq!(input[6].0.cmp(&input[6].1), Greater);
    assert_eq!(input[7].0.cmp(&input[7].1), Greater);
    Ok(())
}
