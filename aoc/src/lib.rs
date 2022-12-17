#[macro_use]
pub mod parser {
    pub use color_eyre::eyre::eyre;
    pub use color_eyre::Result;
    pub use nom::branch;
    pub use nom::bytes::complete as bytes;
    pub use nom::character::complete as character;
    pub use nom::combinator;
    pub use nom::multi;
    pub use nom::Finish;
    pub use nom::IResult;
    pub use std::io::BufRead;

    #[macro_export]
    macro_rules! parse_with {
        ($parser:expr, $buf:ident) => {{
            let mut input = String::default();
            $buf.read_to_string(&mut input)?;
            let result = combinator::all_consuming($parser)(&input).finish();
            Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
        }};
    }
}
