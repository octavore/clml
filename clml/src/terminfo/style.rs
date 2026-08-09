//! Lazy constants representing the ANSI codes for setting terminal styles, like bold, underline,
//! etc...
//!
//! Terminfo is used internally to guess the right codes.

use std::sync::LazyLock;

use terminfo_crate::{capability as cap, expand, Capability};

use crate::terminfo::TERMINFO;

pub static CLEAR: LazyLock<String> = LazyLock::new(style::<cap::ExitAttributeMode>);
pub static BOLD: LazyLock<String> = LazyLock::new(style::<cap::EnterBoldMode>);
pub static DIM: LazyLock<String> = LazyLock::new(style::<cap::EnterDimMode>);
pub static BLINK: LazyLock<String> = LazyLock::new(style::<cap::EnterBlinkMode>);
pub static ITALICS: LazyLock<String> = LazyLock::new(style::<cap::EnterItalicsMode>);
pub static REVERSE: LazyLock<String> = LazyLock::new(style::<cap::EnterReverseMode>);
pub static UNDERLINE: LazyLock<String> = LazyLock::new(style::<cap::EnterUnderlineMode>);
pub static NO_ITALICS: LazyLock<String> = LazyLock::new(style::<cap::ExitItalicsMode>);
pub static NO_UNDERLINE: LazyLock<String> = LazyLock::new(style::<cap::ExitUnderlineMode>);

/// Gets the ANSI code which sets the given style `T`.
fn style<'a, T>() -> String
where
    T: Capability<'a> + AsRef<[u8]>,
{
    expand0::<'a, T>().unwrap_or_default()
}

/// Shortcut function for the `style()` function.
fn expand0<'a, T>() -> Option<String>
where
    T: Capability<'a> + AsRef<[u8]>,
{
    let info = (*TERMINFO).as_ref()?;
    let e = expand!(info.get::<T>()?.as_ref()).ok()?;
    let s = std::str::from_utf8(&e).ok()?;
    Some(s.to_owned())
}
