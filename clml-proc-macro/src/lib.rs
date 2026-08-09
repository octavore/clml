//! This internal library provides the procedural macros needed by the crate [`clml`].
//!
//! [`clml`]: https://crates.io/crates/clml

extern crate proc_macro;

#[macro_use]
mod util;
#[cfg(not(feature = "terminfo"))]
mod ansi;
#[cfg(not(feature = "terminfo"))]
mod ansi_constants;
mod color_context;
mod error;
mod format_args;
mod parse;
#[cfg(feature = "terminfo")]
mod terminfo;
mod untagged;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
    Expr,
};

// The doc examples below are gated on `not(terminfo)`: with the `terminfo` feature the generated
// code refers to `clml::` runtime constants, which this crate cannot name from its own doctests.

/// The same as `format!()`, but parses color tags.
///
#[cfg_attr(
    not(feature = "terminfo"),
    doc = r#"#### Example

```
# use clml_proc_macro::cformat;
let s: String = cformat!("A <g>green</> word, {}", "placeholders are allowed");
assert_eq!(s, "A \u{1b}[32mgreen\u{1b}[39m word, placeholders are allowed");
```"#
)]
#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    get_macro("format", input, false)
}

/// The same as `print!()`, but parses color tags.
///
#[cfg_attr(
    not(feature = "terminfo"),
    doc = r#"#### Example

```
# use clml_proc_macro::cprint;
cprint!("A <g>green</> word, {}", "placeholders are allowed");
```"#
)]
#[proc_macro]
pub fn cprint(input: TokenStream) -> TokenStream {
    get_macro("print", input, false)
}

/// The same as `eprint!()`, but parses color tags.
///
#[cfg_attr(
    not(feature = "terminfo"),
    doc = r#"#### Example

```
# use clml_proc_macro::ceprint;
ceprint!("A <g>green</> word, {}", "placeholders are allowed");
```"#
)]
#[proc_macro]
pub fn ceprint(input: TokenStream) -> TokenStream {
    get_macro("eprint", input, false)
}

/// The same as `println!()`, but parses color tags.
///
#[cfg_attr(
    not(feature = "terminfo"),
    doc = r#"#### Example

```
# use clml_proc_macro::cprintln;
cprintln!("A <g>green</> word, {}", "placeholders are allowed");
```"#
)]
#[proc_macro]
pub fn cprintln(input: TokenStream) -> TokenStream {
    get_macro("println", input, false)
}

/// The same as `eprintln!()`, but parses color tags.
///
#[cfg_attr(
    not(feature = "terminfo"),
    doc = r#"#### Example

```
# use clml_proc_macro::ceprintln;
ceprintln!("A <g>green</> word, {}", "placeholders are allowed");
```"#
)]
#[proc_macro]
pub fn ceprintln(input: TokenStream) -> TokenStream {
    get_macro("eprintln", input, false)
}

/// The same as `write!()`, but parses color tags.
#[proc_macro]
pub fn cwrite(input: TokenStream) -> TokenStream {
    get_macro("write", input, true)
}

/// The same as `writeln!()`, but parses color tags.
#[proc_macro]
pub fn cwriteln(input: TokenStream) -> TokenStream {
    get_macro("writeln", input, true)
}

/// Colorizes a string literal, without formatting the `format!`-like placeholders.
///
/// * Accepts only one argument;
/// * Will panic if feature `terminfo` is activated.
///
/// #### Example
///
/// ```
/// # use clml_proc_macro::cstr;
/// let s: &str = cstr!("A <g>green</> word");
/// assert_eq!(s, "A \u{1b}[32mgreen\u{1b}[39m word");
/// ```
#[cfg(not(feature = "terminfo"))]
#[proc_macro]
pub fn cstr(input: TokenStream) -> TokenStream {
    crate::ansi::get_cstr(input)
        .unwrap_or_else(|err| err.to_token_stream())
        .into()
}

/// Removes all the color tags from the given string literal.
///
/// Accepts only one argument.
///
/// #### Example
///
/// ```
/// # use clml_proc_macro::untagged;
/// let s: &str = untagged!("A <g>normal</> word");
/// assert_eq!(s, "A normal word");
/// ```
#[proc_macro]
pub fn untagged(input: TokenStream) -> TokenStream {
    crate::untagged::get_untagged(input)
        .unwrap_or_else(|err| err.to_token_stream())
        .into()
}

/// Colorizes a string literal, without formatting the `format!`-like placeholders.
///
/// * Accepts only one argument;
/// * Will panic if feature `terminfo` is activated.
#[cfg(feature = "terminfo")]
#[proc_macro]
pub fn cstr(_: TokenStream) -> TokenStream {
    panic!("Macro cstr!() cannot be used with terminfo feature")
}

struct WriteInput {
    dst: Expr,
    rest: TokenStream,
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let dst: Expr = input.parse()?;
        let _: Comma = input.parse()?;
        // Forward the tail as raw tokens: it is re-parsed as `format!`-like arguments downstream,
        // so parsing it as expressions here would only round-trip it for no benefit.
        let rest = input.parse::<TokenStream2>()?.into();
        Ok(Self { dst, rest })
    }
}

/// Renders a whole processed macro.
fn get_macro(macro_name: &str, input: TokenStream, is_write_macro: bool) -> TokenStream {
    let macro_name = util::ident(macro_name);
    let fmt_args = |input_tail| {
        #[cfg(not(feature = "terminfo"))]
        let format_args = crate::ansi::get_format_args(input_tail);
        #[cfg(feature = "terminfo")]
        let format_args = crate::terminfo::get_format_args(input_tail);
        format_args.unwrap_or_else(|err| err.to_token_stream())
    };

    if is_write_macro {
        let WriteInput { dst, rest } = parse_macro_input!(input);
        let format_args = fmt_args(rest);
        (quote! { #macro_name!(#dst, #format_args) }).into()
    } else {
        let format_args = fmt_args(input);
        (quote! { #macro_name!(#format_args) }).into()
    }
}
