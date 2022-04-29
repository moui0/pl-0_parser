mod lexer;
mod parser;

pub use lexer::Lexer;
use parser::Parser;

fn main() {
    let parser = Parser::new("sample/sample0.pl0");
    parser.gen_ast();
    parser.ast_root.borrow().print_tree();
}
