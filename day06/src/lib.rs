// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::eyre;
use eyre::Result;

pub const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n";
pub const EXAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz\n";
pub const EXAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg\n";
pub const EXAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n";
pub const EXAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n";

pub mod parser {
    use eyre::Result;
    use std::io::BufRead;

    pub fn parse(mut bufin: impl BufRead) -> Result<String> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        input.pop();
        Ok(input)
    }
}

pub fn solve<const WINDOW_SIZE: usize>(input: String) -> Result<usize> {
    let binput = input.as_bytes();
    for istart in 0..binput.len() {
        let mut slice: [u8; WINDOW_SIZE] = binput[istart..(istart + WINDOW_SIZE)].try_into()?;
        slice.sort();
        let duplicate = (1..slice.len()).any(|i| slice[i - 1] == slice[i]);
        if !duplicate {
            return Ok(istart + WINDOW_SIZE);
        }
    }
    Err(eyre!("marker not found"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    );
    Ok(())
}
