use std::fs;
use super::symbol::Symbols;
use super::{reach_eof, is_keywords};

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
    pub fn get_sym(&mut self) -> Symbols {
        let mut s = String::new();
        let mut c = self.get_char();
        while c.is_whitespace() && !reach_eof(self) {
            c = self.get_char();
        }
        match c {
            'a'..='z' | 'A'..='Z' => {
                s.push(c);
                c = self.get_char();
                while c.is_ascii_alphanumeric() {
                    s.push(c);
                    c = self.get_char();
                }
                self.step_back();
                if let Ok(_r) = is_keywords(&s) {// keyword
                    Symbols::Keyword(s)
                } else {// ident
                    Symbols::Ident(s)
                }
            }
            '0'..='9' => {
                while c.is_ascii_digit() {
                    s.push(c);
                    c = self.get_char();
                }
                self.step_back();
                Symbols::Number(s)
            }
            '+' | '-' | '*' | '/' | '=' => {
                s.push(c);
                Symbols::Operator(s)
            }
            '<' | '>' | ':' => {
                s.push(c);
                c = self.get_char();
                if c == '=' {
                    s.push(c);
                } else {
                    self.step_back();
                }
                Symbols::Operator(s)
            }
            '(' | ')' | ';' | '.' | ',' => {
                s.push(c);
                Symbols::Delimiter(s)
            }
            _ => {
                Symbols::Nul
            }
        }
    }
}
