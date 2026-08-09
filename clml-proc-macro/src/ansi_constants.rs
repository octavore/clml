//! ANSI constants.

// pub const RESET: u8 = 0;
pub const BOLD: u8 = 1;
pub const DIM: u8 = 2;
pub const ITALIC: u8 = 3;
pub const UNDERLINE: u8 = 4;
pub const BLINK: u8 = 5;
pub const REVERSE: u8 = 7;
pub const CONCEAL: u8 = 8;
pub const STRIKE: u8 = 9;
pub const NO_BOLD: u8 = 22;
pub const NO_ITALIC: u8 = 23;
pub const NO_UNDERLINE: u8 = 24;
pub const NO_BLINK: u8 = 25;
pub const NO_REVERSE: u8 = 27;
pub const NO_CONCEAL: u8 = 28;
pub const NO_STRIKE: u8 = 29;
pub const SET_FOREGROUND_BASE: u8 = 30;
pub const SET_FOREGROUND: u8 = 38;
pub const DEFAULT_FOREGROUND: u8 = 39;
pub const SET_BACKGROUND_BASE: u8 = 40;
pub const SET_BACKGROUND: u8 = 48;
pub const DEFAULT_BACKGROUND: u8 = 49;
pub const SET_BRIGHT_FOREGROUND_BASE: u8 = 90;
pub const SET_BRIGHT_BACKGROUND_BASE: u8 = 100;

/// Generate an SGR ANSI sequence.
pub fn generate_ansi_code(params: &[u8]) -> String {
    let params = params
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(";");
    format!("\u{1b}[{params}m")
}

/// Generate an OSC 8 hyperlink sequence. Passing an empty `url` closes the previously opened link,
/// as specified by the OSC 8 convention.
pub fn generate_osc8_link(url: &str) -> String {
    format!("\u{1b}]8;;{url}\u{1b}\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ansi_code() {
        assert_eq!(generate_ansi_code(&[0]), "\u{1b}[0m");
        assert_eq!(generate_ansi_code(&[31]), "\u{1b}[31m");
        assert_eq!(generate_ansi_code(&[38, 5, 1]), "\u{1b}[38;5;1m");
    }

    #[test]
    fn osc8_link() {
        assert_eq!(
            generate_osc8_link("https://example.com"),
            "\u{1b}]8;;https://example.com\u{1b}\\"
        );
        assert_eq!(generate_osc8_link(""), "\u{1b}]8;;\u{1b}\\");
    }
}
