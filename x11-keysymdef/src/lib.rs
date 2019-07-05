//! The "X11 Window System Protocol" standard defines in Appendix A the keysym
//! codes. These 29-bit integer values identify characters or functions
//! associated with each key (e.g., via the visible engraving) of a keyboard
//! layout.
//!
//! This library contains mappings between mnemonic macro names and these keysym
//! codes.

#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Look up a keysym code by the mnemonic macro name
pub fn string_to_keysym(name: &str) -> Option<u32> {
    NAME_TO_KEYSYM.get(&name).copied()
}

/// Look up a mnemonic macro name by the keysym code
///
/// Note that one code can have multiple names. The first one in the definition
/// file will be returned.
pub fn keysym_to_string(keysym: u32) -> Option<&'static str> {
    KEYSYM_TO_NAME.get(&keysym).copied()
}

#[test]
fn what_code_is_this() {
    assert!(string_to_keysym("XK_Uhorngrave").is_some());
}

#[test]
fn how_do_you_call_this() {
    assert!(keysym_to_string(0x10004b3).is_some());
}
