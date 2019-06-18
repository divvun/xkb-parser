use crate::{ast::*, xkb::Rule};
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

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::definition))]
pub struct Definition<'src> {
    pub modifiers: BlockModifiers<'src>,
    pub symbols: Directive<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::block_modifiers))]
pub struct BlockModifiers<'src> {
    pub values: Vec<BlockModifier<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::block_modifier))]
pub struct BlockModifier<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::directive))]
pub enum Directive<'src> {
    #[derivative(Debug = "transparent")]
    XkbSymbols(XkbSymbols<'src>),
    #[derivative(Debug = "transparent")]
    XkbKeycodes(XkbKeycodes<'src>),
    #[derivative(Debug = "transparent")]
    XkbTypes(XkbTypes<'src>),
    #[derivative(Debug = "transparent")]
    XkbCompatibility(XkbCompatibility<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::include))]
pub struct Include<'src> {
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::override_))]
pub struct Override<'src> {
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::augment))]
pub struct Augment<'src> {
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::virtual_modifiers))]
pub struct VirtualModifiers<'src> {
    pub name: Vec<KeyCombo<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::action))]
pub struct Action<'src> {
    pub name: Ident<'src>,
    pub params: Vec<ActionParam<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::action_param))]
pub enum ActionParam<'src> {
    #[derivative(Debug = "transparent")]
    ParamAssignment(ParamAssignment<'src>),
    #[derivative(Debug = "transparent")]
    ParamExpression(ParamExpression<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::param_assignment))]
pub struct ParamAssignment<'src> {
    pub ident: Ident<'src>,
    pub expr: ParamExpression<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::param_expression))]
pub struct ParamExpression<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub content: &'src str,
}
