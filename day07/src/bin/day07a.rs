// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day07::*;

fn size_visitor(recentry: &RecEntry, capped: &mut u32) -> u32 {
    match recentry.entry {
        Entry::Dir(_) => {
            let size = recentry
                .children
                .values()
                .map(|e| size_visitor(e, capped))
                .sum();
            if size < 100000 {
                *capped += size;
            }
            size
        }
        Entry::File(_, size) => size,
    }
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let mut root = RecEntry {
        entry: Entry::Dir(String::from("/")),
        children: HashMap::default(),
    };
    rec_builder(&mut root, &input, 1)?;
    let mut capped = 0;
    size_visitor(&root, &mut capped);
    Ok(capped)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 95437);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
