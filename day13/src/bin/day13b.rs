// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::io::{stdin, BufRead};

use day13::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut packets = input
        .into_iter()
        .flat_map(|(cell1, cell2)| [cell1, cell2].into_iter())
        .collect::<Vec<Cell>>();
    let divider1 = parser::parse_cell("[[2]]".as_bytes())?;
    packets.push(divider1.clone());
    let divider2 = parser::parse_cell("[[6]]".as_bytes())?;
    packets.push(divider2.clone());
    packets.sort();
    let index1 = 1 + packets.iter().position(|p| p == &divider1).unwrap();
    let index2 = 1 + packets.iter().position(|p| p == &divider2).unwrap();
    Ok(index1 * index2)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 140);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
