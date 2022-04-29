mod lexer;
mod parser;

use std::env;

pub use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("{}", file_path);
    let parser = Parser::new(&file_path);
    parser.gen_ast();
    parser.ast_root.borrow().print_tree();
}
