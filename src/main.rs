mod lexer;
mod parser;

pub use lexer::{Lexer, reach_eof};

fn main() {
    let mut lexer = Lexer::new("sample/sample2.pl0");
    while !reach_eof(&lexer) {
        lexer.get_sym();
    }
}
