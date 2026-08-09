mod color;
mod style;

use std::sync::LazyLock;

pub use color::*;
pub use style::*;
use terminfo_crate::Database;

/// The terminfo database.
static TERMINFO: LazyLock<Option<Database>> = LazyLock::new(|| Database::from_env().ok());
