use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_keycodes))]
pub struct XkbKeycodes<'src> {
    pub name: StringContent<'src>,
    pub symbols: Vec<XkbKeycodesItem<'src>>,
}

// FIXME: not impl'd
#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_keycodes_item))]
pub struct XkbKeycodesItem<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub debug: &'src str,
}
