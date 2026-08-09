//! Lazy constants representing the ANSI codes for setting terminal colors.
//!
//! Terminfo is used internally to guess the right codes.

use std::sync::LazyLock;

use terminfo_crate::{capability as cap, expand, Capability};

use crate::terminfo::TERMINFO;

#[rustfmt::skip]
mod constants {
    use super::*;

    pub static BLACK: LazyLock<String> = LazyLock::new(|| foreground(0));
    pub static RED: LazyLock<String> = LazyLock::new(|| foreground(1));
    pub static GREEN: LazyLock<String> = LazyLock::new(|| foreground(2));
    pub static YELLOW: LazyLock<String> = LazyLock::new(|| foreground(3));
    pub static BLUE: LazyLock<String> = LazyLock::new(|| foreground(4));
    pub static MAGENTA: LazyLock<String> = LazyLock::new(|| foreground(5));
    pub static CYAN: LazyLock<String> = LazyLock::new(|| foreground(6));
    pub static WHITE: LazyLock<String> = LazyLock::new(|| foreground(7));

    pub static BRIGHT_BLACK: LazyLock<String> = LazyLock::new(|| foreground(8));
    pub static BRIGHT_RED: LazyLock<String> = LazyLock::new(|| foreground(9));
    pub static BRIGHT_GREEN: LazyLock<String> = LazyLock::new(|| foreground(10));
    pub static BRIGHT_YELLOW: LazyLock<String> = LazyLock::new(|| foreground(11));
    pub static BRIGHT_BLUE: LazyLock<String> = LazyLock::new(|| foreground(12));
    pub static BRIGHT_MAGENTA: LazyLock<String> = LazyLock::new(|| foreground(13));
    pub static BRIGHT_CYAN: LazyLock<String> = LazyLock::new(|| foreground(14));
    pub static BRIGHT_WHITE: LazyLock<String> = LazyLock::new(|| foreground(15));

    pub static BG_BLACK: LazyLock<String> = LazyLock::new(|| background(0));
    pub static BG_RED: LazyLock<String> = LazyLock::new(|| background(1));
    pub static BG_GREEN: LazyLock<String> = LazyLock::new(|| background(2));
    pub static BG_YELLOW: LazyLock<String> = LazyLock::new(|| background(3));
    pub static BG_BLUE: LazyLock<String> = LazyLock::new(|| background(4));
    pub static BG_MAGENTA: LazyLock<String> = LazyLock::new(|| background(5));
    pub static BG_CYAN: LazyLock<String> = LazyLock::new(|| background(6));
    pub static BG_WHITE: LazyLock<String> = LazyLock::new(|| background(7));

    pub static BG_BRIGHT_BLACK: LazyLock<String> = LazyLock::new(|| background(8));
    pub static BG_BRIGHT_RED: LazyLock<String> = LazyLock::new(|| background(9));
    pub static BG_BRIGHT_GREEN: LazyLock<String> = LazyLock::new(|| background(10));
    pub static BG_BRIGHT_YELLOW: LazyLock<String> = LazyLock::new(|| background(11));
    pub static BG_BRIGHT_BLUE: LazyLock<String> = LazyLock::new(|| background(12));
    pub static BG_BRIGHT_MAGENTA: LazyLock<String> = LazyLock::new(|| background(13));
    pub static BG_BRIGHT_CYAN: LazyLock<String> = LazyLock::new(|| background(14));
    pub static BG_BRIGHT_WHITE: LazyLock<String> = LazyLock::new(|| background(15));
}

pub use constants::*;

/// Gets the ANSI code which sets the foreground color to the given color (0 to 15 included).
fn foreground(v: u8) -> String {
    debug_assert!(v < 16);
    expand1_string::<cap::SetAForeground>(v)
}

/// Gets the ANSI code which sets the background color to the given color (0 to 15 included).
fn background(v: u8) -> String {
    debug_assert!(v < 16);
    expand1_string::<cap::SetABackground>(v)
}

/// Shortcut function for the `foreground()` and `background()` functions.
fn expand1_string<'a, T>(v: u8) -> String
where
    T: Capability<'a> + AsRef<[u8]>,
{
    expand1::<'a, T>(v).unwrap_or_default()
}

/// Shortcut function for the `foreground()` and `background()` functions.
fn expand1<'a, T>(v: u8) -> Option<String>
where
    T: Capability<'a> + AsRef<[u8]>,
{
    let info = (*TERMINFO).as_ref()?;
    let e = expand!(info.get::<T>()?.as_ref(); v).ok()?;
    let s = std::str::from_utf8(&e).ok()?;
    Some(s.to_owned())
}
