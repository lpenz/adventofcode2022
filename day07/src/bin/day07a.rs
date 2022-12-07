// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day07::*;

#[derive(Debug)]
pub struct RecEntry {
    pub entry: Entry,
    pub children: HashMap<String, RecEntry>,
}

impl From<&Entry> for RecEntry {
    fn from(entry: &Entry) -> Self {
        Self {
            entry: entry.clone(),
            children: HashMap::default(),
        }
    }
}

fn builder(recentry: &mut RecEntry, cmds: &[Cmd], mut icmd: usize) -> usize {
    while icmd < cmds.len() {
        let cmd = &cmds[icmd];
        match cmd {
            Cmd::Ls(ref entries) => {
                recentry.children = entries
                    .iter()
                    .map(|entry| (entry.name().to_string(), RecEntry::from(entry)))
                    .collect();
                icmd += 1;
            }
            Cmd::Cd(ref dir) => {
                icmd += 1;
                if dir == ".." {
                    return icmd;
                } else {
                    let recentry_child = recentry.children.get_mut(dir).unwrap();
                    icmd = builder(recentry_child, cmds, icmd);
                }
            }
        }
    }
    icmd
}

fn size_visitor(recentry: &RecEntry, capped: &mut u32) -> u32 {
    match recentry.entry {
        Entry::Dir(_) => {
            let size = recentry
                .children
                .iter()
                .map(|(_, e)| size_visitor(e, capped))
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
    builder(&mut root, &input, 1);
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
