use kbd_parser::parse;
use std::{
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

#[test]
fn parse_fixtures() -> Result<(), Box<dyn Error>> {
    let xkb_files = fs::read_dir("tests/fixtures")?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| x.is_file())
        .filter(|x| x.extension() == Some(OsStr::new("xkb")));
    parse_files(xkb_files)
}

#[test]
fn parse_x11_fixtures() -> Result<(), Box<dyn Error>> {
    let xkb_files = fs::read_dir("tests/fixtures/x11/symbols")?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| x.is_file());
    parse_files(xkb_files)
}

fn parse_files(xkb_files: impl Iterator<Item = PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut failed = 0;
    for file in xkb_files {
        if let Err(e) = parse_one_file(&file) {
            panic!("error parsing file {}:\n{}", file.display(), e);
            failed += 1;
        }
    }
    if failed > 0 {
        Err(format!("{} tests failed", failed))?
    }
    Ok(())
}

fn parse_one_file(file_name: &Path) -> Result<(), Box<dyn Error>> {
    eprintln!("file: {}", file_name.display());
    let file = std::fs::read_to_string(&file_name)?;
    let _ = parse(&file)?;
    Ok(())
}
