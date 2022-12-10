// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use sqrid::qaqr::qaqr_resolve;

use day09::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut head = Qa::new::<500, 500>();
    let mut tail = Qa::new::<500, 500>();
    let mut visited = HashSet::<Qa>::default();
    let mut dirs = Qr::iter::<true>().collect::<Vec<_>>();
    dirs.sort_by_key(|qr| qr.is_diagonal());
    visited.insert(head);
    for mv in input {
        for _ in 0..mv.dist {
            head = qaqr_resolve(head, mv.qr)?;
            if head != tail && !Qr::iter::<true>().any(|d| head + d == Some(tail)) {
                for qr in &dirs {
                    if let Some(newtail) = tail + qr {
                        if Qa::manhattan(&newtail, &head) == 1 {
                            tail = newtail;
                            break;
                        }
                    }
                }
            }
            visited.insert(tail);
        }
    }
    Ok(visited.len())
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
