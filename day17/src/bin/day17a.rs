// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::stdin;

use day17::*;

#[test]
fn test() -> Result<()> {
    assert_eq!(process(2022, EXAMPLE.as_bytes())?, 3068);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(2022, stdin().lock())?);
    Ok(())
}
