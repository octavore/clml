//! Demonstrates the `anstream` feature.
//!
//! Run it both ways, with stdout redirected to a file (i.e. not a terminal):
//!
//! ```text
//! cargo run --example stream                          | cat -v   # escape codes stripped
//! cargo run --example stream --no-default-features    | cat -v   # escape codes present
//! ```
//!
//! With the feature (on by default) the macros write through `anstream::AutoStream`, which strips
//! the codes when the destination is not a terminal (and honours `NO_COLOR`, `CLICOLOR`, and legacy
//! Windows consoles). Without it the macros write to `std::io::stdout` and the ANSI codes go out
//! verbatim.

use clml::cprintln;

fn main() {
    let target = "clml";
    cprintln!("<green,bold>Finished</> building <cyan>{target}</cyan>");
}
