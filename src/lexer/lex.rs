use std::fs;
use super::symbol::Symbol;

#[derive(Debug)]
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

    pub fn get_char(&mut self) -> Result<char, ()> {
        if self.pos == self.content.len() {
            Err(())
        } else {
            self.pos += 1;
            Ok(self.content[self.pos-1])
        }
    }

    pub fn step_back(&mut self) {
        self.pos -= 1;
    }
    
    pub fn get_sym(&mut self) -> Result<Symbol, ()> {
        let mut s = String::new();
        let mut c = self.get_char().unwrap();
        // Remove whitespace before character.
        while c.is_ascii_whitespace() {
            if let Ok(r) = self.get_char() {
                c = r;
            } else {
                return Err(())
            }
        }

        let symbol = match c {
            'a'..='z' | 'A'..='Z' => {
                s.push(c);
                c = self.get_char().unwrap();
                while c.is_alphanumeric() {
                    s.push(c);
                    c = self.get_char().unwrap();
                }
                self.step_back();
                Symbol::new_keyword_or_ident(s)
            }
            '0'..='9' => {
                while c.is_ascii_digit() {
                    s.push(c);
                    c = self.get_char().unwrap();
                }
                self.step_back();
                Symbol::Number(s)
            }
            '<' => {
                s.push(c);
                c = self.get_char().unwrap();
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
                c = self.get_char().unwrap();
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
                c = self.get_char().unwrap();
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
                println!("%%%{}%%%", c as u8);
                Symbol::Nul
            }
        };
        Ok(symbol)
    }
}

