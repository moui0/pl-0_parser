#[derive(Debug)]
pub enum Symbol {
    // keyword
    Beginsym(String),
    Callsym(String),
    Constsym(String),
    Dosym(String),
    Endsym(String),
    Ifsym(String),
    Oddsym(String),
    Procsym(String),
    Readsym(String),
    Thensym(String),
    Varsym(String),
    Whilesym(String),
    Writesym(String),
    // identifier and number
    Ident(String),
    Number(String),
    // operator
    Plus(String),
    Minus(String),
    Times(String),
    Slash(String),
    Eql(String),
    Neq(String),
    Lss(String),
    Leq(String),
    Gtr(String),
    Geq(String),
    Becomes(String),
    // delimiter
    Lparen(String),
    Rparen(String),
    Comma(String),
    Semicolon(String),
    Period(String),
    // null
    Nul,
}

impl Symbol {
    pub fn new_keyword_or_ident(s: String) -> Symbol {
        match &s[..] {
            "begin"     => Symbol::Beginsym(s),
            "call"      => Symbol::Callsym(s),
            "const"     => Symbol::Constsym(s),
            "do"        => Symbol::Dosym(s),
            "end"       => Symbol::Endsym(s),
            "if"        => Symbol::Ifsym(s),
            "odd"       => Symbol::Oddsym(s),
            "procedure" => Symbol::Procsym(s),
            "read"      => Symbol::Readsym(s),
            "then"      => Symbol::Thensym(s),
            "var"       => Symbol::Varsym(s),
            "while"     => Symbol::Whilesym(s),
            "write"     => Symbol::Writesym(s),
            _           => Symbol::Ident(s),
        }
    }
}
