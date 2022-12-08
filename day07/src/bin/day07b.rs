// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::eyre;
use eyre::Result;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day07::*;

pub fn size_visitor(recentry: &RecEntry, dirsize: &mut HashMap<String, u32>) -> u32 {
    match recentry.entry {
        Entry::Dir(ref name) => {
            let size = recentry
                .children
                .iter()
                .map(|(_, e)| size_visitor(e, dirsize))
                .sum();
            dirsize.insert(name.to_string(), size);
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
    let mut dirsize = HashMap::default();
    size_visitor(&root, &mut dirsize);
    let used = dirsize.get("/").ok_or_else(|| eyre!("root not found"))?;
    let unused = 70000000 - used;
    let needed = 30000000 - unused;
    let toclear = dirsize
        .into_values()
        .filter(|&size| size >= needed)
        .min()
        .ok_or_else(|| eyre!("no dir with sufficient size"))?;
    Ok(toclear)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 24933642);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
