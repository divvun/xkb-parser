use from_pest::FromPest;
use jwalk::WalkDir;
use kbd_parser::{Rule, Xkb, XkbParser};
use log::{debug, error};
use pest::Parser;
use rayon::prelude::*;
use std::{
    error::Error as StdError,
    ffi::OsStr,
    path::{Path, PathBuf},
};

type Error = Box<dyn StdError + Send + Sync>;

#[test]
fn parse_custom_fixtures() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/custom")
            .into_iter()
            .par_bridge()
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .filter(|x| x.is_file())
            .filter(|x| x.extension() == Some(OsStr::new("xkb"))),
    )
}

#[test]
fn parse_kbdgen_fixtures() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/kbdgen")
            .into_iter()
            .par_bridge()
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .filter(|x| x.is_file())
            .filter(|x| x.extension() == Some(OsStr::new("xkb"))),
    )
}

#[test]
fn parse_x11_symbols() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/x11/symbols")
            .into_iter()
            .par_bridge()
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .filter(|x| x.is_file()),
    )
}

#[test]
fn parse_x11_keycodes() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/x11/keycodes")
            .into_iter()
            .par_bridge()
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .filter(|x| x.is_file()),
    )
}

#[test]
fn parse_x11_types() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/x11/types")
            .into_iter()
            .par_bridge()
            .filter_map(|x| x.ok())
            .map(|x| x.path())
            .filter(|x| x.is_file()),
    )
}

fn parse_files(xkb_files: impl ParallelIterator<Item = PathBuf>) -> Result<(), Error> {
    let failed: usize = xkb_files
        .filter(|f| f.extension() != Some(OsStr::new("json")))
        .filter(|f| f.extension() != Some(OsStr::new("ron")))
        .filter(|f| f.file_name() != Some(OsStr::new("README")))
        .map(|f: PathBuf| {
            if std::env::var("VERBOSE_PARSE_ERRORS") == Ok("1".to_owned()) {
                debug!("next: {}", f.display());
            }
            let res = parse_one_file(&f);
            (f, res)
        })
        .filter_map(|(f, x)| x.err().map(|e| (f, e)))
        .inspect(|(file_name, error)| {
            if std::env::var("VERBOSE_PARSE_ERRORS") == Ok("1".to_owned()) {
                error!("error parsing file {}:\n{}", file_name.display(), error);
            } else {
                error!("failed: {}", file_name.display());
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
    std::fs::write(file_name.with_extension("debug.ron"), format!("{:#?}", parse_tree))?;
    let _ =
        Xkb::from_pest(&mut parse_tree).map_err(|e| format!("ast generation failed: {:?}", e))?;
    Ok(())
}

fn enable_logging() {
    let _ = env_logger::builder()
        .filter(None, log::LevelFilter::Trace)
        .default_format_timestamp(false)
        .is_test(true)
        .try_init();
}
