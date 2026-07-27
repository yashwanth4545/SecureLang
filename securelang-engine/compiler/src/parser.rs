use crate::token::{Token, TokenKind};
use crate::errors::{CompilerError, ErrorPhase};
use crate::ast::{Program, Statement, Expr, Type, BinaryOperator};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn consume(&mut self, expected: TokenKind, message: &str) -> Result<&Token, String> {
        if self.peek().kind == expected {
            Ok(self.advance())
        } else {
            Err(format!("{} (Found {:?} at line {})", message, self.peek().kind, self.peek().line))
        }
    }

    // Panic mode error recovery
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon { return; }
            match self.peek().kind {
                TokenKind::Class | TokenKind::Fn | TokenKind::Let | TokenKind::If | TokenKind::While | TokenKind::Return => return,
                _ => (),
            }
            self.advance();
        }
    }

    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.peek().kind == *kind {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_declaration()?);
        }
        Ok(Program { statements })
    }

    fn parse_declaration(&mut self) -> Result<Statement, String> {
        let is_async = self.match_token(&[TokenKind::Async]);
        
        if self.match_token(&[TokenKind::Fn]) {
            self.parse_function_decl(is_async)
        } else if self.match_token(&[TokenKind::Class]) {
            self.parse_class_decl()
        } else if self.match_token(&[TokenKind::Let]) {
            self.parse_let_decl()
        } else if self.match_token(&[TokenKind::User]) {
            self.parse_user_block()
        } else {
            self.parse_statement()
        }
    }

    fn parse_class_decl(&mut self) -> Result<Statement, String> {
        // Mock stub for class parsing to satisfy AST requirements
        let name = match &self.advance().kind {
            TokenKind::Identifier(id) => id.clone(),
            _ => return Err("Expected class name".into()),
        };
        self.consume(TokenKind::LBrace, "Expected '{' before class body")?;
        self.consume(TokenKind::RBrace, "Expected '}' after class body")?;
        Ok(Statement::ClassDecl { name, generics: vec![], fields: vec![], methods: vec![] })
    }

    fn parse_function_decl(&mut self, is_async: bool) -> Result<Statement, String> {
        let name = match &self.advance().kind {
            TokenKind::Identifier(id) => id.clone(),
            _ => return Err("Expected function name".into()),
        };

        self.consume(TokenKind::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                let param_name = match &self.advance().kind {
                    TokenKind::Identifier(id) => id.clone(),
                    _ => return Err("Expected parameter name".into()),
                };
                self.consume(TokenKind::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));

                if !self.match_token(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after parameters")?;

        let mut return_type = Type::Void;
        if self.match_token(&[TokenKind::Arrow]) {
            return_type = self.parse_type()?;
        }

        self.consume(TokenKind::LBrace, "Expected '{' before function body")?;
        let body = self.parse_block()?;

        Ok(Statement::FunctionDecl { name, is_async, params, return_type, body })
    }

    fn parse_let_decl(&mut self) -> Result<Statement, String> {
        let is_mut = self.match_token(&[TokenKind::Mut]);
        let name = match &self.advance().kind {
            TokenKind::Identifier(id) => id.clone(),
            _ => return Err("Expected variable name".into()),
        };

        let mut ty = None;
        if self.match_token(&[TokenKind::Colon]) {
            ty = Some(self.parse_type()?);
        }

        self.consume(TokenKind::Assign, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';' after let declaration")?;

        Ok(Statement::LetDecl { name, is_mut, ty, value })
    }

    fn parse_user_block(&mut self) -> Result<Statement, String> {
        self.consume(TokenKind::Login, "Expected 'login' after 'user'")?;
        self.consume(TokenKind::LBrace, "Expected '{' before user block")?;

        let mut statements = Vec::new();
        while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
            if self.match_token(&[TokenKind::Authenticate]) {
                statements.push(Statement::AuthenticateStmt);
            } else if let TokenKind::Identifier(id) = &self.peek().kind {
                let name = id.clone();
                self.advance();
                if self.match_token(&[TokenKind::Input]) {
                    statements.push(Statement::InputDecl(name));
                } else if self.match_token(&[TokenKind::Secure]) {
                    statements.push(Statement::SecureDecl(name));
                } else {
                    return Err("Expected 'input' or 'secure' after identifier in user block".into());
                }
            } else {
                return Err("Unexpected token in user block".into());
            }
        }
        self.consume(TokenKind::RBrace, "Expected '}' after user block")?;

        Ok(Statement::UserLoginBlock { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&[TokenKind::If]) {
            self.parse_if_stmt()
        } else if self.match_token(&[TokenKind::While]) {
            self.parse_while_stmt()
        } else if self.match_token(&[TokenKind::Match]) {
            self.parse_match_stmt()
        } else if self.match_token(&[TokenKind::Return]) {
            self.parse_return_stmt()
        } else {
            self.parse_expression_stmt()
        }
    }

    fn parse_match_stmt(&mut self) -> Result<Statement, String> {
        let value = self.parse_expression()?;
        self.consume(TokenKind::LBrace, "Expected '{' after match")?;
        self.consume(TokenKind::RBrace, "Expected '}' after match")?;
        Ok(Statement::MatchStmt { value, arms: vec![] })
    }

    fn parse_if_stmt(&mut self) -> Result<Statement, String> {
        let condition = self.parse_expression()?;
        self.consume(TokenKind::LBrace, "Expected '{' after if condition")?;
        let then_branch = self.parse_block()?;

        let mut else_branch = None;
        if self.match_token(&[TokenKind::Else]) {
            self.consume(TokenKind::LBrace, "Expected '{' after else")?;
            else_branch = Some(self.parse_block()?);
        }

        Ok(Statement::IfStmt { condition, then_branch, else_branch })
    }

    fn parse_while_stmt(&mut self) -> Result<Statement, String> {
        let condition = self.parse_expression()?;
        self.consume(TokenKind::LBrace, "Expected '{' after while condition")?;
        let body = self.parse_block()?;

        Ok(Statement::WhileStmt { condition, body })
    }

    fn parse_return_stmt(&mut self) -> Result<Statement, String> {
        let mut value = None;
        if self.peek().kind != TokenKind::Semicolon {
            value = Some(self.parse_expression()?);
        }
        self.consume(TokenKind::Semicolon, "Expected ';' after return")?;
        Ok(Statement::ReturnStmt(value))
    }

    fn parse_expression_stmt(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expression()?;
        
        if self.match_token(&[TokenKind::Assign]) {
            if let Expr::Identifier(name) = expr {
                let value = self.parse_expression()?;
                self.consume(TokenKind::Semicolon, "Expected ';' after assignment")?;
                return Ok(Statement::Assignment { name, value });
            } else {
                return Err("Invalid assignment target".into());
            }
        }

        self.consume(TokenKind::Semicolon, "Expected ';' after expression")?;
        Ok(Statement::ExpressionStmt(expr))
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
            statements.push(self.parse_declaration()?);
        }
        self.consume(TokenKind::RBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;

        while self.match_token(&[TokenKind::Eq, TokenKind::Neq]) {
            let operator = match self.previous().kind {
                TokenKind::Eq => BinaryOperator::Eq,
                TokenKind::Neq => BinaryOperator::Neq,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;

        while self.match_token(&[TokenKind::Lt, TokenKind::Gt, TokenKind::Lte, TokenKind::Gte]) {
            let operator = match self.previous().kind {
                TokenKind::Lt => BinaryOperator::Lt,
                TokenKind::Gt => BinaryOperator::Gt,
                TokenKind::Lte => BinaryOperator::Lte,
                TokenKind::Gte => BinaryOperator::Gte,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expr = Expr::BinaryOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;

        while self.match_token(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = match self.previous().kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        while self.match_token(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = match self.previous().kind {
                TokenKind::Star => BinaryOperator::Mul,
                TokenKind::Slash => BinaryOperator::Div,
                _ => unreachable!(),
            };
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenKind::True]) {
            return Ok(Expr::BoolLiteral(true));
        }
        if self.match_token(&[TokenKind::False]) {
            return Ok(Expr::BoolLiteral(false));
        }

        match &self.peek().kind {
            TokenKind::IntLiteral(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::IntLiteral(val))
            }
            TokenKind::StringLiteral(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::StringLiteral(val))
            }
            TokenKind::Identifier(id) => {
                let name = id.clone();
                self.advance();
                if self.match_token(&[TokenKind::LParen]) {
                    let mut args = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.match_token(&[TokenKind::Comma]) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenKind::RParen, "Expected ')' after arguments")?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(format!("Expected expression, found {:?}", self.peek().kind)),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if self.match_token(&[TokenKind::Int]) {
            Ok(Type::Int)
        } else if self.match_token(&[TokenKind::StringType]) {
            Ok(Type::String)
        } else if self.match_token(&[TokenKind::Bool]) {
            Ok(Type::Bool)
        } else {
            Err("Expected type".into())
        }
    }
}
