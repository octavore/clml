//! Colorize and stylize strings for terminal at compile-time, by using an HTML-like syntax. Based
//! on [`color-print`](https://gitlab.com/dajoha/color-print). CLML stands for `command-line markup
//! language`.
//!
//! This library provides the following macros:
//!
//!  - `cformat!(<FORMAT_STRING> [, ARGS...])`
//!  - `cprint!(<FORMAT_STRING> [, ARGS...])`
//!  - `cprintln!(<FORMAT_STRING> [, ARGS...])`
//!  - `ceprint!(<FORMAT_STRING> [, ARGS...])`
//!  - `ceprintln!(<FORMAT_STRING> [, ARGS...])`
//!  - `cwrite!(f, <FORMAT_STRING> [, ARGS...])`
//!  - `cwriteln!(f, <FORMAT_STRING> [, ARGS...])`
//!  - `cstr!(<FORMAT_STRING>)`
//!  - `untagged!(<FORMAT_STRING>)`
//!
//! With the `doc` feature enabled, a `...doc!` variant of each formatting macro is also available
//! (`cformatdoc!`, `cprintdoc!`, `cprintlndoc!`, `ceprintdoc!`, `ceprintlndoc!`, `cwritedoc!`,
//! `cwritelndoc!`), which dedents the format string like [`indoc::indoc!()`]; see below for more.
//!
//! CLML macros support colors via HTML-like tags which add ANSI colors/styles at compile-time.
//!
//! [`cstr!()`] only transforms the given string literal into another string literal, without
//! formatting anything else than the colors tag.
//!
//! [`untagged!()`] removes all the tags found in the given string literal.
//!
//! ## What does it do ?
//!
//! By default, the provided macros will replace the tags found in the format string by ANSI
//! hexadecimal escape codes. e.g.:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("HELLO <green>WORLD</green>");
//! cprintln!("HELLO <green>WORLD</>"); // Alternative, shorter syntax
//!
//! # }
//! ```
//!
//! will be replaced by:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! println!("HELLO \u{1b}[31mWORLD\u{1b}[39m")
//! # }
//! ```
//!
//! # Pros/cons of this crate
//!
//! ## Pros
//!
//! * Styling is processed at compile-time, so there is no runtime payload.
//! * Nested tags are well handled, e.g. `"<green>...<blue>...</blue>...</green>"`.
//! * Some optimizations are performed to avoid redundant ANSI sequences, because these
//!   optimizations can be done at compile-time without impacting the runtime.
//! * Almost every tag has a short name, so colorizing can be done quickly: `"my <b>blue</> word"`.
//! * Each provided macro can be used exactly in the same way as the standard `format!`-like macros.
//!   e.g., positional arguments and named arguments can be used as usual.
//! * Supports 16, 256 and 16M colors.
//! * Fine-grained error handling (errors will be given at compile-time).
//! * Macros can be composed with other macros, e.g. re-exported through your own `macro_rules!`.
//! * Native support for [`anstream`](https://crates.io/crates/anstream) (automatically removes ANSI
//!   codes where not supported), via the `anstream` feature.
//! * Native support for multi-line strings with automatic dedenting, via the `doc` feature.
//! * Supports OSC 8 hyperlinks.
//!
//! ## Cons
//!
//! * Not compatible with non-ANSI terminals.
//!
//! # Introduction
//!
//! ## Basic example
//!
//! ```
//! use clml::cprintln;
//! cprintln!("Hello <green>world</green>!");
//! ```
//!
//! ## Closing a tag more simply: the `</>` tag
//!
//! Instead of closing tags with a matching tag, which must be exact, you can also close the last
//! open tag simply with `</>`:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("Hello <green>world</>!");
//! # }
//! ```
//!
//! ## Combining colors and styles
//!
//! Multiple styles and colors can be combined into a single tag by separating them with the `,`
//! comma character:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("This a <green,bold>green and bold text</green,bold>.");
//! // The same, but closing with the </> tag:
//! cprintln!("This a <green,bold>green and bold text</>.");
//! # }
//! ```
//!
//! ## Nesting tags
//!
//! Any tag can be nested with any other.
//!
//! *Note*: The closing tags must match correctly (following the basic rules of nesting for HTML
//! tags), but it can always be simplified by using the tag `</>`.
//!
//! Example of nested tags:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("<green>This is green, <bold>then green and bold</bold>, then green again</green>");
//! cprintln!("<green>This is green, <bold>then green and bold</>, then green again</>");
//!
//! // Colors can be nested as well:
//! cprintln!("<green>This is green, <blue>then blue</blue>, then green again</green>");
//! cprintln!("<green>This is green, <blue>then blue</>, then green again</>");
//! # }
//! ```
//!
//! ## Unclosed tags are automatically closed at the end of the format string
//!
//! Tags which have not been closed manually will be closed automatically, which means that the ANSI
//! sequences needed to go back to the original state will be added:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! // The two following lines are strictly equivalent:
//! cprintln!("<green><bold>Hello");
//! cprintln!("<green><bold>Hello</></>");
//! # }
//! ```
//!
//! ## How to display the chars `<` and `>` verbatim
//!
//! As for `{` and `}` in standard format strings, the chars `<` and `>` have to  be doubled in
//! order to display them verbatim:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("This is an angle bracket character: <<, and here is another one: >>");
//! # }
//! ```
//!
//! ## Hyperlinks
//!
//! `<link(URL)>` wraps text in an OSC 8 hyperlink, which most modern terminal  emulators render as
//! a clickable link:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("<link(https://example.com)>click me</>");
//! # }
//! ```
//!
//! Like other tags, it can be combined with styles (`<link(...),bold>`) and closed either by
//! repeating the URL (`</link(https://example.com)>`) or with `</>`.
//!
//! # Optimization: no redundant ANSI codes
//!
//! The expanded format string will only contain the *needed* ANSI codes. This is done by making a
//! diff of the different style attributes, each time a tag is encountered, instead of mechanically
//! adding the ANSI codes.
//!
//! E.g., several nested `<bold>` tags will only produce one bold ANSI sequence:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("<bold><bold> A <bold,blue> B </> C </></>")
//! # }
//! ```
//!
//! will be replaced by:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! println!("\u{1b}[1m A \u{1b}[34m B \u{1b}[39m C \u{1b}[22m")
//! //        ^-------^   ^--------^   ^--------^   ^--------^
//! //          bold         blue         color        bold
//! //                                    reset        reset
//! # }
//! ```
//!
//! # Composability
//!
//! `clml` is designed to be wrapped. Macros will work when re-exporting them through your own
//! `macro_rules!`, e.g. routing output through a tty-aware adapter such as
//! [`anstream`](https://crates.io/crates/anstream):
//!
//! ```
//! macro_rules! status {
//!     ($($arg:tt)*) => { ::clml::cprintln!($($arg)*) };
//! }
//!
//! let package = "clml";
//! status!("<green>Compiling</green> {package}");
//! ```
//!
//! Implicit named captures (e.g. `{package}` above) will resolve against the caller's scope, not
//! the wrapper.
//!
//! # `anstream` feature
//!
//! By default the printing macros write to `std::io::stdout`/`stderr` and the escape codes go out
//! verbatim. Enable the `anstream` feature to route them through
//! [`anstream`](https://crates.io/crates/anstream) instead:
//!
//! ```toml
//! clml = { version = "0.1", features = ["anstream"] }
//! ```
//!
//! With anstream, backend codes are stripped when stdout isn't a terminal,
//! `NO_COLOR`/`CLICOLOR`/`CLICOLOR_FORCE` are honored, and legacy Windows consoles get console API
//! calls instead of escape sequences.
//!
//!
//! # `doc` feature
//!
//! Enabling the `doc` feature adds a `...doc!` variant of the formatting macros:
//! [`cformatdoc!()`], [`cprintdoc!()`], [`cprintlndoc!()`], [`ceprintdoc!()`], [`ceprintlndoc!()`],
//! [`cwritedoc!()`], [`cwritelndoc!()`]. These dedent the format string like [`indoc::indoc!()`]
//! does, on top of the usual tag processing:
//!
//! ```toml
//! clml = { version = "0.1", features = ["doc"] }
//! ```
//!
//! ```text
//! cprintlndoc!(
//!     "
//!     <green>Hello, {name}!</green>
//!         This line is indented one level further.
//!     "
//! );
//! ```
//!
//! Color tags are resolved first, then the common leading whitespace is stripped from every line,
//! similar to wrapping the format string in `indoc!`. Supports implicit named captures (e.g.
//! `{name}` above).
//!
//! # Naming rules of the tags:
//!
//! Each tag has at least a **long name**, like `<magenta>` or `<underline>`.
//!
//! The tags directly relative to *colors* (like `<red>`, `<bg:blue>`, `<bg:bright-green>`..., as
//! opposed to *style* tags like `<bold>`, `<italics>`...) have some common naming rules:
//!
//!  * Each tag has four variants:
//!    - `<mycolor>`: the normal, foreground color;
//!    - `<bright-mycolor>` or `<mycolor!>`: the bright, foreground color;
//!    - `<bg:mycolor>`, `<MYCOLOR>`: the normal, background color;
//!    - `<bg:bright-mycolor>`, `<bg:mycolor!>`, `<BRIGHT-MYCOLOR>` or `<MYCOLOR!>`: the bright,
//!      background color;
//!  * Each tag has a *shortcut*, with a base letter for each color; example with the `x` letter:
//!    - `<x>`: the normal, foreground color;
//!    - `<x!>`: the bright, foreground color;
//!    - `<bg:x>`, `<X>`: the normal, background color;
//!    - `<bg:x!>`, `<X!>`: the bright, background color;
//!  * Each color's shortcut letter is simply the **first letter of its name** (excepted for `<k>`
//!    which is the shortcut for `<black>`), e.g. `<y>` is the shortcut for `<yellow>`;
//!  * Each color's tag which is uppercase is a **background color**;
//!  * Each tag which has a trailing exclamation point `!` is a **bright color**;
//!
//! # List of accepted tags:
//!
//! Every tag below is supported; each has a long name, and most have a shortcut and aliases.
//!
//! | Shortcuts       | Long names            | Aliases                                                     |
//! |-----------------|-----------------------|-------------------------------------------------------------|
//! | `<s>`           | `<strong>`            | `<em>` `<bold>`                                             |
//! |                 | `<dim>`               |                                                             |
//! | `<u>`           | `<underline>`         |                                                             |
//! |                 | `<strike>`            |                                                             |
//! |                 | `<reverse>`           | `<rev>`                                                     |
//! |                 | `<conceal>`           | `<hide>`                                                    |
//! | `<i>`           | `<italics>`           | `<italic>`                                                  |
//! |                 | `<blink>`             |                                                             |
//! | `<k>`           | `<black>`             |                                                             |
//! | `<r>`           | `<red>`               |                                                             |
//! | `<g>`           | `<green>`             |                                                             |
//! | `<y>`           | `<yellow>`            |                                                             |
//! | `<b>`           | `<blue>`              |                                                             |
//! | `<m>`           | `<magenta>`           |                                                             |
//! | `<c>`           | `<cyan>`              |                                                             |
//! | `<w>`           | `<white>`             |                                                             |
//! | `<k!>`          | `<bright-black>`      | `<black!>`                                                  |
//! | `<r!>`          | `<bright-red>`        | `<red!>`                                                    |
//! | `<g!>`          | `<bright-green>`      | `<green!>`                                                  |
//! | `<y!>`          | `<bright-yellow>`     | `<yellow!>`                                                 |
//! | `<b!>`          | `<bright-blue>`       | `<blue!>`                                                   |
//! | `<m!>`          | `<bright-magenta>`    | `<magenta!>`                                                |
//! | `<c!>`          | `<bright-cyan>`       | `<cyan!>`                                                   |
//! | `<w!>`          | `<bright-white>`      | `<white!>`                                                  |
//! | `<K>`           | `<bg:black>`          | `<BLACK>`                                                   |
//! | `<R>`           | `<bg:red>`            | `<RED>`                                                     |
//! | `<G>`           | `<bg:green>`          | `<GREEN>`                                                   |
//! | `<Y>`           | `<bg:yellow>`         | `<YELLOW>`                                                  |
//! | `<B>`           | `<bg:blue>`           | `<BLUE>`                                                    |
//! | `<M>`           | `<bg:magenta>`        | `<MAGENTA>`                                                 |
//! | `<C>`           | `<bg:cyan>`           | `<CYAN>`                                                    |
//! | `<W>`           | `<bg:white>`          | `<WHITE>`                                                   |
//! | `<K!>`          | `<bg:bright-black>`   | `<BLACK!>` `<bg:black!>` `<BRIGHT-BLACK>`                   |
//! | `<R!>`          | `<bg:bright-red>`     | `<RED!>` `<bg:red!>` `<BRIGHT-RED>`                         |
//! | `<G!>`          | `<bg:bright-green>`   | `<GREEN!>` `<bg:green!>` `<BRIGHT-GREEN>`                   |
//! | `<Y!>`          | `<bg:bright-yellow>`  | `<YELLOW!>` `<bg:yellow!>` `<BRIGHT-YELLOW>`                |
//! | `<B!>`          | `<bg:bright-blue>`    | `<BLUE!>` `<bg:blue!>` `<BRIGHT-BLUE>`                      |
//! | `<M!>`          | `<bg:bright-magenta>` | `<MAGENTA!>` `<bg:magenta!>` `<BRIGHT-MAGENTA>`             |
//! | `<C!>`          | `<bg:bright-cyan>`    | `<CYAN!>` `<bg:cyan!>` `<BRIGHT-CYAN>`                      |
//! | `<W!>`          | `<bg:bright-white>`   | `<WHITE!>` `<bg:white!>` `<BRIGHT-WHITE>`                   |
//! |                 | `<rgb(r,g,b)>`        | `<#RRGGBB>`                                                 |
//! |                 | `<bg:rgb(r,g,b)>`     | `<bg:#RRGGBB>` `<RGB(r,g,b)>`                               |
//! | `<0>`...`<255>` | `<palette(...)>`      | `<p(...)>` `<pal(...)>`                                     |
//! | `<P(...)>`      | `<bg:palette(...)>`   | `<PALETTE(...)>` `<PAL(...)>` `<bg:p(...)>` `<bg:pal(...)>` |
//! |                 | `<link(URL)>`         |                                                             |

