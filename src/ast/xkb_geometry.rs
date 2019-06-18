use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_geometry))]
pub struct XkbGeometry<'src> {
    pub name: StringContent<'src>,
    pub values: Vec<Debug<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_geometry_item))]
pub struct XkbGeometryItem<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub debug: &'src str, // FIXME: actual content pls
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::geometry_assigment))]
pub struct GeometryAssigment<'src> {
    pub key: Ident<'src>,
    pub subkey: Option<Ident<'src>>,
    #[pest_ast(inner(with(span_into_str)))]
    pub value: &'src str,
}
