// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::eyre;
use eyre::Result;
use std::collections::HashMap;

pub const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

// Input structures:

#[derive(Debug, Clone)]
pub enum Entry {
    Dir(String),
    File(String, u32),
}

impl Entry {
    pub fn name(&self) -> &str {
        match self {
            Entry::Dir(ref name) => name,
            Entry::File(ref name, _) => name,
        }
    }
}

#[derive(Debug)]
pub enum Cmd {
    Ls(Vec<Entry>),
    Cd(String),
}

pub mod parser {
    use eyre::eyre;
    use eyre::Result;
    use nom::branch;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn entryname(input: &str) -> IResult<&str, String> {
        let (input, filename) = multi::many1(character::none_of("\n"))(input)?;
        Ok((input, filename.into_iter().collect()))
    }

    fn entry_dir(input: &str) -> IResult<&str, Entry> {
        let (input, _) = bytes::tag("dir ")(input)?;
        let (input, dirname) = entryname(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Entry::Dir(dirname)))
    }

    fn entry_file(input: &str) -> IResult<&str, Entry> {
        let (input, size) = character::u32(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, filename) = entryname(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Entry::File(filename, size)))
    }

    fn entry(input: &str) -> IResult<&str, Entry> {
        branch::alt((entry_dir, entry_file))(input)
    }

    fn cmd_ls(input: &str) -> IResult<&str, Cmd> {
        let (input, _) = bytes::tag("ls\n")(input)?;
        let (input, entries) = multi::many0(entry)(input)?;
        Ok((input, Cmd::Ls(entries)))
    }

    fn cmd_cd(input: &str) -> IResult<&str, Cmd> {
        let (input, _) = bytes::tag("cd ")(input)?;
        let (input, dirname) = entryname(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Cmd::Cd(dirname)))
    }

    fn cmd(input: &str) -> IResult<&str, Cmd> {
        let (input, _) = bytes::tag("$ ")(input)?;
        branch::alt((cmd_cd, cmd_ls))(input)
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Cmd>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(cmd))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 10);
    Ok(())
}

// Recursive entries

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

pub fn rec_builder(recentry: &mut RecEntry, cmds: &[Cmd], mut icmd: usize) -> Result<usize> {
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
                    return Ok(icmd);
                } else {
                    let recentry_child = recentry
                        .children
                        .get_mut(dir)
                        .ok_or_else(|| eyre!("could not find dir {}", dir))?;
                    icmd = rec_builder(recentry_child, cmds, icmd)?;
                }
            }
        }
    }
    Ok(icmd)
}
