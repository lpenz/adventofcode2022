// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::eyre;
use eyre::Result;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

use day13::*;

fn cell_wrap(cell: &Cell) -> Cell {
    Cell::List(VecDeque::from_iter([cell.clone()]))
}

fn cell_tail(cell: &Cell) -> Result<Cell> {
    if let Cell::List(list) = cell {
        Ok(Cell::List(list.iter().skip(1).cloned().collect()))
    } else {
        Err(eyre!("can't get tail of Cell::Int"))
    }
}

fn cell_compare(cell1: &Cell, cell2: &Cell) -> Result<Ordering> {
    match (cell1, cell2) {
        (Cell::Int(int1), Cell::Int(int2)) => Ok(int1.cmp(int2)),
        (Cell::List(list1), Cell::List(list2)) => {
            if list1.is_empty() && list2.is_empty() {
                Ok(Equal)
            } else if list1.is_empty() {
                Ok(Less)
            } else if list2.is_empty() {
                Ok(Greater)
            } else {
                let cmp = cell_compare(&list1[0], &list2[0])?;
                if cmp != Equal {
                    Ok(cmp)
                } else {
                    cell_compare(&cell_tail(cell1)?, &cell_tail(cell2)?)
                }
            }
        }
        (Cell::List(_), Cell::Int(_)) => cell_compare(cell1, &cell_wrap(cell2)),
        (Cell::Int(_), Cell::List(_)) => cell_compare(&cell_wrap(cell1), cell2),
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            if cell_compare(&pair.0, &pair.1).ok()? == Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum())
}

#[test]
fn test_each() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(cell_compare(&input[0].0, &input[0].1)?, Less);
    assert_eq!(cell_compare(&input[1].0, &input[1].1)?, Less);
    assert_eq!(cell_compare(&input[2].0, &input[2].1)?, Greater);
    assert_eq!(cell_compare(&input[3].0, &input[3].1)?, Less);
    assert_eq!(cell_compare(&input[4].0, &input[4].1)?, Greater);
    assert_eq!(cell_compare(&input[5].0, &input[5].1)?, Less);
    assert_eq!(cell_compare(&input[6].0, &input[6].1)?, Greater);
    assert_eq!(cell_compare(&input[7].0, &input[7].1)?, Greater);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 13);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