pub use clml_proc_macro::{cformat, cstr, cwrite, cwriteln, untagged};
#[cfg(feature = "doc")]
pub use clml_proc_macro::{cformatdoc, cwritedoc, cwritelndoc};

/// The same as `print!()`, but parses color tags.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[macro_export]
macro_rules! cprint {
    ($($arg:tt)*) => { $crate::__private::print!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `println!()`, but parses color tags.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[macro_export]
macro_rules! cprintln {
    ($($arg:tt)*) => { $crate::__private::println!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `eprint!()`, but parses color tags.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[macro_export]
macro_rules! ceprint {
    ($($arg:tt)*) => { $crate::__private::eprint!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `eprintln!()`, but parses color tags.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[macro_export]
macro_rules! ceprintln {
    ($($arg:tt)*) => { $crate::__private::eprintln!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `print!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[cfg(feature = "doc")]
#[macro_export]
macro_rules! cprintdoc {
    ($($arg:tt)*) => { $crate::__private::print!("{}", $crate::cformatdoc!($($arg)*)) };
}

/// The same as `println!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[cfg(feature = "doc")]
#[macro_export]
macro_rules! cprintlndoc {
    ($($arg:tt)*) => { $crate::__private::println!("{}", $crate::cformatdoc!($($arg)*)) };
}

/// The same as `eprint!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[cfg(feature = "doc")]
#[macro_export]
macro_rules! ceprintdoc {
    ($($arg:tt)*) => { $crate::__private::eprint!("{}", $crate::cformatdoc!($($arg)*)) };
}

/// The same as `eprintln!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through [`__private`], so the `anstream` feature applies.
#[cfg(feature = "doc")]
#[macro_export]
macro_rules! ceprintlndoc {
    ($($arg:tt)*) => { $crate::__private::eprintln!("{}", $crate::cformatdoc!($($arg)*)) };
}

/// Implementation detail of the printing macros. Not part of the public API.
///
/// `cprint!` and friends expand to `$crate::__private::print!(..)` rather than `std::print!(..)`,
/// which is what lets the `anstream` feature re-point them at an auto-adapting stream without
/// changing anything at the call site. Going through `$crate` also means the expansion keeps
/// working when the dependency is renamed (`mycolor = { package = "clml" }`).
#[doc(hidden)]
pub mod __private {
    #[cfg(not(feature = "anstream"))]
    pub use std::{eprint, eprintln, print, println};

    #[cfg(feature = "anstream")]
    pub use anstream::{eprint, eprintln, print, println};
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use super::*;

    #[test]
    fn format_no_arg() {
        assert_eq!(cformat!(), "");
        cprint!();
        cprintln!();
    }

    #[test]
    fn format_no_color() {
        assert_eq!(cformat!(""), "");
        assert_eq!(cformat!("Hi"), "Hi");
        assert_eq!(cformat!("Hi {}", 12), "Hi 12");
        assert_eq!(cformat!("Hi {n} {}", 12, n = 24), "Hi 24 12");

        let mut s = String::new();
        cwrite!(&mut s, "").unwrap();
        assert_eq!(s, "");

        let mut s = String::new();
        cwrite!(&mut s, "Hi").unwrap();
        assert_eq!(s, "Hi");

        let mut s = String::new();
        cwrite!(&mut s, "Hi {}", 12).unwrap();
        assert_eq!(s, "Hi 12");

        let mut s = String::new();
        cwrite!(&mut s, "Hi {n} {}", 12, n = 24).unwrap();
        assert_eq!(s, "Hi 24 12");
    }

    #[test]
    #[rustfmt::skip]
    fn format_basic() {
        assert_eq!(cformat!("<red>Hi</red>"), "\u{1b}[31mHi\u{1b}[39m");
        assert_eq!(cformat!("<red>Hi</r>"), "\u{1b}[31mHi\u{1b}[39m");
        assert_eq!(cformat!("<red>Hi</>"), "\u{1b}[31mHi\u{1b}[39m");

        assert_eq!(cformat!("<bg:red>Hi</bg:red>"), "\u{1b}[41mHi\u{1b}[49m");
        assert_eq!(cformat!("<bg:red>Hi</R>"), "\u{1b}[41mHi\u{1b}[49m");
        assert_eq!(cformat!("<bg:red>Hi</>"), "\u{1b}[41mHi\u{1b}[49m");

        assert_eq!(
            cformat!("Hi <bold>word</bold> !"),
            "Hi \u{1b}[1mword\u{1b}[22m !"
        );
        assert_eq!(cformat!("Hi <em>word</em> !"), "Hi \u{1b}[1mword\u{1b}[22m !");
        assert_eq!(cformat!("Hi <em>word</> !"), "Hi \u{1b}[1mword\u{1b}[22m !");

        assert_eq!(
            cformat!("
                <bold>bold</>
                <dim>dim</>
                <underline>underline</>
                <strike>strike</>
                <reverse>reverse</>
                <conceal>conceal</>
                <italics>italics</>
                <blink>blink</>
            "),
            "
                \u{1b}[1mbold\u{1b}[22m
                \u{1b}[2mdim\u{1b}[22m
                \u{1b}[4munderline\u{1b}[24m
                \u{1b}[9mstrike\u{1b}[29m
                \u{1b}[7mreverse\u{1b}[27m
                \u{1b}[8mconceal\u{1b}[28m
                \u{1b}[3mitalics\u{1b}[23m
                \u{1b}[5mblink\u{1b}[25m
            "
        );

        let mut s = String::new();
        cwrite!(&mut s, "Hi <r>{v}</> {}", 12, v = "Hi").unwrap();
        assert_eq!(s, "Hi \u{1b}[31mHi\u{1b}[39m 12");

        let mut s = String::new();
        cwriteln!(&mut s, "Hi <r>{v} {}", 12, v = "Hi").unwrap();
        assert_eq!(s, "Hi \u{1b}[31mHi 12\u{1b}[39m\n");
    }

    #[test]
    #[ignore]
    fn bold_and_dim_should_be_optimized() {
        assert_eq!(
            cformat!("<bold>BOLD</><dim>DIM</>"),
            "\u{1b}[1mBOLD\u{1b}[2mDIM\u{1b}[22m"
        );
    }

    #[test]
    #[rustfmt::skip]
    fn format_link() {
        assert_eq!(
            cformat!("<link(https://example.com)>Click</link(https://example.com)>"),
            "\u{1b}]8;;https://example.com\u{1b}\\Click\u{1b}]8;;\u{1b}\\"
        );
        assert_eq!(
            cformat!("<link(https://example.com)>Click</>"),
            "\u{1b}]8;;https://example.com\u{1b}\\Click\u{1b}]8;;\u{1b}\\"
        );
        assert_eq!(
            cformat!("<link( https://example.com )>Click</>"),
            "\u{1b}]8;;https://example.com\u{1b}\\Click\u{1b}]8;;\u{1b}\\"
        );
        // Combined with a style tag:
        assert_eq!(
            cformat!("<link(https://example.com),bold>Click</>"),
            "\u{1b}[1m\u{1b}]8;;https://example.com\u{1b}\\Click\u{1b}[22m\u{1b}]8;;\u{1b}\\"
        );
    }

    #[test]
    fn format_multiple() {
        assert_eq!(
            cformat!("Hi <bold>word</bold> <red>red</red> !"),
            "Hi \u{1b}[1mword\u{1b}[22m \u{1b}[31mred\u{1b}[39m !"
        );
    }

    #[test]
    fn format_optimization() {
        assert_eq!(
            cformat!("<red>RED<blue>BLUE</>RED</>"),
            "\u{1b}[31mRED\u{1b}[34mBLUE\u{1b}[31mRED\u{1b}[39m"
        );
        assert_eq!(
            cformat!("<red><blue>BLUE</>RED</>"),
            "\u{1b}[34mBLUE\u{1b}[31mRED\u{1b}[39m"
        );
        assert_eq!(cformat!("<red></>Text"), "Text");
    }

    #[test]
    #[rustfmt::skip]
    fn format_auto_close_tag() {
        assert_eq!(
            cformat!("<red>RED<blue>BLUE"),
            "\u{1b}[31mRED\u{1b}[34mBLUE\u{1b}[39m"
        );
        assert!(
            cformat!("<red>RED<em>BOLD") == "\u{1b}[31mRED\u{1b}[1mBOLD\u{1b}[22m\u{1b}[39m"
            ||
            cformat!("<red>RED<em>BOLD") == "\u{1b}[31mRED\u{1b}[1mBOLD\u{1b}[39m\u{1b}[22m"
        );
    }

    #[test]
    fn untagged() {
        assert_eq!(untagged!(""), "");
        assert_eq!(untagged!("hi"), "hi");
        assert_eq!(untagged!("<red>hi"), "hi");
        assert_eq!(untagged!("<red>hi</>"), "hi");
        assert_eq!(untagged!("<red>hi <em,blue>all"), "hi all");
        assert_eq!(untagged!("<red>hi <em>all</></>"), "hi all");
    }

    /// Regression test for wrapping the macros in another `macro_rules!`, the
    /// pattern used to route output through a stream adapter such as
    /// `anstream`.
    ///
    /// Implicit named captures (RFC 2795) are resolved against the span of the
    /// format string literal, so the literal we rebuild must carry the span
    /// of the literal the caller wrote. If it instead gets a fresh
    /// `Span::call_site()`, that resolves at the wrapper macro's definition
    /// site and `{msg}` fails to compile with "cannot find value `msg` in this
    /// scope".
    #[test]
    fn implicit_capture_through_macro_rules_wrapper() {
        macro_rules! wrapped_cformat {
            ($($arg:tt)*) => { cformat!($($arg)*) };
        }
        macro_rules! wrapped_untagged {
            ($($arg:tt)*) => { untagged!($($arg)*) };
        }

        let msg = "hello";
        let count = 2;

        assert_eq!(wrapped_cformat!("{msg}"), "hello");
        assert_eq!(wrapped_cformat!("{msg} {count}"), "hello 2");
        assert_eq!(
            wrapped_cformat!("<red>{msg}</red> {msg}")
                .matches(msg)
                .count(),
            2
        );
        assert_eq!(wrapped_untagged!("<red>hi</red>"), "hi");

        macro_rules! wrapped_cstr {
            ($($arg:tt)*) => { cstr!($($arg)*) };
        }
        assert_eq!(
            wrapped_cformat!("<red>{msg}</red>"),
            "\u{1b}[31mhello\u{1b}[39m"
        );
        assert_eq!(wrapped_cstr!("<red>hi</red>"), "\u{1b}[31mhi\u{1b}[39m");
    }

    #[cfg(feature = "doc")]
    #[test]
    fn format_doc_dedents() {
        assert_eq!(
            cformatdoc!(
                "
                <red>RED
                    indented</red>
                "
            ),
            "\u{1b}[31mRED\n    indented\u{1b}[39m\n"
        );

        assert_eq!(cformatdoc!("<red>{}</red>", "Hi"), "\u{1b}[31mHi\u{1b}[39m");

        let mut s = String::new();
        cwritedoc!(
            &mut s,
            "
            <bold>a
              b</bold>
            "
        )
        .unwrap();
        assert_eq!(s, "\u{1b}[1ma\n  b\u{1b}[22m\n");

        let mut s = String::new();
        cwritelndoc!(
            &mut s,
            "
            <bold>a
              b</bold>"
        )
        .unwrap();
        assert_eq!(s, "\u{1b}[1ma\n  b\u{1b}[22m\n");
    }

    #[cfg(feature = "doc")]
    #[test]
    fn print_doc_macros_compile_and_run() {
        cprintdoc!(
            "
            <green>ok</green>
            "
        );
        cprintlndoc!(
            "
            <green>ok</green>"
        );
        ceprintdoc!(
            "
            <red>note</red>
            "
        );
        ceprintlndoc!(
            "
            <red>note</red>"
        );
    }
}
