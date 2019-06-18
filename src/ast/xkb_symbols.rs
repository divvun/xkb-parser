use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_symbols))]
pub struct XkbSymbols<'src> {
    pub name: StringContent<'src>,
    pub value: Vec<XkbSymbolsItem<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_symbols_item))]
pub enum XkbSymbolsItem<'src> {
    #[derivative(Debug = "transparent")]
    Include(Include<'src>),
    #[derivative(Debug = "transparent")]
    Override(Override<'src>),
    #[derivative(Debug = "transparent")]
    Augment(Augment<'src>),

    #[derivative(Debug = "transparent")]
    Name(Name<'src>),
    #[derivative(Debug = "transparent")]
    Key(Key<'src>),
    #[derivative(Debug = "transparent")]
    KeyType(KeyType<'src>),
    #[derivative(Debug = "transparent")]
    VirtualModifiers(VirtualModifiers<'src>),
    #[derivative(Debug = "transparent")]
    ModifierMap(ModifierMap<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::name))]
pub struct Name<'src> {
    pub group: Group<'src>,
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key_type))]
pub struct KeyType<'src> {
    pub group: Option<Group<'src>>,
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
#[pest_ast(rule(Rule::key))]
pub struct Key<'src> {
    pub mode: Option<KeyMode>,
    pub id: Symbol<'src>,
    pub values: Vec<KeyValue<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_mode))]
pub enum KeyMode {
    KeyModeReplace(KeyModeReplace),
    KeyModeOverride(KeyModeOverride),
    KeyModeAugment(KeyModeAugment),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key_mode_replace))]
pub struct KeyModeReplace;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key_mode_override))]
pub struct KeyModeOverride;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key_mode_augment))]
pub struct KeyModeAugment;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_value))]
pub enum KeyValue<'src> {
    KeyNames(KeyNames<'src>),
    KeyDefs(KeyDef<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_names))]
pub struct KeyNames<'src> {
    pub values: Vec<Ident<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_def))]
pub enum KeyDef<'src> {
    TypeDef(TypeDef<'src>),
    SymbolDef(SymbolDef<'src>),
    VirtualModsDef(VirtualModsDef<'src>),
    ActionsDef(ActionsDef<'src>),
    OverlayDef(OverlayDef<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::type_def))]
pub struct TypeDef<'src> {
    pub group: Option<Group<'src>>,
    #[pest_ast(inner(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbol_def))]
pub struct SymbolDef<'src> {
    pub group: Group<'src>,
    pub values: KeyNames<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::virtual_mods_def))]
pub struct VirtualModsDef<'src> {
    pub name: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::actions_def))]
pub struct ActionsDef<'src> {
    pub group: Group<'src>,
    pub values: Vec<Action<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::action))]
pub struct Action<'src> {
    pub name: Ident<'src>,
    pub param_name: Ident<'src>,
    pub param_values: Vec<KeyCombo<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::overlay_def))]
pub struct OverlayDef<'src> {
    #[pest_ast(inner(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub level: u64,
    pub key: Symbol<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::modifier_map))]
pub struct ModifierMap<'src> {
    pub name: Ident<'src>,
    pub values: Vec<Modifier<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::modifier))]
pub enum Modifier<'src> {
    KeyId(Symbol<'src>),
    Ident(Ident<'src>),
}
