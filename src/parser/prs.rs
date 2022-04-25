use std::cell::RefCell;

use crate::{Lexer, lexer::{Symbol, self}};

use super::ast::ASTNode;

#[derive(Debug)]
pub struct Parser {
    lexer: RefCell<Lexer>,
    pub ast_root: RefCell<ASTNode>,
}

impl Parser {
    pub fn new(file_path: &str) -> Self {
        Parser { 
            lexer: RefCell::new(Lexer::new(file_path)),
            ast_root: RefCell::new(ASTNode::new(Symbol::Root)),
        }
    }

    pub fn generate_ast(&self) {
        let mut lexer = self.lexer.borrow_mut();
        while let Ok(sym) = lexer.get_sym() {
            match sym {
                Symbol::Varsym(v) => {
                    let mut ast_node = ASTNode::new(Symbol::Varsym(v));
                    ident(&mut *lexer, &mut ast_node);
                    becomes(&mut *lexer, &mut ast_node);
                    number(&mut *lexer, &mut ast_node);
                    self.ast_root.borrow_mut().child.push(Box::new(ast_node));
                },
                _ => {
                    
                }
            }
        }
    }
}

fn error_hander() -> ! {
    panic!("Parse error.")
}

pub fn ident(lexer: &mut Lexer, node: &mut ASTNode) {
    if let Ok(sym) = lexer.get_sym() {
        match sym {
            Symbol::Ident(ident) => {
                node.child.push(Box::new(ASTNode::new(Symbol::Ident(ident))));
            },
            _ => {
                error_hander();
            }
        }
    } else {
        error_hander();
    }
}

pub fn becomes(lexer: &mut Lexer, node: &mut ASTNode) {
    if let Ok(sym) = lexer.get_sym() {
        match sym {
            Symbol::Becomes(becomes) => {
                node.child.push(Box::new(ASTNode::new(Symbol::Becomes(becomes))));
            },
            _ => {
                error_hander();
            }
        }
    } else {
        error_hander();
    }
}

pub fn number(lexer: &mut Lexer, node: &mut ASTNode) {
    if let Ok(sym) = lexer.get_sym() {
        match sym {
            Symbol::Number(number) => {
                node.child.push(Box::new(ASTNode::new(Symbol::Number(number))));
            },
            _ => {
                error_hander();
            }
        }
    } else {
        error_hander();
    }
}
