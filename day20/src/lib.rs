// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use std::io::BufRead;

pub const EXAMPLE: &str = "1
2
-3
3
-2
0
4
";

pub type Num = i64;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn line(input: &str) -> IResult<&str, Num> {
        let (input, num) = character::i64(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Num>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 7);
    Ok(())
}

pub fn mix(orig: &[(usize, Num)], num: (usize, Num), nums: &mut Vec<(usize, Num)>) {
    let len = orig.len() as Num;
    let oldpos = nums.iter().position(|i| i == &num).unwrap() as Num;
    let mut newpos = (oldpos + num.1).rem_euclid(len - 1);
    if newpos == 0 {
        newpos = len - 1;
    }
    nums.remove(oldpos as usize);
    nums.insert(newpos as usize, num);
}

pub fn test_mix(orig: &[Num], num: Num, nums: &mut Vec<Num>) {
    let orig = orig.iter().copied().enumerate().collect::<Vec<_>>();
    let pos = nums.iter().position(|i| i == &num).unwrap();
    let mut nums_tmp = nums.iter().copied().enumerate().collect::<Vec<_>>();
    mix(&orig, (pos, num), &mut nums_tmp);
    *nums = nums_tmp.into_iter().map(|(_, i)| i).collect::<Vec<_>>();
}

#[test]
fn test_steps() -> Result<()> {
    let mut nums = vec![1, 2, -3, 3, -2, 0, 4];
    let orig = nums.clone();
    test_mix(&orig, 1, &mut nums);
    assert_eq!(nums, [2, 1, -3, 3, -2, 0, 4]);
    test_mix(&orig, 2, &mut nums);
    assert_eq!(nums, [1, -3, 2, 3, -2, 0, 4]);
    test_mix(&orig, -3, &mut nums);
    assert_eq!(nums, [1, 2, 3, -2, -3, 0, 4]);
    test_mix(&orig, 3, &mut nums);
    assert_eq!(nums, [1, 2, -2, -3, 0, 3, 4]);
    test_mix(&orig, -2, &mut nums);
    assert_eq!(nums, [1, 2, -3, 0, 3, 4, -2]);
    test_mix(&orig, 0, &mut nums);
    assert_eq!(nums, [1, 2, -3, 0, 3, 4, -2]);
    test_mix(&orig, 4, &mut nums);
    assert_eq!(nums, [1, 2, -3, 4, 0, 3, -2]);
    Ok(())
}

pub fn do_process<const MUL: i64, const MIXES: i32>(bufin: impl BufRead) -> Result<Num> {
    let orig = parser::parse(bufin)?
        .into_iter()
        .map(|i| i * MUL)
        .enumerate()
        .collect::<Vec<_>>();
    let len = orig.len() as Num;
    let mut nums = orig.clone();
    for _ in 0..MIXES {
        for num in &orig {
            mix(&orig, *num, &mut nums);
        }
    }
    let pos0 = nums
        .iter()
        .position(|&i| i.1 == 0)
        .ok_or_else(|| eyre!("could not find value 0"))? as Num;
    Ok([1000, 2000, 3000]
        .into_iter()
        .map(|jump| nums[((pos0 + jump) % len) as usize].1)
        .sum::<Num>())
}
