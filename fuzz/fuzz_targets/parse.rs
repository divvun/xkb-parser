#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate xkb_parser;

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = xkb_parser::parse(&data);
    }
});
