#[derive(Debug)]
pub enum Symbols {
    Keyword(String),
    Ident(String),
    Number(String),
    Operator(String),
    Delimiter(String),
    Nul,
}

pub const KEYWORDS: [&str; 13] = [
    "begin",
    "call",
    "const",
    "do",
    "end",
    "if",
    "odd",
    "procedure",
    "read",
    "then",
    "var",
    "while",
    "write"
];
