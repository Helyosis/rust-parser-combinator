// This module adds differents way to build basic parser. Focused on strings/printable text.

mod digits;
mod letters;
mod special;
mod str;
mod whitespace;

pub use self::str::*;
pub use digits::*;
pub use letters::*;
pub use special::*;
pub use whitespace::*;
