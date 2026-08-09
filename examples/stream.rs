//! Demonstrates the `anstream` feature.
//!
//! Run it both ways, with stdout redirected to a file (i.e. not a terminal):
//!
//! ```text
//! cargo run --example stream                     | cat -v   # escape codes present
//! cargo run --example stream --features anstream | cat -v   # escape codes stripped
//! ```
//!
//! Without the feature the macros write to `std::io::stdout` and the ANSI codes go out verbatim.
//! With it they write through `anstream::AutoStream`, which strips them when the destination is not
//! a terminal (and honours `NO_COLOR`, `CLICOLOR`, and legacy Windows consoles).

use clml::cprintln;

fn main() {
    let target = "clml";
    cprintln!("<green,bold>Finished</> building <cyan>{target}</cyan>");
}
