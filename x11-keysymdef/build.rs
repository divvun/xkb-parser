use pest::Parser;
use std::{
    env,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    from_header();
    from_tsv();
}

fn from_tsv() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("from_tsv.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let tsv = read_to_string("src/keysym.tsv").unwrap();
    let mapping: Vec<_> = tsv
        .lines()
        .skip(47)
        .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        .map(|x| (x[0], x[1]))
        .collect();

    writeln!(&mut file, "lazy_static::lazy_static! {{").unwrap();

    writeln!(
        &mut file,
        "static ref NAME_TO_CODEPOINT: std::collections::HashMap<&'static str, char> = {{"
    )
    .unwrap();
    writeln!(&mut file, "let mut map = std::collections::HashMap::new();").unwrap();
    for (name, code) in &mapping {
        writeln!(&mut file, r#"map.entry("{}").or_insert('\u{{{}}}');"#, name, code).unwrap();
    }
    writeln!(&mut file, "map").unwrap();
    writeln!(&mut file, "}};").unwrap();

    writeln!(
        &mut file,
        "static ref CODEPOINT_TO_NAME: std::collections::HashMap<char, &'static str> = {{"
    )
    .unwrap();
    writeln!(&mut file, "let mut map = std::collections::HashMap::new();").unwrap();

    for (name, code) in &mapping {
        writeln!(&mut file, r#"map.entry('\u{{{}}}').or_insert("{}");"#, code, name).unwrap();
    }

    writeln!(&mut file, "map").unwrap();
    writeln!(&mut file, "}};").unwrap();

    writeln!(&mut file, "}}").unwrap();
}

fn from_header() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("from_header.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let header_file = read_to_string("src/keysymdef.h").unwrap();
    let definitions = read_defs(&header_file).unwrap();

    writeln!(&mut file, "lazy_static::lazy_static! {{").unwrap();

    writeln!(
        &mut file,
        "static ref NAME_TO_KEYSYM: std::collections::HashMap<&'static str, u32> = {{"
    )
    .unwrap();
    writeln!(&mut file, "let mut map = std::collections::HashMap::new();").unwrap();
    for Define { name, code } in &definitions {
        writeln!(&mut file, r#"map.entry("{}").or_insert({});"#, name, code).unwrap();
    }
    writeln!(&mut file, "map").unwrap();
    writeln!(&mut file, "}};").unwrap();

    writeln!(
        &mut file,
        "static ref KEYSYM_TO_NAME: std::collections::HashMap<u32, &'static str> = {{"
    )
    .unwrap();
    writeln!(&mut file, "let mut map = std::collections::HashMap::new();").unwrap();
    for Define { name, code } in &definitions {
        writeln!(&mut file, r#"map.entry({}).or_insert("{}");"#, code, name).unwrap();
    }
    writeln!(&mut file, "map").unwrap();
    writeln!(&mut file, "}};").unwrap();

    writeln!(&mut file, "}}").unwrap();
}

fn read_defs(input: &str) -> Result<Vec<Define>, Box<dyn std::error::Error>> {
    let items = parser::CDefines::parse(parser::Rule::file, input)?;

    let defs = items
        .filter(|x| x.as_rule() == parser::Rule::file)
        .next()
        .ok_or("no file parsed :O")?
        .into_inner()
        .filter(|x| x.as_rule() == parser::Rule::def)
        .map(|x| {
            let mut inner = x.into_inner();
            let mut name = inner.next().unwrap().as_span().as_str();
            if name.starts_with("XK_") {
                name = &name[3..];
            }
            let code = inner.next().unwrap().as_span().as_str();

            Define { name, code }
        })
        .collect();

    Ok(defs)
}

struct Define<'a> {
    name: &'a str,
    code: &'a str,
}

mod parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "c_defines.pest"]
    pub struct CDefines;
}
