//! Color and style terminal strings at compile time using an HTML-like syntax. CLML stands for
//! `command-line markup language`, and the library is based on
//! [`color-print`](https://gitlab.com/dajoha/color-print).
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
//! `cwritelndoc!`), which dedent the format string like [`indoc::indoc!()`]. See the `doc`
//! feature section below.
//!
//! [`cstr!()`] replaces the tags in a string literal with ANSI sequences and returns a new string
//! literal. It does not process formatting placeholders.
//!
//! [`untagged!()`] removes all tags from a string literal.
//!
//! # How it works
//!
//! The macros replace the tags in the format string with ANSI escape codes at compile time. For
//! example:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("HELLO <green>WORLD</green>");
//! cprintln!("HELLO <green>WORLD</>"); // Shorter closing tag
//! # }
//! ```
//!
//! expands to:
//!
//! ```
//! # fn main() {
//! println!("HELLO \u{1b}[32mWORLD\u{1b}[39m")
//! # }
//! ```
//!
//! # Pros and cons
//!
//! ## Pros
//!
//! * Tags are processed at compile time, so there is no runtime cost.
//! * Tags nest, e.g. `"<green>...<blue>...</blue>...</green>"`.
//! * The output contains no redundant ANSI sequences.
//! * Almost every tag has a short name, e.g. `"my <b>blue</> word"`.
//! * Each macro accepts the same arguments as its `std` counterpart, including positional and
//!   named arguments.
//! * Supports 16, 256, and 16M colors.
//! * Reports errors at compile time, pointing into the format string.
//! * The macros can be wrapped in your own `macro_rules!`.
//! * Native support for [`anstream`](https://crates.io/crates/anstream), which strips ANSI codes
//!   when the output does not support them, via the `anstream` feature.
//! * Native support for multi-line strings with automatic dedenting, via the `doc` feature.
//! * Supports OSC 8 hyperlinks.
//!
//! ## Cons
//!
//! * Not tested with non-ANSI terminals such as legacy Windows consoles.
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
//! ## Closing the last open tag with `</>`
//!
//! A closing tag must exactly match its opening tag. `</>` closes the last open tag without
//! repeating its name:
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
//! Separate colors and styles with a comma to combine them in a single tag:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! cprintln!("This is <green,bold>green and bold text</green,bold>.");
//! // The same, closed with </>:
//! cprintln!("This is <green,bold>green and bold text</>.");
//! # }
//! ```
//!
//! ## Nesting tags
//!
//! Any tag can nest inside any other. Closing tags follow HTML nesting rules, and `</>` always
//! closes the innermost open tag:
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
//! The macros append the ANSI sequences that reset any tags left open:
//!
//! ```
//! # use clml::cprintln;
//! # fn main() {
//! // These two lines are equivalent:
//! cprintln!("<green><bold>Hello");
//! cprintln!("<green><bold>Hello</></>");
//! # }
//! ```
//!
//! ## Printing `<` and `>` verbatim
//!
//! Double `<` and `>` to print them verbatim, as with `{` and `}` in standard format strings:
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
//! `<link(URL)>` wraps text in an OSC 8 hyperlink, which most modern terminal emulators render as a
//! clickable link:
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
//! The expanded format string contains only the ANSI codes that change the current style. At each
//! tag, the macros compare the new style attributes with the current ones and emit codes only for
//! the differences. For example, several nested `<bold>` tags produce one bold sequence:
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
//! The macros work inside your own `macro_rules!`:
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
//! Implicit named captures such as `{package}` resolve against the caller's scope, not the
//! wrapper's.
//!
//! # `anstream` feature
//!
//! The `anstream` feature is enabled by default. It routes the printing macros through
//! [`anstream`](https://crates.io/crates/anstream), which strips escape codes when the output is
//! not a terminal, honors `NO_COLOR`, `CLICOLOR`, and `CLICOLOR_FORCE`, and calls the console API
//! on legacy Windows consoles. CLML has not been tested with non-ANSI terminals such as legacy
//! Windows consoles.
//!
//! Without it, the printing macros write to `std::io::stdout` or `std::io::stderr` and emit the
//! escape codes verbatim. To opt out:
//!
//! ```toml
//! clml = { version = "0.2", default-features = false }
//! ```
//!
//! # `doc` feature
//!
//! The `doc` feature is enabled by default. It adds a `...doc!` variant of every formatting macro:
//! [`cformatdoc!()`], [`cprintdoc!()`], [`cprintlndoc!()`], [`ceprintdoc!()`], [`ceprintlndoc!()`],
//! [`cwritedoc!()`], and [`cwritelndoc!()`]. These macros process tags first, then strip the
//! common leading whitespace from every line, as [`indoc::indoc!()`] does. Implicit named captures
//! such as `{name}` work as usual.
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
//! To opt out, disable default features and re-add `anstream` if you want it:
//!
//! ```toml
//! clml = { version = "0.2", default-features = false, features = ["anstream"] }
//! ```
//!
//! # Tag naming rules
//!
//! Each tag has a **long name**, such as `<magenta>` or `<underline>`.
//!
//! Color tags such as `<red>`, `<bg:blue>`, and `<bg:bright-green>` follow common naming rules.
//! Style tags such as `<bold>` and `<italics>` do not.
//!
//!  * Each color has four variants:
//!    - `<mycolor>`: normal foreground
//!    - `<bright-mycolor>` or `<mycolor!>`: bright foreground
//!    - `<bg:mycolor>` or `<MYCOLOR>`: normal background
//!    - `<bg:bright-mycolor>`, `<bg:mycolor!>`, `<BRIGHT-MYCOLOR>`, or `<MYCOLOR!>`: bright
//!      background
//!  * Each color has a one-letter **shortcut**. With `x` as the letter:
//!    - `<x>`: normal foreground
//!    - `<x!>`: bright foreground
//!    - `<bg:x>` or `<X>`: normal background
//!    - `<bg:x!>` or `<X!>`: bright background
//!  * The shortcut is the **first letter of the color's name**, except for black, which uses `<k>`.
//!    For example, `<y>` is the shortcut for `<yellow>`.
//!  * An uppercase color tag is a **background color**.
//!  * A trailing `!` makes a color **bright**.
//!
//! # Accepted tags
//!
//! Each tag has a long name. Most also have a shortcut and aliases.
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
//!
//! [`indoc::indoc!()`]: https://docs.rs/indoc/latest/indoc/macro.indoc.html

