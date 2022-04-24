
mod lexer;
mod parser;

pub use lexer::{Lexer, reach_eof};
use parser::Parser;

extern crate num_enum;

fn main() {
    let parser = Parser::new("sample/sample0.pl0");
}
