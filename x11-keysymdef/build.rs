use pest::Parser;
use std::{
    env,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
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
        writeln!(&mut file, r#"map.insert({}, "{}");"#, code, name).unwrap();
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
            Define {
                name: inner.next().unwrap().as_span().as_str(),
                code: inner.next().unwrap().as_span().as_str(),
            }
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
