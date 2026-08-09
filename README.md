# CLML

CLML is a Rust library for coloring and styling strings for the terminal at compile-time, using an HTML-like syntax. Based on [`color-print`](https://gitlab.com/dajoha/color-print). CLML stands for `command-line markup language`.

Key differences include:
- support for composing with other macros
- native support for [`anstream`](https://crates.io/crates/anstream) (automatically remove ANSI codes where not supported)
- support for OSC 8 hyperlinks

```rust
use clml::cprintln;

cprintln!("Hello <green>world</green>!");
cprintln!("Hello <green>world</>!"); // `</>` closes the last open tag
```

The tags are resolved at compile time into ANSI escape sequences, so there is no runtime cost and no runtime state to configure.

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

- `cstr!(...)` turns a string literal into another string literal with the tags replaced by ANSI sequences, without formatting placeholders.
- `untagged!(...)` strips all tags from a string literal.

## Tags

Colors have four variants:

- `<red>` for foreground
- `<bright-red>` or `<red!>` for bright foreground
- `<bg:red>` or `<RED>` for background
- `<bg:bright-red>`, `<bg:red!>` or `<RED!>` for bright background

Every color has a one-letter shortcut, its first letter (`<k>` for black): `<y>` is `<yellow>`, `<Y!>` is `<bg:bright-yellow>`. Uppercase means background; a trailing `!` means bright.

Styles: `<strong>`/`<em>`/`<bold>`/`<s>`, `<dim>`, `<underline>`/`<u>`, `<italics>`/`<i>`, `<blink>`, `<reverse>`/`<rev>`, `<strike>`, `<conceal>`/`<hide>`.

256-color and true-color are supported: `<palette(42)>` (aliases `<p(...)>`, `<pal(...)>`, or just `<42>`), `<rgb(10,20,30)>`, and `<#a0b0c0>`.

Hyperlinks use the OSC 8 escape sequence, understood by most modern terminal emulators: `<link(https://example.com)>text</>` makes `text` a clickable link.

Nested tags work as you would expect, and unclosed tags are closed automatically at the end of the string. Consecutive tags are collapsed so no redundant escape sequences are emitted. Errors (e.g. unknown tags, mismatched close tag) are reported at compile time, pointing into the format string.

## Composability

`clml` is designed to be wrapped. Macros will work when re-exporting them through your own `macro_rules!`, e.g. routing output through a tty-aware adapter such as [`anstream`](https://crates.io/crates/anstream):

```rust
macro_rules! status {
    ($($arg:tt)*) => { ::clml::cprintln!($($arg)*) };
}

let package = "clml";
status!("<green>Compiling</green> {package}");
```

Implicit named captures (e.g. `{package}` above) will resolve against the caller's scope, not the wrapper.

## The `anstream` feature

By default the printing macros write to `std::io::stdout`/`stderr` and the escape codes go out verbatim. Enable the `anstream` feature to route them through [`anstream`](https://crates.io/crates/anstream) instead:

```toml
clml = { version = "0.1", features = ["anstream"] }
```

With anstream, backend codes are stripped when stdout isn't a terminal, `NO_COLOR`/`CLICOLOR`/`CLICOLOR_FORCE` are honored, and legacy Windows consoles get console API calls instead of escape sequences.

```console
$ cargo run --example stream | cat -v
^[[32m^[[1mFinished^[[39m^[[22m building ^[[36mclml^[[39m

$ cargo run --example stream --features anstream | cat -v
Finished building clml
```

## The `doc` feature

Enable the `doc` feature to get a `...doc!` variant of every formatting macro (`cformatdoc!`, `cprintdoc!`, `cprintlndoc!`, `ceprintdoc!`, `ceprintlndoc!`, `cwritedoc!`, `cwritelndoc!`) which dedents the format string the same way [`indoc::indoc!()`](https://crates.io/crates/indoc) does, on top of the usual tag processing:

```toml
clml = { version = "0.1", features = ["doc"] }
```

```rust
use clml::cprintlndoc;

let name = "world";
cprintlndoc!(
    "
    <green>Hello, {name}!</green>
        This line is indented one level further.
    "
);
```

Color tags are resolved first, then the common leading whitespace is stripped from every line, similar to wrapping the format string in `indoc!`.Supports implicit named captures (e.g. `{name}` above).

## License

MIT. `clml` began as a fork of [`color-print`](https://gitlab.com/dajoha/color-print) by Johann David, which is dual-licensed MIT OR Apache-2.0; it is used here under the MIT terms.
