# CLML

CLML is a Rust library for coloring and styling strings for the terminal at compile-time, using an HTML-like syntax. Based on [`color-print`](https://gitlab.com/dajoha/color-print). CLML stands for `command-line markup language`.

Differences from `color-print`:

- the macros can be wrapped in other macros
- native support for [`anstream`](https://crates.io/crates/anstream), which strips ANSI codes when the output does not support them
- support for OSC 8 hyperlinks

```rust
use clml::cprintln;

cprintln!("Hello <green>world</green>!");
cprintln!("Hello <green>world</>!"); // `</>` closes the last open tag
```

The macros resolve tags into ANSI escape sequences at compile time, so there is no runtime cost and no runtime state to configure.

## Macros

Each macro mirrors its `std` counterpart and accepts the same positional and named arguments:

| `clml`        | `std`        |
| ------------- | ------------ |
| `cformat!`    | `format!`    |
| `cprint!`     | `print!`     |
| `cprintln!`   | `println!`   |
| `ceprint!`    | `eprint!`    |
| `ceprintln!`  | `eprintln!`  |
| `cwrite!`     | `write!`     |
| `cwriteln!`   | `writeln!`   |

Additionally, this package adds two new macros:

- `cstr!(...)` replaces the tags in a string literal with ANSI sequences and returns a new string literal. It does not process formatting placeholders.
- `untagged!(...)` removes all tags from a string literal.

## Tags

Colors have four variants:

- `<red>` for foreground
- `<bright-red>` or `<red!>` for bright foreground
- `<bg:red>` or `<RED>` for background
- `<bg:bright-red>`, `<bg:red!>`, or `<RED!>` for bright background

Every color has a one-letter shortcut. The shortcut is the color's first letter, except for black, which uses `<k>`. For example, `<y>` is `<yellow>` and `<Y!>` is `<bg:bright-yellow>`. Uppercase means background, and a trailing `!` means bright.

Styles: `<strong>`/`<em>`/`<bold>`/`<s>`, `<dim>`, `<underline>`/`<u>`, `<italics>`/`<i>`, `<blink>`, `<reverse>`/`<rev>`, `<strike>`, `<conceal>`/`<hide>`.

CLML supports 256-color and true-color output: `<palette(42)>` (aliases `<p(...)>`, `<pal(...)>`, or just `<42>`), `<rgb(10,20,30)>`, and `<#a0b0c0>`.

Hyperlinks use the OSC-8 escape sequence, which most modern terminal emulators understand. `<link(https://example.com)>text</>` makes `text` a clickable link.

Tags can nest, and macros close any unclosed tags at the end of the string. The macros also skip escape sequences that would not change the current style. Errors such as unknown tags or mismatched closing tags are surfaced at compile time and point into format string.

## Composability

These macros work inside your own `macro_rules!`:

```rust
macro_rules! status {
    ($($arg:tt)*) => { ::clml::cprintln!($($arg)*) };
}

let package = "clml";
status!("<green>Compiling</green> {package}");
```

Implicit named captures such as `{package}` resolve against the caller's scope, not the wrapper's.

## `anstream` support

The `anstream` feature is enabled by default. It routes the printing macros through [`anstream`](https://crates.io/crates/anstream), which strips escape codes when the output is not a terminal, honors `NO_COLOR`, `CLICOLOR`, and `CLICOLOR_FORCE`. CLML has not been tested with non-ANSI terminals such as legacy Windows consoles.

Without `anstream`, the printing macros write to `std::io::stdout` or `std::io::stderr` and emit the escape codes verbatim. To opt out:

```toml
clml = { version = "0.3", default-features = false }
```

```console
$ cargo run --example stream | cat -v
Finished building clml

$ cargo run --example stream --no-default-features | cat -v
^[[32m^[[1mFinished^[[39m^[[22m building ^[[36mclml^[[39m
```

## The `doc` feature

The `doc` feature is enabled by default. It adds a `...doc!` variant of every formatting macro: `cformatdoc!`, `cprintdoc!`, `cprintlndoc!`, `ceprintdoc!`, `ceprintlndoc!`, `cwritedoc!`, and `cwritelndoc!`. These macros process tags first, then strip the common leading whitespace from every line, as [`indoc::indoc!()`](https://crates.io/crates/indoc) does. Implicit named captures such as `{name}` work as usual.

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

To opt out, disable default features and re-add `anstream` if you want it:

```toml
clml = { version = "0.3", default-features = false, features = ["anstream"] }
```

## License

MIT. `clml` began as a fork of [`color-print`](https://gitlab.com/dajoha/color-print) by Johann David, which is dual-licensed MIT OR Apache-2.0. This project uses it under the MIT terms.
