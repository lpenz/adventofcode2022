// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Item(char);

impl Item {
    pub fn priority(&self) -> u8 {
        if self.0.is_ascii_lowercase() {
            self.0 as u8 - b'a' + 1
        } else {
            // uppercase
            self.0 as u8 - b'A' + 27
        }
    }
}

impl TryFrom<char> for Item {
    type Error = Error;
    fn try_from(c: char) -> Result<Self> {
        if c.is_ascii_alphabetic() {
            Ok(Item(c))
        } else {
            Err(anyhow!("Invalid item {}", c))
        }
    }
}

pub type Rucksack = Vec<Item>;

pub const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn item(input: &str) -> IResult<&str, Item> {
        // Item::try_from fails at newline, which makes the parser fail,
        // which is what we want:
        combinator::map_res(character::anychar, Item::try_from)(input)
    }

    fn rucksack(input: &str) -> IResult<&str, Rucksack> {
        let (input, rucksack) = multi::many1(item)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, rucksack))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Rucksack>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(rucksack))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input[0].len(), 24);
    assert_eq!(input[1].len(), 32);
    assert_eq!(input[2].len(), 18);
    assert_eq!(input[3].len(), 30);
    assert_eq!(input[4].len(), 16);
    assert_eq!(input[5].len(), 24);
    Ok(())
}
