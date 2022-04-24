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
        let symbol = match c {
            'a'..='z' | 'A'..='Z' => {
                s.push(c);
                c = self.get_char();
                while c.is_alphanumeric() {
                    s.push(c);
                    c = self.get_char();
                }
                self.step_back();
                if let Ok(idx) = is_keywords(&s) {
                    Symbols::from(idx as u8)
                } else {
                    Symbols::Ident
                }
            }
            '0'..='9' => {
                while c.is_ascii_digit() {
                    s.push(c);
                    c = self.get_char();
                }
                self.step_back();
                Symbols::Number
            }
            '<' => {
                s.push(c);
                c = self.get_char();
                if c == '=' {
                    s.push(c);
                    Symbols::Leq
                } else {
                    self.step_back();
                    Symbols::Lss
                }
            }
            '>' => {
                s.push(c);
                c = self.get_char();
                if c == '=' {
                    s.push(c);
                    Symbols::Geq
                } else {
                    self.step_back();
                    Symbols::Gtr
                }
            }
            ':' => {
                s.push(c);
                c = self.get_char();
                if c == '=' {
                    s.push(c);
                    Symbols::Becomes
                } else {
                    Symbols::Nul
                }
            }
            '=' => {
                s.push(c);
                Symbols::Eql
            }
            '#' => {
                s.push(c);
                Symbols::Neq
            }
            '+' => {
                s.push(c);
                Symbols::Plus
            }
            '-' => {
                s.push(c);
                Symbols::Minus
            }
            '*' => {
                s.push(c);
                Symbols::Times
            }
            '/' => {
                s.push(c);
                Symbols::Slash
            }
            '(' => {
                s.push(c);
                Symbols::Lparen
            }
            ')' => {
                s.push(c);
                Symbols::Rparen
            }
            ',' => {
                s.push(c);
                Symbols::Comma
            }
            ';' => {
                s.push(c);
                Symbols::Semicolon
            }
            '.' => {
                s.push(c);
                Symbols::Period
            }
            _ => Symbols::Nul,
        };
        symbol
    }
}

