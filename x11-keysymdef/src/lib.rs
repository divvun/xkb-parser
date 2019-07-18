//! The "X11 Window System Protocol" standard defines in Appendix A the keysym
//! codes. These 29-bit integer values identify characters or functions
//! associated with each key (e.g., via the visible engraving) of a keyboard
//! layout.
//!
//! This library contains mappings between mnemonic macro names and these keysym
//! codes.

#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/from_header.rs"));
include!(concat!(env!("OUT_DIR"), "/from_tsv.rs"));

/// Look up a keysym code by the mnemonic macro name
pub fn name_to_keysym(name: &str) -> Option<u32> {
    NAME_TO_KEYSYM.get(&name).copied()
}

/// Look up a unicode char (unicode code point) by the mnemonic macro name
pub fn name_to_codepoint(name: &str) -> Option<char> {
    NAME_TO_CODEPOINT.get(&name).copied()
}

/// Look up a mnemonic macro name by the keysym code
///
/// Note that one code can have multiple names. The first one in the definition
/// file will be returned.
pub fn keysym_to_name(keysym: u32) -> Option<&'static str> {
    KEYSYM_TO_NAME.get(&keysym).copied()
}

/// Look up a mnemonic macro name by the keysym code
///
/// Note that one code can have multiple names. The first one in the definition
/// file will be returned.
pub fn char_to_name(codepoint: char) -> Option<&'static str> {
    CODEPOINT_TO_NAME.get(&codepoint).copied()
}

/// Look up unicode codepoint for keysym
pub fn keysym_to_char(keysym: u32) -> Option<char> {
    keysym_to_name(keysym).and_then(|name| name_to_codepoint(name))
}

/// Look up keysym for unicode codepoint
pub fn char_to_keysym(codepoint: char) -> Option<u32> {
    char_to_name(codepoint).and_then(|name| name_to_keysym(name))
}

#[test]
fn what_code_is_this() {
    assert!(name_to_keysym("Uhorngrave").is_some());
    assert!(name_to_codepoint("Uhorngrave").is_some());
    assert!(char_to_keysym('\u{1EEA}').is_some());
}

#[test]
fn how_do_you_call_this() {
    assert!(keysym_to_name(0x10004b3).is_some());
    assert!(keysym_to_char(0x10004b3).is_some());
    assert!(char_to_name('\u{1EEA}').is_some());
}
