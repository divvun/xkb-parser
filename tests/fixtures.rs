use from_pest::FromPest;
use kbd_parser::{Rule, Xkb, XkbParser};
use pest::Parser;
use std::{
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

#[test]
fn parse_fixtures() -> Result<(), Box<dyn Error>> {
    let _ = env_logger::builder().is_test(true).try_init();

    let xkb_files = fs::read_dir("tests/fixtures")?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| x.is_file())
        .filter(|x| x.extension() == Some(OsStr::new("xkb")));
    parse_files(xkb_files)
}

#[test]
#[ignore]
fn parse_x11_fixtures() -> Result<(), Box<dyn Error>> {
    let _ = env_logger::builder().is_test(true).try_init();

    let xkb_files = fs::read_dir("tests/fixtures/x11/symbols")?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| x.is_file());
    parse_files(xkb_files)
}

fn parse_files(xkb_files: impl Iterator<Item = PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut failed = 0;
    for file_name in xkb_files.filter(|x| x.extension() != Some(OsStr::new("json"))) {
        eprintln!("   next: {}", file_name.display());
        if let Err(e) = parse_one_file(&file_name) {
            if std::env::var("VERBOSE_PARSE_ERRORS") == Ok("1".to_owned()) {
                eprintln!("error parsing file {}:\n{}", file_name.display(), e);
            } else {
                eprintln!(" failed: {}", file_name.display());
            }
            failed += 1;
        }
    }
    if failed > 0 {
        panic!("{} tests failed -- Set `VERBOSE_PARSE_ERRORS=1` for error messages", failed);
    }
    Ok(())
}

fn parse_one_file(file_name: &Path) -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(&file_name)?;
    let mut parse_tree = XkbParser::parse(Rule::file, &file)?;
    std::fs::write(file_name.with_extension("debug.json"), parse_tree.to_json())?;
    let _ =
        Xkb::from_pest(&mut parse_tree).map_err(|e| format!("ast generation failed: {:?}", e))?;
    Ok(())
}
