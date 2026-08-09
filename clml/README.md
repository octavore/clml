# clml

Colorize and stylize strings for the terminal at compile-time, using an HTML-like syntax.

```rust
use clml::cprintln;

cprintln!("Hello <green>world</green>!");
cprintln!("Hello <green>world</>!"); // `</>` closes the last open tag
```

The tags are resolved at compile time into ANSI escape sequences, so there is no runtime cost and
no runtime state to configure.

## Composability

`clml` is designed to be wrapped. Its macros keep working when you re-export them through your own
`macro_rules!` — the usual way to route output through a tty-aware adapter such as
[`anstream`](https://crates.io/crates/anstream):

```rust
macro_rules! status {
    ($($arg:tt)*) => { ::clml::cprintln!($($arg)*) };
}

let package = "clml";
status!("<green>Compiling</green> {package}");
```

Implicit named captures (`{package}`) resolve against the caller's scope, not the wrapper's, because
the rewritten format string carries the span of the literal you actually wrote. This is the same
span-preservation that `format_args!` relies on, and it is what makes the wrapper above compile.

## Macros

Each macro mirrors its `std` counterpart, accepting the same positional and named arguments:

| `clml`        | `std`        |
| ------------- | ------------ |
| `cformat!`    | `format!`    |
| `cprint!`     | `print!`     |
| `cprintln!`   | `println!`   |
| `ceprint!`    | `eprint!`    |
| `ceprintln!`  | `eprintln!`  |
| `cwrite!`     | `write!`     |
| `cwriteln!`   | `writeln!`   |

Two macros have no `std` equivalent:

- `cstr!(...)` turns a string literal into another string literal with the tags replaced by ANSI
  sequences, without formatting placeholders.
- `untagged!(...)` strips all tags from a string literal.

## Tags

Colors have four variants, spelled consistently:

- `<red>` — foreground
- `<bright-red>` or `<red!>` — bright foreground
- `<bg:red>` or `<RED>` — background
- `<bg:bright-red>`, `<bg:red!>` or `<RED!>` — bright background

Every color has a one-letter shortcut, its first letter (`<k>` for black): `<y>` is `<yellow>`,
`<Y!>` is `<bg:bright-yellow>`. Uppercase means background; a trailing `!` means bright.

Styles: `<strong>`/`<em>`/`<bold>`/`<s>`, `<dim>`, `<underline>`/`<u>`, `<italics>`/`<i>`,
`<blink>`, `<reverse>`/`<rev>`, `<strike>`, `<conceal>`/`<hide>`.

256-color and true-color are supported: `<palette(42)>` (aliases `<p(...)>`, `<pal(...)>`, or just
`<42>`), `<rgb(10,20,30)>`, and `<#a0b0c0>`.

Nested tags work as you would expect, and unclosed tags are closed automatically at the end of the
string. Consecutive tags are collapsed so no redundant escape sequences are emitted. Errors — an
unknown color, a mismatched close tag — are reported at compile time, pointing into the format
string.

See the [full tag table](https://docs.rs/clml) for every accepted spelling.

## The `anstream` feature

By default the printing macros write to `std::io::stdout`/`stderr` and the escape codes go out
verbatim. Enable the `anstream` feature to route them through
[`anstream`](https://crates.io/crates/anstream) instead:

```toml
clml = { version = "0.1", features = ["anstream"] }
```

Call sites don't change. What changes is the destination: codes are stripped when stdout isn't a
terminal, `NO_COLOR`/`CLICOLOR`/`CLICOLOR_FORCE` are honored, and legacy Windows consoles get
console API calls instead of escape sequences.

```console
$ cargo run --example stream | cat -v
^[[32m^[[1mFinished^[[39m^[[22m building ^[[36mclml^[[39m

$ cargo run --example stream --features anstream | cat -v
Finished building clml
```

This works because `cprintln!` expands to `clml::__private::println!`, which the feature re-points
at `anstream`. Only the four printing macros are affected — `cformat!`, `cstr!`, and the `cwrite!`
family produce values rather than writing to a stream, so they're unchanged.

## License

MIT. `clml` began as a fork of [`color-print`](https://gitlab.com/dajoha/color-print) by Johann
David, which is dual-licensed MIT OR Apache-2.0; it is used here under the MIT terms.
