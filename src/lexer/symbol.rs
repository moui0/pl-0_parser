use num_enum::FromPrimitive;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Symbols {
    Beginsym,
    #[num_enum(default)]
    Callsym,
    Constsym,
    Dosym,
    Endsym,
    Ifsym,
    Oddsym,
    Procsym,
    Readsym,
    Thensym,
	Varsym,
    Whilesym,
	Writesym,
    
    Ident,
    Number,

    Plus,
    Minus,
	Times,
    Slash,
    Eql,
    Neq,
	Lss,
    Leq,
    Gtr,
    Geq,
    Becomes,

    Lparen,
	Rparen,
    Comma,
    Semicolon,
    Period,
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
