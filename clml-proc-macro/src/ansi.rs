//! Rewrites a format string's colour tags into ANSI escape sequences at compile time.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::LitStr;

use crate::color_context::Context;
use crate::error::{Error, SpanError};
use crate::format_args::{
    Node, get_args_and_format_string, get_format_string, parse_args, parse_format_string,
};

/// Common code shared between the public macros, ANSI implementation.
pub fn get_format_args(input: TokenStream) -> Result<TokenStream2, SpanError> {
    let (format_string_token, args) = get_args_and_format_string(input)?;
    let format_string = format_string_token.value();

    // Split the format string into a list of nodes; each node is either a string literal (text), a
    // placeholder for a `format!`-like macro, or a color code:
    let format_nodes = parse_format_string(&format_string, &format_string_token)?;

    let final_format_string = get_format_string_from_nodes(format_nodes)?;
    let final_format_string = LitStr::new(&final_format_string, format_string_token.span());

    // Group all the final arguments into a single iterator:
    let args = args.iter().map(|arg| arg.to_token_stream()).skip(1); // Skip the original format string
    let final_args = std::iter::once(quote! { #final_format_string }).chain(args);

    Ok(quote! { #(#final_args),* })
}

/// Same as [`get_format_args`], but dedents the format string the same way `indoc::indoc!()`
/// does.
///
/// Color tags are resolved first, then the resulting text is dedented using
/// [`unindent::unindent`]).
///
/// The dedented text is spliced directly into `format!`/`write!`/`writeln!` as a plain literal,
/// in one macro expansion, exactly like [`get_format_args`] does for color tags, rather than
/// emitting a nested `indoc!(..)` call which would silently disable implict named captures: see
/// the `implicit_capture_through_macro_rules_wrapper` test
#[cfg(feature = "doc")]
pub fn get_format_args_doc(input: TokenStream) -> Result<TokenStream2, SpanError> {
    let (format_string_token, args) = get_args_and_format_string(input)?;
    let format_string = format_string_token.value();

    let format_nodes = parse_format_string(&format_string, &format_string_token)?;
    let final_format_string = get_format_string_from_nodes(format_nodes)?;
    let final_format_string = unindent::unindent(&final_format_string);
    let final_format_string = LitStr::new(&final_format_string, format_string_token.span());

    let args = args.iter().map(|arg| arg.to_token_stream()).skip(1); // Skip the original format string
    let final_args = std::iter::once(quote! { #final_format_string }).chain(args);

    Ok(quote! { #(#final_args),* })
}

/// Transforms a string literal by parsing its color tags.
pub fn get_cstr(input: TokenStream) -> Result<TokenStream2, SpanError> {
    let args = parse_args(input)?;
    let format_string_token = get_format_string(args.first())?;
    let format_string = format_string_token.value();

    if args.len() > 1 {
        return Err(SpanError::new(Error::TooManyArgs, None));
    }

    // Split the format string into a list of nodes; each node is either a string literal (text), or
    // a color code; `format!`-like placeholders will be parsed indenpendently, but as they are put
    // back unchanged into the format string, it's not a problem:
    let format_nodes = parse_format_string(&format_string, &format_string_token)?;
    let final_format_string = get_format_string_from_nodes(format_nodes)?;
    let final_format_string = LitStr::new(&final_format_string, format_string_token.span());
    Ok(quote! { #final_format_string })
}

/// Generates a new format string with the color tags replaced by the right ANSI codes.
fn get_format_string_from_nodes(nodes: Vec<Node>) -> Result<String, SpanError> {
    // The final, modified format string which will be given to the `format!`-like macro:
    let mut format_string = String::new();
    // Stores which colors and attributes are set while processing the format string:
    let mut color_context = Context::new();

    // Generate the final format string:
    for node in nodes {
        match node {
            Node::Text(s) | Node::Placeholder(s) => {
                format_string.push_str(s);
            },
            Node::ColorTagGroup(tag_group) => {
                let ansi_string = color_context.ansi_apply_tags(tag_group)?;
                format_string.push_str(&ansi_string);
            },
        }
    }

    Ok(format_string)
}
