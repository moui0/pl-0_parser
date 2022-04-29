use std::fs;
use super::symbol::Symbol;

#[derive(Debug)]
pub struct Lexer {
    pub content: Vec<char>,
    pub pos: usize,
    pub old_pos: usize,
}

impl Lexer {
    pub fn new(file_path: &str) -> Self {
        Lexer { 
            content: fs::read_to_string(file_path).unwrap().chars().collect(),
            pos: 0,
            old_pos: 0,
        }
    }

    pub fn get_char(&mut self) -> char {
        self.pos += 1;
        self.content[self.pos-1]
    }

    pub fn step_back(&mut self) {
        self.pos -= 1;
    }

    pub fn back(&mut self) {
        self.pos = self.old_pos;
    }
    
    pub fn get_sym(&mut self) -> Symbol {
        self.old_pos = self.pos;
        let mut s = String::new();
        let mut c = self.get_char();
        // Remove whitespace before character.
        while c.is_ascii_whitespace() {
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
                Symbol::new_keyword_or_ident(s)
            }
            '0'..='9' => {
                while c.is_ascii_digit() {
                    s.push(c);
                    c = self.get_char();
                }
                self.step_back();
                Symbol::Number(s)
            }
            '<' => {
                s.push(c);
                c = self.get_char();
                if c == '=' {
                    s.push(c);
                    Symbol::Leq(s)
                } else {
                    self.step_back();
                    Symbol::Lss(s)
                }
            }
            '>' => {
                s.push(c);
                c = self.get_char();
                if c == '=' {
                    s.push(c);
                    Symbol::Geq(s)
                } else {
                    self.step_back();
                    Symbol::Gtr(s)
                }
            }
            ':' => {
                s.push(c);
                c = self.get_char();
                if c == '=' {
                    s.push(c);
                    Symbol::Becomes(s)
                } else {
                    Symbol::Nul
                }
            }
            '=' => {
                s.push(c);
                Symbol::Eql(s)
            }
            '#' => {
                s.push(c);
                Symbol::Neq(s)
            }
            '+' => {
                s.push(c);
                Symbol::Plus(s)
            }
            '-' => {
                s.push(c);
                Symbol::Minus(s)
            }
            '*' => {
                s.push(c);
                Symbol::Times(s)
            }
            '/' => {
                s.push(c);
                Symbol::Slash(s)
            }
            '(' => {
                s.push(c);
                Symbol::Lparen(s)
            }
            ')' => {
                s.push(c);
                Symbol::Rparen(s)
            }
            ',' => {
                s.push(c);
                Symbol::Comma(s)
            }
            ';' => {
                s.push(c);
                Symbol::Semicolon(s)
            }
            '.' => {
                s.push(c);
                Symbol::Period(s)
            }
            _ => {
                Symbol::Nul
            }
        };
        println!("{:#?}", symbol);
        symbol
    }
}

