// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use eyre::Result;

pub use sqrid::Qr;

pub type Sqrid = sqrid::sqrid_create!(143, 41, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, char);

pub const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

pub type Cell = char;

pub mod parser {
    use eyre::eyre;
    use eyre::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::*;

    fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, line) = multi::many1(character::none_of("\n"))(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, line))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Cell>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(line))(&input);
        Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 5);
    Ok(())
}

pub fn vecs2grid(input: Vec<Vec<Cell>>) -> Result<(Qa, Qa, Grid)> {
    let mut grid = Grid::repeat(char::from(b'~'));
    let mut src = Qa::default();
    let mut dst = Qa::default();
    for (y, line) in input.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            if line[x] == 'S' {
                src = qa;
                grid[qa] = 'a';
            } else if line[x] == 'E' {
                dst = qa;
                grid[qa] = 'z';
            } else {
                grid[qa] = *cell;
            }
        }
    }
    Ok((src, dst, grid))
}
