mod lexer;
mod parser;

pub use lexer::Lexer;
use parser::Parser;

fn main() {
    let parser = Parser::new("sample/sample0.pl0");
    while let Ok(sym) = parser.get_sym() {
        println!("{:?}", sym);
    }
}
