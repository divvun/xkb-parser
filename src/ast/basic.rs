use crate::{ast::*, xkb::Rule};
use derivative::Derivative;
use pest_ast::FromPest;
use shrinkwraprs::Shrinkwrap;

#[derive(Derivative, Shrinkwrap, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, Shrinkwrap, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::symbol_name))]
pub struct Symbol<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, Shrinkwrap, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::group))]
pub struct Group<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, Shrinkwrap, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_combo))]
pub struct KeyCombo<'src> {
    pub content: Vec<Ident<'src>>,
}

#[derive(Derivative, Shrinkwrap, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
pub struct StringContent<'src> {
    pub content: &'src str,
}

impl<'src> From<StringContent<'src>> for String {
    fn from(x: StringContent<'src>) -> String {
        x.content.to_owned()
    }
}

impl<'a, 'src> From<&'a StringContent<'src>> for String {
    fn from(x: &StringContent) -> String {
        x.content.to_owned()
    }
}

impl<'a> ::from_pest::FromPest<'a> for StringContent<'a> {
    type Rule = Rule;
    type FatalError = ::from_pest::Void;

    fn from_pest(
        pest: &mut ::from_pest::pest::iterators::Pairs<'a, Self::Rule>,
    ) -> ::std::result::Result<Self, ::from_pest::ConversionError<::from_pest::Void>> {
        let mut clone = pest.clone();
        let pair = clone.next().ok_or(::from_pest::ConversionError::NoMatch)?;
        if pair.as_rule() == Rule::string {
            let mut inner = pair.into_inner();
            let inner = &mut inner;
            let mut inner = inner.clone();
            let first_item = inner.next();
            let this = if let Some(item) = first_item {
                if item.as_rule() == Rule::string_content {
                    StringContent { content: item.as_str() }
                } else {
                    return Err(::from_pest::ConversionError::NoMatch);
                }
            } else {
                return Err(::from_pest::ConversionError::NoMatch);
            };
            if inner.next().is_some() {
                log::trace!("extraneous token, current node: `string`");
                return Err(::from_pest::ConversionError::NoMatch);
            }
            *pest = clone;
            Ok(this)
        } else {
            Err(::from_pest::ConversionError::NoMatch)
        }
    }
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::negation))]
pub struct Negation<'src> {
    pub content: Ident<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::EOI))]
pub(crate) struct EOI;
