


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
    pub fn step_back(&mut self) {
        self.pos -= 1;
    }
    pub fn get_sym(&mut self) -> String {
        let mut sym = String::new();
        let mut c = self.get_char();
        while c.is_whitespace() && !reach_eof(self) {
            c = self.get_char();
        }
        if c.is_ascii_alphabetic() {// keyword or ident
            sym.push(c);
            c = self.get_char();
            while c.is_ascii_alphanumeric() {
                sym.push(c);
                c = self.get_char();
            }
            if let Ok(_r) = is_keywords(&sym) {
                println!("keyword: {}", sym);
            } else {
                println!("ident: {}", sym);
            }
            self.step_back();
        } else if c.is_ascii_digit() {// number
            while c.is_ascii_digit() {
                sym.push(c);
                c = self.get_char();
            }
            println!("number: {}", sym);
        } else if c == '+' {// plus
            sym.push(c);
            println!("plus: {}", sym);
        } else if c == '-' {// minus 
            sym.push(c);
            println!("minus: {}", sym);
        } else if c == '*' {// times
            sym.push(c);
            println!("times: {}", sym);
        } else if c == '/' {// slash
            sym.push(c);
            println!("slash: {}", sym);
        } else if c == '<' {// lss or leq
            sym.push(c);
            c = self.get_char();
            if c == '=' {// leq
                sym.push(c);
                println!("leq: {}", sym);
            } else {// lss
                println!("lss: {}", sym);
                self.step_back();
            }
        } else if c == '>' {// gtr or geq
            sym.push(c);
            c = self.get_char();
            if c == '=' {// geq
                sym.push(c);
                println!("geq: {}", sym);
            } else {// gtr
                println!("gtr: {}", sym);
                self.step_back();
            }
        } else if c == '#' {
            
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