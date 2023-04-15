// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::stdin;

use day17::*;

#[test]
fn test() -> Result<()> {
    assert_eq!(
        process(1_000_000_000_000, EXAMPLE.as_bytes())?,
        1_514_285_714_288
    );
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(1_000_000_000_000, stdin().lock())?);
    Ok(())
}
