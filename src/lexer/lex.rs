


use std::fs;
use super::symbol::KEYWORDS;
pub struct Lexer {
    pub content: Vec<char>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(file_path: &str) -> Self {
        Lexer { 
            content: fs::read_to_string(file_path).unwrap().chars().collect(),
            pos: 0,
        }
    }
    pub fn get_char(&mut self) -> char {
        self.pos += 1;
        self.content[self.pos-1]
    }
    pub fn get_sym(&mut self) -> String {
        let mut sym = String::new();
        let mut c = self.get_char();
        while c.is_whitespace() && !reach_eof(self) {
            c = self.get_char();
        }
        if c.is_ascii_alphabetic() {
            sym.push(c);
            c = self.get_char();
            while c.is_ascii_alphanumeric() {
                sym.push(c);
                c = self.get_char();
            }
            if let Ok(_r) = is_keywords(&sym) {
                println!("{} is a keyword", sym);
            } else {
                println!("{} is a ident", sym);
            }
        } else {

        }
        sym
    }
}

pub fn reach_eof(lexer: &Lexer) -> bool {
    lexer.pos == lexer.content.len() as usize
}

pub fn is_keywords(sym: &str) -> Result<usize, usize> {
    KEYWORDS.binary_search(&sym)
}