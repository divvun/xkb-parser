use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_compatibility))]
pub struct XkbCompatibility<'src> {
    pub name: StringContent<'src>,
    pub values: Vec<XkbCompatItem<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_compat_item))]
pub enum XkbCompatItem<'src> {
    #[derivative(Debug = "transparent")]
    Include(Include<'src>),
    #[derivative(Debug = "transparent")]
    Override(Override<'src>),
    #[derivative(Debug = "transparent")]
    Augment(Augment<'src>),

    #[derivative(Debug = "transparent")]
    VirtualModifiers(VirtualModifiers<'src>),
    #[derivative(Debug = "transparent")]
    CompatSetMods(CompatSetMods<'src>),
    #[derivative(Debug = "transparent")]
    CompatLatchMods(CompatLatchMods<'src>),
    #[derivative(Debug = "transparent")]
    CompatGroup(CompatGroup<'src>),
    #[derivative(Debug = "transparent")]
    CompatInterpretLine(CompatInterpretLine<'src>),
    #[derivative(Debug = "transparent")]
    CompatIndicatorLine(CompatIndicatorLine<'src>),
    #[derivative(Debug = "transparent")]
    CompatInterpretBlock(CompatInterpretBlock<'src>),
    #[derivative(Debug = "transparent")]
    CompatIndicatorBlock(CompatIndicatorBlock<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_set_mods))]
pub struct CompatSetMods<'src> {
    pub item: Ident<'src>,
    pub value: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_latch_mods))]
pub struct CompatLatchMods<'src> {
    pub item: Ident<'src>,
    pub value: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_group))]
pub struct CompatGroup<'src> {
    pub name: Ident<'src>,
    pub value: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_interpret_line))]
pub struct CompatInterpretLine<'src> {
    pub key: Ident<'src>,
    pub value: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_indicator_line))]
pub struct CompatIndicatorLine<'src> {
    pub key: Ident<'src>,
    pub value: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_interpret_block))]
pub struct CompatInterpretBlock<'src> {
    pub keys: KeyCombo<'src>,
    pub condition: Option<KeyCombo<'src>>,
    pub values: Vec<CompatInterpretItem<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::interpret_item))]
pub enum CompatInterpretItem<'src> {
    #[derivative(Debug = "transparent")]
    CompatAction(CompatAction<'src>),
    #[derivative(Debug = "transparent")]
    CompatModifier(CompatModifier<'src>),
    #[derivative(Debug = "transparent")]
    UseModMapMods(UseModMapMods<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_action))]
pub struct CompatAction<'src> {
    pub action: Action<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_modifier))]
pub struct CompatModifier<'src> {
    pub name: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::use_mod_map_mods))]
pub struct UseModMapMods<'src> {
    pub name: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::compat_indicator_block))]
pub struct CompatIndicatorBlock<'src> {
    pub name: StringContent<'src>,
    pub values: Vec<IndicatorItem<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator_item))]
pub enum IndicatorItem<'src> {
    #[derivative(Debug = "transparent")]
    IndicatorNegation(IndicatorNegation<'src>),
    #[derivative(Debug = "transparent")]
    AllowExplicit(AllowExplicit),
    #[derivative(Debug = "transparent")]
    IndicatorDrivesKeyboard(IndicatorDrivesKeyboard),
    #[derivative(Debug = "transparent")]
    IndicatorControls(IndicatorControls<'src>),
    #[derivative(Debug = "transparent")]
    WhichModState(WhichModState<'src>),
    #[derivative(Debug = "transparent")]
    IndicatorModifiers(IndicatorModifiers<'src>),
    #[derivative(Debug = "transparent")]
    IndicatorGroups(IndicatorGroups<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator_negation))]
pub struct IndicatorNegation<'src> {
    pub name: Negation<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::allow_explicit))]
pub struct AllowExplicit;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator_drives_keyboard))]
pub struct IndicatorDrivesKeyboard;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator_controls))]
pub struct IndicatorControls<'src> {
    pub name: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::which_mod_state))]
pub struct WhichModState<'src> {
    pub name: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator_modifiers))]
pub struct IndicatorModifiers<'src> {
    pub name: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::indicator_groups))]
pub struct IndicatorGroups<'src> {
    pub name: Ident<'src>,
}
