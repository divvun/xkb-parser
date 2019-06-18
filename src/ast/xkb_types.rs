use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_types))]
pub struct XkbTypes<'src> {
    pub name: StringContent<'src>,
    pub values: Vec<XkbTypesItem<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_types_item))]
pub enum XkbTypesItem<'src> {
    #[derivative(Debug = "transparent")]
    Include(Include<'src>),
    #[derivative(Debug = "transparent")]
    Override(Override<'src>),
    #[derivative(Debug = "transparent")]
    Augment(Augment<'src>),

    #[derivative(Debug = "transparent")]
    VirtualModifiers(VirtualModifiers<'src>),
    #[derivative(Debug = "transparent")]
    TypeItem(TypeItem<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::type_item))]
pub struct TypeItem<'src> {
    pub name: StringContent<'src>,
    pub values: Vec<TypeComponent<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::type_component))]
pub enum TypeComponent<'src> {
    #[derivative(Debug = "transparent")]
    Modifiers(Modifiers<'src>),
    #[derivative(Debug = "transparent")]
    Map(Map<'src>),
    #[derivative(Debug = "transparent")]
    Preserve(Preserve<'src>),
    #[derivative(Debug = "transparent")]
    LevelName(LevelName<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::modifiers))]
pub struct Modifiers<'src> {
    pub name: KeyCombo<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::map))]
pub struct Map<'src> {
    pub group: KeyCombo<'src>,
    pub name: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::preserve))]
pub struct Preserve<'src> {
    pub left: KeyCombo<'src>,
    pub right: KeyCombo<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::level_name))]
pub struct LevelName<'src> {
    pub group: Group<'src>,
    pub content: StringContent<'src>,
}
