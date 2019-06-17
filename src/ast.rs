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
    pub what: What<'src>,
    pub symbols: Symbols<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::what))]
pub struct What<'src> {
    pub how: How,
    pub name: Vec<Ident<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::how))]
pub enum How {
    DefaultPartial(DefaultPartial),
    Partial(Partial),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::default_partial))]
pub struct DefaultPartial;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::partial))]
pub struct Partial;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbols))]
pub struct Symbols<'src> {
    pub name: StringContent<'src>,
    pub symbols: Vec<Symbol<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbol))]
pub enum Symbol<'src> {
    #[derivative(Debug = "transparent")]
    Include(Include<'src>),
    #[derivative(Debug = "transparent")]
    Name(Name<'src>),
    #[derivative(Debug = "transparent")]
    Key(Key<'src>),
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
#[pest_ast(rule(Rule::name))]
pub struct Name<'src> {
    pub group: Ident<'src>,
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key))]
pub struct Key<'src> {
    pub id: KeyId<'src>,
    pub values: KeyValues<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_id))]
pub struct KeyId<'src> {
    pub content: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_values))]
pub enum KeyValues<'src> {
    KeyNames(KeyNames<'src>),
    KeyDefs(KeyDefs<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_names))]
pub struct KeyNames<'src> {
    pub names: Vec<Ident<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_defs))]
pub enum KeyDefs<'src> {
    TypeDef(TypeDef<'src>),
    SymbolDef(SymbolDef<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::type_def))]
pub struct TypeDef<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbol_def))]
pub struct SymbolDef<'src> {
    pub name: Ident<'src>,
    pub content: KeyNames<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key_name))]
pub struct ModifierMap<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
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
    fn test_ast_how() {
        enable_logging();

        assert_parse(Rule::how, "default partial\n", How::DefaultPartial(DefaultPartial));
        assert_parse(Rule::how, "partial\n", How::Partial(Partial));
    }

    #[test]
    fn test_ast_ident() {
        enable_logging();

        assert_parse(Rule::ident, "foobar\n", Ident { content: "foobar" });
    }

    #[test]
    fn test_ast_what() {
        enable_logging();

        assert_parse(
            Rule::what,
            "default partial alphanumeric_keys\n",
            What {
                how: How::DefaultPartial(DefaultPartial),
                name: vec![Ident { content: "alphanumeric_keys" }],
            },
        );

        assert_parse(
            Rule::what,
            "default partial alphanumeric_keys modifier_keys\n",
            What {
                how: How::DefaultPartial(DefaultPartial),
                name: vec![
                    Ident { content: "alphanumeric_keys" },
                    Ident { content: "modifier_keys" },
                ],
            },
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
        let mut parse_tree = XkbParser::parse(r, input).expect("parse success");
        println!("parse tree = {:#?}", parse_tree);

        let syntax_tree = T::from_pest(&mut parse_tree).expect("infallible");
        println!("syntax tree = {:#?}", syntax_tree);

        assert_eq!(syntax_tree, expected);
    }
}
