mod lexer;
mod parser;
mod config;

use std::{env, process};

use lexer::Lexer;
use parser::Parser;
use config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let parser = Parser::new(&config.filename);
    parser.gen_ast();
    parser.ast_root.borrow().print_ast();
}
