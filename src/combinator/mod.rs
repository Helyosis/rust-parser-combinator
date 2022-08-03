/// This module add differents way to combine differents parsers to make one bigger parser.
mod any;
mod combine;
mod pair;
mod triplets;

pub use any::*;
pub use combine::*;
pub use pair::*;
pub use triplets::*;
