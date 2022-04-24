use std::cell::RefCell;

use crate::{Lexer, lexer::Symbols, reach_eof};

pub struct Parser {
    lexer: RefCell<Lexer>,
}

impl Parser {
    pub fn new(file_path: &str) -> Self {
        Parser { 
            lexer: RefCell::new(Lexer::new(file_path)),

        }
    }

    pub fn get_sym(&self) -> Symbols {
        if !reach_eof(&self.lexer.borrow()) {
            self.lexer.borrow_mut().get_sym()
        } else {
            Symbols::Nul
        }
    }
}
