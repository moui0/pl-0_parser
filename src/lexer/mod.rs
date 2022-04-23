mod lex;
mod symbol;

pub use lex::Lexer;
pub use symbol::{KEYWORDS, Symbols};

pub fn reach_eof(lexer: &Lexer) -> bool {
    lexer.pos == lexer.content.len() as usize
}

pub fn is_keywords(sym: &str) -> Result<usize, usize> {
    KEYWORDS.binary_search(&sym)
}
