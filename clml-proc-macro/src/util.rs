use std::ops::RangeBounds;

use proc_macro2::{Ident, Span};
use syn::LitStr;

/// Joins the arguments with `&&` operators.
macro_rules! and {
    ($($expr:expr),* $(,)?) => {
        $($expr)&&*
    };
}

/// Creates a new [`Ident`] which can be tokenized.
pub fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

/// Creates a new [`struct@LitStr`] which can be tokenized.
pub fn literal_string(s: &str) -> LitStr {
    LitStr::new(s, Span::call_site())
}

/// Returns the subspan corresponding to the range of `inside` inside `input`, considering that:
///  - `input` is exactly `&input_lit_str.value()`,
///  - `inside` is a subslice of `input`,
///
/// Note: `subspan` only returns `Some` on a nightly compiler, so on stable this falls back to
/// `input_lit_str`'s full span.
pub fn inner_span<'a>(input: &'a str, input_lit_str: &LitStr, inside: &'a str) -> Span {
    let input_offset = (inside.as_ptr() as usize) - (input.as_ptr() as usize);
    let range = input_offset + 1..input_offset + inside.len() + 1;
    subspan(input_lit_str.span(), range).unwrap_or_else(|| input_lit_str.span())
}

/// Returns a subspan of the given span.
///
/// `Span` itself has no public `subspan` method; only `Literal` does. So we stash `span` onto a
/// throwaway literal and delegate to its `subspan` instead.
fn subspan<R: RangeBounds<usize>>(span: Span, range: R) -> Option<Span> {
    let mut lit = proc_macro2::Literal::i8_suffixed(0); // wtf...
    lit.set_span(span);
    lit.subspan(range)
}
