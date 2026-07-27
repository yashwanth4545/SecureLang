#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    Let, Mut, Fn, Return, If, Else, While,
    User, Login, Input, Secure, Authenticate,
    True, False, Class, Interface, Trait, Async, Await, Match, Impl,

    // Types
    Int, StringType, Bool,

    // Identifiers & Literals
    Identifier(String),
    IntLiteral(i64),
    StringLiteral(String),

    // Operators
    Plus, Minus, Star, Slash,
    Assign, Eq, Neq, Lt, Gt, Lte, Gte,

    // Punctuation
    LParen, RParen, LBrace, RBrace,
    Comma, Colon, Semicolon, Arrow,

    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}
