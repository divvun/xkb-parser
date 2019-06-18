use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_keycodes))]
pub struct XkbKeycodes<'src> {
    pub name: StringContent<'src>,
    pub value: Vec<XkbKeycodesItem<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_keycodes_item))]
pub enum XkbKeycodesItem<'src> {
    #[derivative(Debug = "transparent")]
    Include(Include<'src>),
    #[derivative(Debug = "transparent")]
    Override(Override<'src>),
    #[derivative(Debug = "transparent")]
    Augment(Augment<'src>),

    #[derivative(Debug = "transparent")]
    Minimum(Minimum),
    #[derivative(Debug = "transparent")]
    Maximum(Maximum),
    #[derivative(Debug = "transparent")]
    SymbolMapping(SymbolMapping<'src>),
    #[derivative(Debug = "transparent")]
    Alternate(Alternate<'src>),
    #[derivative(Debug = "transparent")]
    Indicator(Indicator<'src>),
    #[derivative(Debug = "transparent")]
    Alias(Alias<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::minimum))]
pub struct Minimum {
    #[pest_ast(inner(with(parse_u64)))]
    pub level: u64,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::maximum))]
pub struct Maximum {
    #[pest_ast(inner(with(parse_u64)))]
    pub level: u64,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbol_mapping))]
pub struct SymbolMapping<'src> {
    symbol: Symbol<'src>,
    #[pest_ast(inner(with(parse_u64)))]
    pub key_code: u64,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::alternate))]
pub struct Alternate<'src> {
    symbol: Symbol<'src>,
    #[pest_ast(inner(with(parse_u64)))]
    pub key_code: u64,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator))]
pub struct Indicator<'src> {
    pub prefix: Option<IndicatorPrefix<'src>>,
    #[pest_ast(inner(with(parse_u64)))]
    pub id: u64,
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator_prefix))]
pub struct IndicatorPrefix<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::alias))]
pub struct Alias<'src> {
    new: Symbol<'src>,
    original: Symbol<'src>,
}