pub use clml_proc_macro::{cformat, cstr, cwrite, cwriteln, untagged};
#[cfg(feature = "doc")]
pub use clml_proc_macro::{cformatdoc, cwritedoc, cwritelndoc};

/// The same as `print!()`, but parses color tags.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
#[macro_export]
macro_rules! cprint {
    ($($arg:tt)*) => { $crate::__private::print!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `println!()`, but parses color tags.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
#[macro_export]
macro_rules! cprintln {
    ($($arg:tt)*) => { $crate::__private::println!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `eprint!()`, but parses color tags.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
#[macro_export]
macro_rules! ceprint {
    ($($arg:tt)*) => { $crate::__private::eprint!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `eprintln!()`, but parses color tags.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
#[macro_export]
macro_rules! ceprintln {
    ($($arg:tt)*) => { $crate::__private::eprintln!("{}", $crate::cformat!($($arg)*)) };
}

/// The same as `print!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
#[cfg(feature = "doc")]
#[macro_export]
macro_rules! cprintdoc {
    ($($arg:tt)*) => { $crate::__private::print!("{}", $crate::cformatdoc!($($arg)*)) };
}

/// The same as `println!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
#[cfg(feature = "doc")]
#[macro_export]
macro_rules! cprintlndoc {
    ($($arg:tt)*) => { $crate::__private::println!("{}", $crate::cformatdoc!($($arg)*)) };
}

/// The same as `eprint!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
#[cfg(feature = "doc")]
#[macro_export]
macro_rules! ceprintdoc {
    ($($arg:tt)*) => { $crate::__private::eprint!("{}", $crate::cformatdoc!($($arg)*)) };
}

/// The same as `eprintln!()`, but parses color tags and dedents the format string like
/// `indoc::indoc!()`.
///
/// Writes through `anstream` when the `anstream` feature is enabled.
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
