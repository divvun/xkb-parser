use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::symbol_name))]
pub struct Symbol<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::group))]
pub struct Group<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_combo))]
pub struct KeyCombo<'src> {
    pub content: Vec<Ident<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::string_content))]
pub struct StringContent<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::EOI))]
pub(crate) struct EOI;
