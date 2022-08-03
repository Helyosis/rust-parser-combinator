// This module adds differents way to build basic parser. Focused on strings/printable text.

mod digits;
mod str;
mod whitespace;

#[allow(non_upper_case_globals)]
mod special;

pub use self::str::*;
pub use digits::*;
pub use special::*;
pub use whitespace::*;
