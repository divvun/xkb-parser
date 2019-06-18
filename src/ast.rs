use crate::xkb::Rule;
use derivative::Derivative;
use pest::Span;
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
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::xkb_symbols))]
pub struct XkbSymbols<'src> {
    pub name: StringContent<'src>,
    pub symbols: Vec<XkbSymbolsItem<'src>>,
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
    pub name: Vec<Ident<'src>>,
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

// #[derive(Derivative, FromPest, Clone, PartialEq)]
// #[derivative(Debug = "transparent")]
// #[pest_ast(rule(Rule::key_defs))]
// pub struct KeyDefs<'src> {
//     pub values: Vec<KeyDef<'src>>,
// }

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
    pub param_values: Vec<Ident<'src>>,
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

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::symbol))]
pub struct Symbol<'src> {
    #[pest_ast(inner(with(span_into_str)))]
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
#[pest_ast(rule(Rule::string_content))]
pub struct StringContent<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xkb::{Rule, XkbParser};
    use from_pest::FromPest;
    use pest::Parser;
    use std::fmt::Debug;

    #[test]
    fn test_ast_ident() {
        enable_logging();

        assert_parse(Rule::ident, "foobar\n", Ident { content: "foobar" });
    }

    #[test]
    fn test_ast_string() {
        enable_logging();

        assert_parse(
            Rule::string,
            r#""Czech (with <\|> key)""#,
            StringContent { content: r"Czech (with <\|> key)" },
        );
    }

    #[test]
    fn test_ast_what() {
        enable_logging();

        assert_parse(
            Rule::block_modifiers,
            "default partial alphanumeric_keys modifier_keys\n",
            BlockModifiers {
                values: vec![
                    BlockModifier { content: "default" },
                    BlockModifier { content: "partial" },
                    BlockModifier { content: "alphanumeric_keys" },
                    BlockModifier { content: "modifier_keys" },
                ],
            },
        );
    }

    #[test]
    fn test_ast_symbol() {
        enable_logging();

        assert_parse(
            Rule::xkb_symbols_item,
            "key <ESC>  {	[ Escape		]	};",
            XkbSymbolsItem::Key(Key {
                mode: None,
                id: Symbol { content: "ESC" },
                values: vec![KeyValue::KeyNames(KeyNames {
                    values: vec![Ident { content: "Escape" }],
                })],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            "override key <LSGT> {	[ less, greater, bar, brokenbar ] };",
            XkbSymbolsItem::Key(Key {
                mode: Some(KeyMode::KeyModeOverride(KeyModeOverride)),
                id: Symbol { content: "LSGT" },
                values: vec![KeyValue::KeyNames(KeyNames {
                    values: vec![
                        Ident { content: "less" },
                        Ident { content: "greater" },
                        Ident { content: "bar" },
                        Ident { content: "brokenbar" },
                    ],
                })],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            std::str::from_utf8(
                b"key <AE01> { [ U10B78                 ] }; // \xf0\x90\xad\xb8\n\t",
            )
            .unwrap(),
            XkbSymbolsItem::Key(Key {
                mode: None,
                id: Symbol { content: "AE01" },
                values: vec![KeyValue::KeyNames(KeyNames {
                    values: vec![Ident { content: "U10B78" }],
                })],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            std::str::from_utf8(
                b"key <KP7>  { [\tKP_Home,\t\tKP_7,\t\n\t\t\tonehalf,\t\tdead_horn\t] };",
            )
            .unwrap(),
            XkbSymbolsItem::Key(Key {
                mode: None,
                id: Symbol { content: "KP7" },
                values: vec![KeyValue::KeyNames(KeyNames {
                    values: vec![
                        Ident { content: "KP_Home" },
                        Ident { content: "KP_7" },
                        Ident { content: "onehalf" },
                        Ident { content: "dead_horn" },
                    ],
                })],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            std::str::from_utf8(b"key  <KP7> {	[  KP_Home	],	overlay1=<KO7>	};").unwrap(),
            XkbSymbolsItem::Key(Key {
                mode: None,
                id: Symbol { content: "KP7" },
                values: vec![
                    KeyValue::KeyNames(KeyNames { values: vec![Ident { content: "KP_Home" }] }),
                    KeyValue::KeyDefs(KeyDef::OverlayDef(OverlayDef {
                        level: 1,
                        key: Symbol { content: "KO7" },
                    })),
                ],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            "key <PRSC> {\n\ttype= \"PC_ALT_LEVEL2\",\n\tsymbols[Group1]= [ Print, Sys_Req ]\n    };",
            XkbSymbolsItem::Key(Key {
                mode: None,
                id: Symbol { content: "PRSC" },
                values: vec![
                    KeyValue::KeyDefs(KeyDef::TypeDef(TypeDef { group: None, content: "PC_ALT_LEVEL2" }),),
                    KeyValue::KeyDefs(KeyDef::SymbolDef(SymbolDef {
                        group: Group { content: "Group1" },
                        values: KeyNames {
                            values: vec![
                                Ident { content: "Print" },
                                Ident { content: "Sys_Req" },
                            ]
                        }
                    }
                ))],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            r#"key <RALT>  { type[Group1]="TWO_LEVEL",
                  [ ISO_Level3_Shift, Multi_key ] };"#,
            XkbSymbolsItem::Key(Key {
                mode: None,
                id: Symbol { content: "RALT" },
                values: vec![
                    KeyValue::KeyDefs(KeyDef::TypeDef(TypeDef {
                        group: Some(Group { content: "Group1" }),
                        content: "TWO_LEVEL",
                    })),
                    KeyValue::KeyNames(KeyNames {
                        values: vec![
                            Ident { content: "ISO_Level3_Shift" },
                            Ident { content: "Multi_key" },
                        ],
                    }),
                ],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            r#"key <AC01> { [ a,            A,              aogonek,         Aogonek    ], type[Group1] = "EIGHT_LEVEL_ALPHABETIC" };"#,
            XkbSymbolsItem::Key(Key {
                mode: None,
                id: Symbol { content: "AC01" },
                values: vec![
                    KeyValue::KeyNames(KeyNames {
                        values: vec![
                            Ident { content: "a" },
                            Ident { content: "A" },
                            Ident { content: "aogonek" },
                            Ident { content: "Aogonek" },
                        ],
                    }),
                    KeyValue::KeyDefs(KeyDef::TypeDef(TypeDef {
                        group: Some(Group { content: "Group1" }),
                        content: "EIGHT_LEVEL_ALPHABETIC",
                    })),
                ],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            r#"replace key <CAPS> {
                type[Group1] = "ONE_LEVEL",
                symbols[Group1] = [ Caps_Lock ],
                actions[Group1] = [ SetMods(modifiers=Control) ]
            };"#,
            XkbSymbolsItem::Key(Key {
                mode: Some(KeyMode::KeyModeReplace(KeyModeReplace)),
                id: Symbol { content: "CAPS" },
                values: vec![
                    KeyValue::KeyDefs(KeyDef::TypeDef(TypeDef {
                        group: Some(Group { content: "Group1" }),
                        content: "ONE_LEVEL",
                    })),
                    KeyValue::KeyDefs(KeyDef::SymbolDef(SymbolDef {
                        group: Group { content: "Group1" },
                        values: KeyNames { values: vec![Ident { content: "Caps_Lock" }] },
                    })),
                    KeyValue::KeyDefs(KeyDef::ActionsDef(ActionsDef {
                        group: Group { content: "Group1" },
                        values: vec![Action {
                            name: Ident { content: "SetMods" },
                            param_name: Ident { content: "modifiers" },
                            param_values: vec![Ident { content: "Control" }],
                        }],
                    })),
                ],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            r#"name[Group1]="Russian (Sweden, phonetic)";"#,
            XkbSymbolsItem::Name(Name {
                group: Group { content: "Group1" },
                name: StringContent { content: "Russian (Sweden, phonetic)" },
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            r#"key.type[group1]="ALPHABETIC";"#,
            XkbSymbolsItem::KeyType(KeyType {
                group: Some(Group { content: "group1" }),
                name: StringContent { content: "ALPHABETIC" },
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            r#"include "srvr_ctrl(fkey2vt)""#,
            XkbSymbolsItem::Include(Include {
                name: StringContent { content: "srvr_ctrl(fkey2vt)" },
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            "modifier_map Shift  { Shift_L, Shift_R };",
            XkbSymbolsItem::ModifierMap(ModifierMap {
                name: Ident { content: "Shift" },
                values: vec![
                    Modifier::Ident(Ident { content: "Shift_L" }),
                    Modifier::Ident(Ident { content: "Shift_R" }),
                ],
            }),
        );

        assert_parse(
            Rule::xkb_symbols_item,
            "modifier_map Mod4 { <META>, Meta_L, Meta_R };",
            XkbSymbolsItem::ModifierMap(ModifierMap {
                name: Ident { content: "Mod4" },
                values: vec![
                    Modifier::KeyId(Symbol { content: "META" }),
                    Modifier::Ident(Ident { content: "Meta_L" }),
                    Modifier::Ident(Ident { content: "Meta_R" }),
                ],
            }),
        );
    }

    fn enable_logging() {
        let _ = env_logger::builder()
            .filter(None, log::LevelFilter::Trace)
            .default_format_timestamp(false)
            .is_test(true)
            .try_init();
    }

    fn assert_parse<'i, T>(r: Rule, input: &'i str, expected: T)
    where
        T: FromPest<'i, Rule = Rule> + PartialEq + Debug,
        <T as FromPest<'i>>::FatalError: Debug,
    {
        let mut parse_tree = match XkbParser::parse(r, input) {
            Ok(parse_tree) => {
                println!("parse tree = {:#?}", parse_tree);
                parse_tree
            }
            Err(e) => {
                panic!("Failed to parse `{}` as {:?}: `{}`", input, r, e);
            }
        };

        let syntax_tree = T::from_pest(&mut parse_tree).expect("infallible");
        println!("syntax tree = {:#?}", syntax_tree);

        assert_eq!(syntax_tree, expected);
    }
}
