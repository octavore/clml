//! Procedural macros for [`clml`]. This crate is internal and has no stable API.
//!
//! [`clml`]: https://crates.io/crates/clml

extern crate proc_macro;

#[macro_use]
mod util;
mod ansi;
mod ansi_constants;
mod color_context;
mod error;
mod format_args;
mod parse;
mod untagged;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{Expr, parse_macro_input};

/// The same as `format!()`, but parses color tags.
///
/// #### Example
///
/// ```
/// # use clml_proc_macro::cformat;
/// let s: String = cformat!("A <g>green</> word, {}", "placeholders are allowed");
/// assert_eq!(s, "A \u{1b}[32mgreen\u{1b}[39m word, placeholders are allowed");
/// ```
#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    get_macro("format", input, false)
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

/// The same as `cformat!()`, but also dedents the format string like `indoc::indoc!()`.
#[cfg(feature = "doc")]
#[proc_macro]
pub fn cformatdoc(input: TokenStream) -> TokenStream {
    get_macro_doc("format", input, false)
}

/// The same as `cwrite!()`, but also dedents the format string like `indoc::indoc!()`.
#[cfg(feature = "doc")]
#[proc_macro]
pub fn cwritedoc(input: TokenStream) -> TokenStream {
    get_macro_doc("write", input, true)
}

/// The same as `cwriteln!()`, but also dedents the format string like `indoc::indoc!()`.
#[cfg(feature = "doc")]
#[proc_macro]
pub fn cwritelndoc(input: TokenStream) -> TokenStream {
    get_macro_doc("writeln", input, true)
}

/// Replaces the tags in a string literal with ANSI sequences. Does not process formatting
/// placeholders.
///
/// Takes exactly one argument.
///
/// #### Example
///
/// ```
/// # use clml_proc_macro::cstr;
/// let s: &str = cstr!("A <g>green</> word");
/// assert_eq!(s, "A \u{1b}[32mgreen\u{1b}[39m word");
/// ```
#[proc_macro]
pub fn cstr(input: TokenStream) -> TokenStream {
    crate::ansi::get_cstr(input)
        .unwrap_or_else(|err| err.to_token_stream())
        .into()
}

/// Removes all tags from a string literal.
///
/// Takes exactly one argument.
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
    get_macro_impl(
        macro_name,
        input,
        is_write_macro,
        crate::ansi::get_format_args,
    )
}

/// Same as [`get_macro`], but the format string is dedented like `indoc::indoc!()`.
#[cfg(feature = "doc")]
fn get_macro_doc(macro_name: &str, input: TokenStream, is_write_macro: bool) -> TokenStream {
    get_macro_impl(
        macro_name,
        input,
        is_write_macro,
        crate::ansi::get_format_args_doc,
    )
}

fn get_macro_impl(
    macro_name: &str,
    input: TokenStream,
    is_write_macro: bool,
    get_format_args: fn(TokenStream) -> Result<TokenStream2, crate::error::SpanError>,
) -> TokenStream {
    let macro_name = util::ident(macro_name);
    let fmt_args =
        |input_tail| get_format_args(input_tail).unwrap_or_else(|err| err.to_token_stream());

    if is_write_macro {
        let WriteInput { dst, rest } = parse_macro_input!(input);
        let format_args = fmt_args(rest);
        (quote! { #macro_name!(#dst, #format_args) }).into()
    } else {
        let format_args = fmt_args(input);
        (quote! { #macro_name!(#format_args) }).into()
    }
}
