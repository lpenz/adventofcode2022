// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::cmp::Ordering;
use std::io::{stdin, BufRead};

use day20::*;

fn mix(orig: &[(usize, i32)], num: (usize, i32), nums: &mut Vec<(usize, i32)>) {
    let len = orig.len() as i32;
    let mut jumps = num.1;
    while jumps < 0 {
        jumps += len - 1;
    }
    let oldpos = nums.iter().position(|i| i == &num).unwrap() as i32;
    let mut newpos = oldpos + jumps;
    while newpos > len {
        newpos -= len - 1;
    }
    match newpos.cmp(&oldpos) {
        Ordering::Less => {
            nums.remove(oldpos as usize);
            nums.insert(newpos as usize, num);
        }
        Ordering::Greater => {
            nums.remove(oldpos as usize);
            if newpos == len {
                nums.push(num);
            } else {
                nums.insert(newpos as usize, num);
            }
        }
        _ => {}
    }
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let orig = parser::parse(bufin)?
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    let len = orig.len() as i32;
    let mut nums = orig.clone();
    for num in &orig {
        mix(&orig, *num, &mut nums);
    }
    let pos0 = nums
        .iter()
        .position(|&i| i.1 == 0)
        .ok_or_else(|| eyre!("could not find value 0"))? as i32;
    Ok([1000, 2000, 3000]
        .into_iter()
        .map(|jump| nums[((pos0 + jump) % len) as usize].1)
        .sum::<i32>())
}

pub fn test_mix(orig: &[i32], num: i32, nums: &mut Vec<i32>) {
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

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 3);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
