use crate::xkb::Rule;
use derivative::Derivative;
use pest::Span;
use pest_ast::FromPest;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

fn from_hex_str(span: Span) -> u64 {
    u64::from_str_radix(span.as_str(), 16).expect("valid hex digit")
}

#[derive(Derivative, FromPest)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::default))]
pub struct Default<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'src str,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::include))]
pub struct Include<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'src str,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::name))]
pub struct Name<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub group: &'src str,
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'src str,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key))]
pub struct Key<'src> {
    #[pest_ast(inner(with(from_hex_str)))]
    pub id: u64,
    pub names: Vec<KeyName<'src>>,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_name))]
pub struct KeyName<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'src str,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbol))]
pub enum Symbol<'src> {
    #[derivative(Debug = "transparent")]
    Include(Include<'src>),
    #[derivative(Debug = "transparent")]
    Name(Name<'src>),
    #[derivative(Debug = "transparent")]
    Key(Key<'src>),
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbols))]
pub struct Symbols<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'src str,
    pub symbols: Vec<Symbol<'src>>,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::definition))]
pub struct Definition<'src> {
    pub default: Vec<Default<'src>>,
    pub symbols: Symbols<'src>,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::file))]
pub struct File<'src> {
    pub definitions: Vec<Definition<'src>>,
    #[derivative(Debug = "ignore")]
    eoi: EOI,
}

#[derive(Derivative, FromPest)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

#[test]
fn test_ast() {
    use from_pest::FromPest;
    use pest::Parser;

    let source = "default partial alphanumeric_keys\n";

    let mut parse_tree =
        crate::xkb::XkbParser::parse(Rule::default, source).expect("parse success");
    println!("parse tree = {:#?}", parse_tree);

    let syntax_tree = Default::from_pest(&mut parse_tree).expect("infallible");
    println!("syntax tree = {:#?}", syntax_tree);
}
