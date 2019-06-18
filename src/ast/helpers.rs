use pest::Span;

pub(crate) fn span_into_str(span: Span) -> &str {
    span.as_str()
}
