use kbd_parser::parse;
use std::{error::Error, ffi::OsStr, fs, path::Path};

#[test]
fn parse_fixtures() -> Result<(), Box<dyn Error>> {
    let xkb_files = fs::read_dir("tests/fixtures")?
        .filter_map(|x| x.ok())
        .filter(|x| x.path().extension() == Some(OsStr::new("xkb")));
    let mut failed = false;
    for file in xkb_files {
        if let Err(e) = parse_one_file(&file.path()) {
            eprintln!("{}", e);
            failed = true;
        }
    }
    if failed {
        Err("failed")?
    } else {
        Ok(())
    }
}

fn parse_one_file(file_name: &Path) -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(&file_name)?;
    let _ = parse(&file)?;
    Ok(())
}
