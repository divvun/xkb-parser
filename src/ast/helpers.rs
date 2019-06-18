use derivative::Derivative;
use pest::Span;

pub(crate) fn span_into_str(span: Span) -> &str {
    span.as_str()
}

pub(crate) fn parse_u64(span: Span) -> u64 {
    let input = span.as_str();
    input
        .parse()
        .map_err(|e| {
            log::error!("Failed to read `{}` as u64: {}", input, e);
            e
        })
        .unwrap()
}

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
pub struct Debug<'src> {
    pub content: &'src str,
}

impl<'a> ::from_pest::FromPest<'a> for Debug<'a> {
    type Rule = crate::xkb::Rule;
    type FatalError = ::from_pest::Void;

    fn from_pest(
        pest: &mut ::from_pest::pest::iterators::Pairs<'a, Self::Rule>,
    ) -> ::std::result::Result<Self, ::from_pest::ConversionError<::from_pest::Void>> {
        let mut clone = pest.clone();
        let pair = clone.next().ok_or(::from_pest::ConversionError::NoMatch)?;
        *pest = clone;
        Ok(Debug { content: pair.as_str() })
    }
}
