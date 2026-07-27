use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    current_char: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Lexer {
            input: chars,
            current_char,
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) {
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        self.current_char = self.input.next();
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.current_char {
                Some(c) if c.is_whitespace() => {
                    self.advance();
                }
                Some('/') => {
                    if self.peek() == Some('/') {
                        while self.current_char != Some('\n') && self.current_char.is_some() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();

        let line = self.line;
        let column = self.column;

        if let Some(c) = self.current_char {
            let kind = match c {
                '+' => { self.advance(); TokenKind::Plus }
                '-' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        self.advance();
                        TokenKind::Arrow
                    } else {
                        TokenKind::Minus
                    }
                }
                '*' => { self.advance(); TokenKind::Star }
                '/' => { self.advance(); TokenKind::Slash }
                '=' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        TokenKind::Eq
                    } else {
                        TokenKind::Assign
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        TokenKind::Neq
                    } else {
                        return Err(format!("Unexpected character '!' at line {}", self.line));
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        TokenKind::Lte
                    } else {
                        TokenKind::Lt
                    }
                }
                '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        TokenKind::Gte
                    } else {
                        TokenKind::Gt
                    }
                }
                '(' => { self.advance(); TokenKind::LParen }
                ')' => { self.advance(); TokenKind::RParen }
                '{' => { self.advance(); TokenKind::LBrace }
                '}' => { self.advance(); TokenKind::RBrace }
                ',' => { self.advance(); TokenKind::Comma }
                ':' => { self.advance(); TokenKind::Colon }
                ';' => { self.advance(); TokenKind::Semicolon }
                '"' => {
                    self.advance();
                    let mut s = String::new();
                    let mut escape = false;
                    while let Some(ch) = self.current_char {
                        if escape {
                            match ch {
                                'n' => s.push('\n'),
                                't' => s.push('\t'),
                                '\\' => s.push('\\'),
                                '"' => s.push('"'),
                                _ => s.push(ch),
                            }
                            escape = false;
                        } else if ch == '\\' {
                            escape = true;
                        } else if ch == '"' {
                            self.advance();
                            break;
                        } else {
                            s.push(ch);
                        }
                        self.advance();
                    }
                    TokenKind::StringLiteral(s)
                }
                _ if c.is_digit(10) => {
                    let mut num = String::new();
                    while let Some(ch) = self.current_char {
                        if ch.is_digit(10) {
                            num.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    match num.parse::<i64>() {
                        Ok(n) => TokenKind::IntLiteral(n),
                        Err(_) => return Err(format!("Invalid integer literal '{}' at line {}", num, line)),
                    }
                }
                _ if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(ch) = self.current_char {
                        if ch.is_alphanumeric() || ch == '_' {
                            ident.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "let" => TokenKind::Let,
                        "mut" => TokenKind::Mut,
                        "fn" => TokenKind::Fn,
                        "return" => TokenKind::Return,
                        "if" => TokenKind::If,
                        "else" => TokenKind::Else,
                        "while" => TokenKind::While,
                        "user" => TokenKind::User,
                        "login" => TokenKind::Login,
                        "input" => TokenKind::Input,
                        "secure" => TokenKind::Secure,
                        "authenticate" => TokenKind::Authenticate,
                        "true" => TokenKind::True,
                        "false" => TokenKind::False,
                        "class" => TokenKind::Class,
                        "interface" => TokenKind::Interface,
                        "trait" => TokenKind::Trait,
                        "async" => TokenKind::Async,
                        "await" => TokenKind::Await,
                        "match" => TokenKind::Match,
                        "impl" => TokenKind::Impl,
                        "int" => TokenKind::Int,
                        "string" => TokenKind::StringType,
                        "bool" => TokenKind::Bool,
                        _ => TokenKind::Identifier(ident),
                    }
                }
                _ => return Err(format!("Unexpected character '{}' at line {}:{}", c, line, column)),
            };

            Ok(Token { kind, line, column })
        } else {
            Ok(Token { kind: TokenKind::EOF, line, column })
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = tok.kind == TokenKind::EOF;
            tokens.push(tok);
            if is_eof { break; }
        }
        Ok(tokens)
    }
}
