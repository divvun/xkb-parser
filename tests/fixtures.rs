use from_pest::FromPest;
use kbd_parser::{Rule, Xkb, XkbParser};
use pest::Parser;
use rayon::prelude::*;
use std::{
    error::Error as StdError,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

type Error = Box<dyn StdError + Send + Sync>;

#[test]
fn parse_fixtures() -> Result<(), Error> {
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
fn parse_x11_fixtures() -> Result<(), Error> {
    let _ = env_logger::builder().is_test(true).try_init();

    let xkb_files = fs::read_dir("tests/fixtures/x11/symbols")?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| x.is_file());
    parse_files(xkb_files)
}

fn parse_files(xkb_files: impl Iterator<Item = PathBuf> + Send) -> Result<(), Error> {
    let failed: usize = xkb_files
        .filter(|f| f.extension() != Some(OsStr::new("json")))
        .par_bridge()
        .map(|f: PathBuf| {
            let res = parse_one_file(&f);
            (f, res)
        })
        .filter_map(|(f, x)| x.err().map(|e| (f, e)))
        .inspect(|(file_name, error)| {
            if std::env::var("VERBOSE_PARSE_ERRORS") == Ok("1".to_owned()) {
                eprintln!("error parsing file {}:\n{}", file_name.display(), error);
            } else {
                eprintln!(" failed: {}", file_name.display());
            }
        })
        .count();

    if failed > 0 {
        panic!("{} tests failed -- Set `VERBOSE_PARSE_ERRORS=1` for error messages", failed);
    }
    Ok(())
}

fn parse_one_file(file_name: &Path) -> Result<(), Error> {
    let file = std::fs::read_to_string(&file_name)?;
    let mut parse_tree = XkbParser::parse(Rule::file, &file)?;
    std::fs::write(file_name.with_extension("debug.json"), parse_tree.to_json())?;
    let _ =
        Xkb::from_pest(&mut parse_tree).map_err(|e| format!("ast generation failed: {:?}", e))?;
    Ok(())
}
