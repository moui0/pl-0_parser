use std::cell::RefCell;

use crate::{Lexer, lexer::Symbol};

pub struct Parser {
    lexer: RefCell<Lexer>,
}

impl Parser {
    pub fn new(file_path: &str) -> Self {
        Parser { 
            lexer: RefCell::new(Lexer::new(file_path)),

        }
    }

    pub fn get_sym(&self) -> Result<Symbol, ()>  {
        self.lexer.borrow_mut().get_sym()
    }

    
}
