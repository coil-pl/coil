#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Identifier(Box<str>),
    Keyword(Keyword),
    Literal(Literal, Box<str>),
    Operator(Operator),
    Parenthesis { closing: bool, kind: Parenthesis },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Break,
    Consttime,
    Continue,
    Do,
    Else,
    Enum,
    Extern,
    Fallthrough,
    False,
    Fn,
    For,
    If,
    Impl,
    Import,
    In,
    Is,
    Launch,
    Let,
    Match,
    Module,
    Mut,
    Pub,
    Return,
    SelfType,
    Static,
    Struct,
    Trait,
    True,
    Type,
    Union,
    Unless,
    Unsafe,
    Where,
    While,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    String,
    Integer { radix: usize },
    Float { radix: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Dot,          // .
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    DoubleDot,    // ..
    QuestionMark, // ?
    Arrow,        // ->
    Bolt,         // =>
    Backslash,    // \
    Plus,
    PlusAssign,
    Minus,
    MinusAssign,
    Star,
    StarAssign,
    Slash,
    SlashAssign,
    Percent,
    PercentAssign,
    Eq,
    NotEq,
    Greater,
    GreaterEq,
    Lesser,
    LesserEq,
    Not,
    And,
    AndAssign,
    Or,
    OrAssign,
    BitNot,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    BitShiftLeft,
    BitShiftLeftAssign,
    BitShiftRight,
    BitShiftRightAssign,
    Assign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Parenthesis {
    Normal,
    Square,
    Curly,
}
