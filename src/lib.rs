pub use crate::xkb::{Rule, XkbParser};
pub use ast::File as Xkb;
use from_pest::FromPest;
use pest::Parser;
use std::error::Error;

pub fn parse(file: &str) -> Result<Xkb, Box<dyn Error>> {
    let mut parse_tree = XkbParser::parse(Rule::file, file)?;
    let syntax_tree =
        Xkb::from_pest(&mut parse_tree).map_err(|e| format!("ast generation failed: {:?}", e))?;

    Ok(syntax_tree)
}

pub(crate) mod xkb {
    #[derive(pest_derive::Parser)]
    #[grammar = "xkb.pest"]
    pub struct XkbParser;
}

pub mod ast;
