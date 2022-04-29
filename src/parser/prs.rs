use std::cell::RefCell;

use crate::{Lexer, lexer::Symbol};

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
            ast_root: RefCell::new(ASTNode::new(String::from("root"))),
        }
    }

    pub fn gen_ast(&self) {
        self.ast_root.borrow_mut().child.push(Box::new(self.gen_program()));
    }

    fn get_sym(&self) -> Symbol {
        self.lexer.borrow_mut().get_sym()
    }

    fn back(&self) {
        self.lexer.borrow_mut().back();
    }

    fn gen_program(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("sub_program"));
        match self.get_sym() {
            Symbol::Constsym(_) => {
                self.back();
                node.child.push(Box::new(self.gen_const_decl()));
            }
            _ => self.back(),
        }
        match self.get_sym() {
            Symbol::Varsym(_) => {
                self.back();
                node.child.push(Box::new(self.gen_var_decl()));
            }
            _ => self.back(),
        }
        match self.get_sym() {
            Symbol::Procsym(_) => {
                self.back();
                node.child.push(Box::new(self.gen_proc_decl()));
            }
            _ => self.back(),
        }
        node.child.push(Box::new(self.gen_stmt()));
        node
    }

    fn gen_const_decl(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Constsym(s) => {
                let mut node = ASTNode::new(String::from("const_decl"));
                node.child.push(Box::new(ASTNode::new(s)));
                loop {
                    node.child.push(Box::new(self.gen_const_def()));
                    match self.get_sym() {
                        Symbol::Semicolon(_) => {
                            node.child.push(Box::new(self.gen_smcol(true)));
                            break;
                        }
                        Symbol::Comma(_) => node.child.push(Box::new(self.gen_comma(true))),
                        _ => error_hander(),
                    }
                }
                node
            }
            _ => error_hander(),
        }
    }

    fn gen_const_def(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("const_def"));
        node.child.push(Box::new(self.gen_ident()));
        node.child.push(Box::new(self.gen_eql()));
        node.child.push(Box::new(self.gen_uint()));
        node
    }

    fn gen_ident(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Ident(id) => ASTNode::new(id),
            _ => error_hander(),
        }
    }

    fn gen_number(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Number(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_eql(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Eql(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_uint(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Number(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_var_decl(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Varsym(s) => {
                let mut node = ASTNode::new(String::from("var_decl"));
                node.child.push(Box::new(ASTNode::new(s)));
                loop {
                    node.child.push(Box::new(self.gen_ident()));
                    match self.get_sym() {
                        Symbol::Semicolon(_) => {
                            node.child.push(Box::new(self.gen_smcol(true)));
                            break;
                        }
                        Symbol::Comma(_) => node.child.push(Box::new(self.gen_comma(true))),
                        _ => error_hander(),
                    }
                }
                node
            }
            _ => error_hander(),
        }
    }

    fn gen_proc_decl(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("proc_decl"));
        node.child.push(Box::new(self.gen_proc_header()));
        node.child.push(Box::new(self.gen_program()));
        node.child.push(Box::new(self.gen_smcol(false)));
        let sym = self.get_sym();
        self.back();
        if matches!(sym, Symbol::Procsym(_)) {
            node.child.push(Box::new(self.gen_proc_decl()));
        }
        node
    }

    fn gen_proc_header(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Procsym(s) => {
                let mut node = ASTNode::new(String::from("proc_header"));
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_ident()));
                node.child.push(Box::new(self.gen_smcol(false)));
                node
            }
            _ => error_hander(),
        }
    }

    fn gen_smcol(&self, back: bool) -> ASTNode {
        if back {
            self.back();
        }
        match self.get_sym() {
            Symbol::Semicolon(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_comma(&self, back: bool) -> ASTNode {
        if back {
            self.back();
        }
        match self.get_sym() {
            Symbol::Comma(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_stmt(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("stmt"));
        let sym = self.get_sym();
        self.back();
        match sym {
            Symbol::Ident(_)    => node.child.push(Box::new(self.gen_stmt_assgin())),
            Symbol::Ifsym(_)    => node.child.push(Box::new(self.gen_stmt_cond())),
            Symbol::Whilesym(_) => node.child.push(Box::new(self.gen_stmt_loop())),
            Symbol::Callsym(_)  => node.child.push(Box::new(self.gen_stmt_call())),
            Symbol::Readsym(_)  => node.child.push(Box::new(self.gen_stmt_read())),
            Symbol::Writesym(_) => node.child.push(Box::new(self.gen_stmt_write())),
            Symbol::Beginsym(_) => node.child.push(Box::new(self.gen_stmt_comp())),
            _ => node.child.push(Box::new(self.gen_stmt_empty())),
        }
        node
    }

    fn gen_stmt_assgin(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("assign_stmt"));
        node.child.push(Box::new(self.gen_ident()));
        node.child.push(Box::new(self.gen_becomes()));
        node.child.push(Box::new(self.gen_expr()));
        node
    }

    fn gen_becomes(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Becomes(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_expr(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("expr"));
        for i in 0.. {
            match self.get_sym() {
                Symbol::Plus(s) | 
                Symbol::Minus(s) => {
                    node.child.push(Box::new(ASTNode::new(s)));
                }
                _ => {
                    self.back();
                    if i != 0 {
                        break;
                    }
                }
            }
            node.child.push(Box::new(self.gen_nomial()));
        }
        node
    }

    fn gen_nomial(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("nomial"));
        node.child.push(Box::new(self.gen_factor()));
        loop {
            match self.get_sym() {
                Symbol::Times(s) |
                Symbol::Slash(s) => {
                    node.child.push(Box::new(ASTNode::new(s)));
                    node.child.push(Box::new(self.gen_nomial()));
                }
                _ => {
                    self.back();
                    break;
                }
            }
        }
        node
    }

    fn gen_factor(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("factor"));
        match self.get_sym() {
            Symbol::Lparen(_) => {
                self.back();
                node.child.push(Box::new(self.gen_lparen()));
                node.child.push(Box::new(self.gen_expr()));
                node.child.push(Box::new(self.gen_rparen()));
            }
            Symbol::Ident(_) => {
                self.back();
                node.child.push(Box::new(self.gen_ident()));
            }
            Symbol::Number(_) => {
                self.back();
                node.child.push(Box::new(self.gen_number()));
            }
            _ => error_hander(),
        }
        node 
    }

    fn gen_lparen(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Lparen(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_rparen(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Rparen(s) => ASTNode::new(s),
            _ => error_hander(),
        }
    }

    fn gen_stmt_cond(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Ifsym(s) => {
                let mut node = ASTNode::new(String::from("cond_stmt"));
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_cond()));
                match self.get_sym() {
                    Symbol::Thensym(s) => {
                        node.child.push(Box::new(ASTNode::new(s)));
                        node.child.push(Box::new(self.gen_stmt()));
                        node
                    }
                    _ => error_hander(),
                }
            }
            _ => error_hander(),
        }
    }

    fn gen_cond(&self) -> ASTNode {
        let mut node = ASTNode::new(String::from("cond"));
        match self.get_sym() {
            Symbol::Oddsym(s) => {
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_expr()));
            }
            _ => {
                self.back();
                node.child.push(Box::new(self.gen_expr()));
                match self.get_sym() {
                    Symbol::Lss(s) |
                    Symbol::Leq(s) |
                    Symbol::Gtr(s) |
                    Symbol::Geq(s) |
                    Symbol::Eql(s) |
                    Symbol::Neq(s)  => node.child.push(Box::new(ASTNode::new(s))),
                    _ => error_hander(),
                }
                node.child.push(Box::new(self.gen_expr()));
            }
        }
        node
    }

    fn gen_stmt_loop(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Whilesym(s) => {
                let mut node = ASTNode::new(String::from("while_stmt"));
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_cond()));
                match self.get_sym() {
                    Symbol::Dosym(s) => {
                        node.child.push(Box::new(ASTNode::new(s)));
                        node.child.push(Box::new(self.gen_stmt()));
                        node
                    }
                    _ => error_hander(),
                }
            }
            _ => error_hander(),
        }
    }

    fn gen_stmt_call(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Callsym(s) => {
                let mut node = ASTNode::new(String::from("call_stmt"));
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_ident()));
                node
            }
            _ => error_hander(),
        }
    }

    fn gen_stmt_read(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Readsym(s) => {
                let mut node = ASTNode::new(String::from("read_stmt"));
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_lparen()));
                loop {
                    match self.get_sym() {
                        Symbol::Ident(_) => {
                            self.back();
                            node.child.push(Box::new(self.gen_ident()));
                        }
                        Symbol::Comma(_) => node.child.push(Box::new(self.gen_comma(true))),
                        _ => {
                            self.back();
                            break;
                        }
                    }
                }
                node.child.push(Box::new(self.gen_rparen()));
                node
            }
            _ => error_hander(),
        }
    }

    fn gen_stmt_write(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Writesym(s) => {
                let mut node = ASTNode::new(String::from("write_stmt"));
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_lparen()));
                loop {
                    match self.get_sym() {
                        Symbol::Ident(_) => {
                            self.back();
                            node.child.push(Box::new(self.gen_ident()));
                        }
                        Symbol::Comma(_) => node.child.push(Box::new(self.gen_comma(true))),
                        _ => {
                            self.back();
                            break;
                        }
                    }
                }
                node.child.push(Box::new(self.gen_rparen()));
                node
            }
            _ => error_hander(),
        }
    }

    fn gen_stmt_comp(&self) -> ASTNode {
        match self.get_sym() {
            Symbol::Beginsym(s) => {
                let mut node = ASTNode::new(String::from("comp_stmt"));
                node.child.push(Box::new(ASTNode::new(s)));
                node.child.push(Box::new(self.gen_stmt()));
                loop {
                    match self.get_sym() {
                        Symbol::Semicolon(_) => {
                            node.child.push(Box::new(self.gen_smcol(true)));
                            node.child.push(Box::new(self.gen_stmt()));
                        }
                        _ => {
                            self.back();
                            break;
                        }
                    }
                }
                match self.get_sym() {
                    Symbol::Endsym(s) => {
                        node.child.push(Box::new(ASTNode::new(s)));
                    }
                    _ => error_hander(),
                }
                node
            }
            _ => error_hander(),
        }
    }

    fn gen_stmt_empty(&self) -> ASTNode {
        ASTNode::new(String::from("empty_stmt"))
    }
}

fn error_hander() -> ! {
    panic!("Parse error.");
}
