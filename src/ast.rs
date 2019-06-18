use crate::xkb::Rule;
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::file))]
pub struct File<'src> {
    pub definitions: Vec<Definition<'src>>,
    #[derivative(Debug = "ignore")]
    eoi: EOI,
}

mod helpers;
pub(crate) use helpers::*;

mod basic;
pub use basic::*;

mod common;
pub use common::*;

mod xkb_symbols;
pub use xkb_symbols::*;

mod xkb_keycodes;
pub use xkb_keycodes::*;

mod xkb_types;
pub use xkb_types::*;

mod xkb_compatibility;
pub use xkb_compatibility::*;

mod xkb_geometry;
pub use xkb_geometry::*;

#[cfg(test)]
mod tests;
