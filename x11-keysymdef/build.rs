use std::{
    env,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    from_json();
}

fn from_json() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("mapping.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let json = read_to_string("src/keysym.json").unwrap();
    let keysyms: Keysyms = serde_json::from_str(&json).unwrap();
    let records = keysyms.records;

    // First off, let's define what a record looks like. Note that all fields
    // are static!
    writeln!(
        &mut file,
        "#[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Record {{
        pub keysym: u32,
        pub unicode: char,
        pub names: &'static [&'static str],
    }}"
    )
    .unwrap();

    // We'll now define a `static` array of records. References into this array
    // will be the values in the hashmaps below.
    writeln!(&mut file, "static RECORDS: &[Record] = &[").unwrap();
    for r in &records {
        writeln!(
            &mut file,
            "Record {{
            keysym: {},
            unicode: '\\u{{{:x}}}',
            names: &[",
            r.keysym, r.unicode
        )
        .unwrap();

        for name in &r.names {
            writeln!(&mut file, r#""{}","#, name).unwrap();
        }

        writeln!(&mut file, "]}},").unwrap();
    }
    writeln!(&mut file, "];").unwrap();

    // We can't easily build hashmaps at compile-time, so let's use lazy_static here.
    writeln!(&mut file, "::lazy_static::lazy_static! {{").unwrap();

    // First map: Index by name
    writeln!(
        &mut file,
        "static ref BY_NAMES: ::std::collections::HashMap<&'static str, &'static Record> = {{"
    )
    .unwrap();
    writeln!(&mut file, "let mut map = ::std::collections::HashMap::new();").unwrap();
    for (index, r) in records.iter().enumerate() {
        // multiple names per record possible, add one entry for each
        for name in &r.names {
            // Use unchecked access here because we just built the array of records
            // with exactly these indices.
            writeln!(
                &mut file,
                r#"map.entry("{}").or_insert(unsafe {{ RECORDS.get_unchecked({}) }});"#,
                name, index
            )
            .unwrap();
        }
    }
    writeln!(&mut file, "map").unwrap();
    writeln!(&mut file, "}};").unwrap();

    // Next up: Index by codepoint
    writeln!(
        &mut file,
        "static ref BY_CODEPOINT: std::collections::HashMap<char, &'static Record> = {{"
    )
    .unwrap();
    writeln!(&mut file, "let mut map = std::collections::HashMap::new();").unwrap();

    for (index, r) in records.iter().enumerate() {
        writeln!(
            &mut file,
            r#"map.entry('\u{{{:x}}}').or_insert(unsafe {{ RECORDS.get_unchecked({}) }});"#,
            r.unicode, index
        )
        .unwrap();
    }
    writeln!(&mut file, "map").unwrap();
    writeln!(&mut file, "}};").unwrap();

    // Last but not least: Index by keysym
    writeln!(
        &mut file,
        "static ref BY_KEYSYM: std::collections::HashMap<u32, &'static Record> = {{"
    )
    .unwrap();
    writeln!(&mut file, "let mut map = std::collections::HashMap::new();").unwrap();

    for (index, r) in records.iter().enumerate() {
        writeln!(
            &mut file,
            r#"map.entry({}).or_insert(unsafe {{ RECORDS.get_unchecked({}) }});"#,
            r.keysym, index
        )
        .unwrap();
    }
    writeln!(&mut file, "map").unwrap();
    writeln!(&mut file, "}};").unwrap();

    // End of lazy_static
    writeln!(&mut file, "}}").unwrap();
}

#[derive(serde::Deserialize)]
struct Keysyms {
    records: Vec<Record>,
}

#[derive(serde::Deserialize)]
struct Record {
    keysym: u32,
    unicode: u32,
    names: Vec<String>,
}
