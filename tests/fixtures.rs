#![type_length_limit = "2893838"]

use from_pest::FromPest;
use kbd_parser::{Rule, Xkb, XkbParser};
use log::{debug, error};
use pest::Parser;
use rayon::prelude::*;
use std::{
    error::Error as StdError,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

type Error = Box<dyn StdError + Send + Sync>;

#[test]
fn parse_custom_fixtures() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/custom")
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .filter(|x| x.extension() == Some(OsStr::new("xkb")))
            .collect(),
    )
}

#[test]
fn parse_kbdgen_fixtures() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/kbdgen")
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .filter(|x| x.extension() == Some(OsStr::new("xkb")))
            .collect(),
    )
}

#[test]
fn parse_x11_symbols() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/x11/symbols")
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .collect(),
    )
}

#[test]
fn parse_x11_keycodes() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/x11/keycodes")
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .collect(),
    )
}

#[test]
fn parse_x11_types() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/x11/types")
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .collect(),
    )
}

#[test]
fn parse_x11_geometry() -> Result<(), Error> {
    enable_logging();

    log::info!("geometry");

    parse_files(
        WalkDir::new("tests/fixtures/x11/geometry")
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .collect(),
    )
}

#[test]
#[ignore]
fn parse_x11_rules() -> Result<(), Error> {
    enable_logging();

    parse_files(
        WalkDir::new("tests/fixtures/x11/rules")
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .collect(),
    )
}

fn parse_files(xkb_files: Vec<PathBuf>) -> Result<(), Error> {
    let failed: usize = xkb_files
        .into_par_iter()
        .filter(|f| f.extension() != Some(OsStr::new("json")))
        .filter(|f| f.extension() != Some(OsStr::new("ron")))
        .filter(|f| f.extension() != Some(OsStr::new("xml")))
        .filter(|f| f.file_name() != Some(OsStr::new("README")))
        .map(|f: PathBuf| {
            debug!("next: {}", f.display());
            let res = parse_one_file(&f);
            (f, res)
        })
        .filter_map(|(f, x)| x.err().map(|e| (f, e)))
        .inspect(|(file_name, error)| {
            debug!("error parsing file {}:\n{}", file_name.display(), error);
            error!("failed: {}", file_name.display());
        })
        .count();

    if failed > 0 {
        panic!("{} tests failed -- Set `VERBOSE_PARSE_ERRORS=1` for error messages", failed);
    }
    Ok(())
}

fn parse_one_file(file_name: &Path) -> Result<(), Error> {
    let file = std::fs::read_to_string(&file_name)?;
    log::trace!("  read {}", file_name.display());
    let mut parse_tree = XkbParser::parse(Rule::file, &file)?;
    log::trace!("parsed {}", file_name.display());
    std::fs::write(file_name.with_extension("debug.ron"), format!("{:#?}", parse_tree))?;
    let _ =
        Xkb::from_pest(&mut parse_tree).map_err(|e| format!("ast generation failed: {:?}", e))?;
    log::trace!(" asted {}", file_name.display());
    Ok(())
}

fn enable_logging() {
    let verbosity = std::env::var("VERBOSE_PARSE_ERRORS").ok().and_then(|x| x.parse::<i16>().ok());
    let level = match verbosity {
        None | Some(0) => log::LevelFilter::Warn,
        Some(1) => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let _ = env_logger::builder()
        .filter(None, level)
        .default_format_timestamp(false)
        .is_test(true)
        .try_init();
}
