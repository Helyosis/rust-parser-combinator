mod combinator;
mod parser_core;
mod sequence;
mod string;

pub use combinator::*;
pub use parser_core::*;
pub use sequence::*;
pub use string::*;

fn main() {
    let input = "\"Hey salut! :)\"".to_string();
    let p = quoted_string();

    println!("{:?}", p.run(input, 0))
}
