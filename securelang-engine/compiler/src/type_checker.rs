use crate::ast::{Program, Statement, Expr, Type, BinaryOperator};
use std::collections::HashMap;

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            scopes: vec![HashMap::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &str, ty: Type) -> Result<(), String> {
        let current_scope = self.scopes.last_mut().ok_or("No active scope")?;
        if current_scope.contains_key(name) {
            return Err(format!("Variable '{}' already declared in this scope", name));
        }
        current_scope.insert(name.to_string(), ty);
        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }

    pub fn check(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::LetDecl { name, ty, value, .. } => {
                let val_ty = self.check_expr(value)?;
                if let Some(expected_ty) = ty {
                    if *expected_ty != val_ty {
                        return Err(format!("Type mismatch: Expected {:?}, got {:?}", expected_ty, val_ty));
                    }
                }
                self.declare(name, val_ty)?;
            }
            Statement::FunctionDecl { name, params, return_type, body } => {
                // To keep it simple, we don't fully support hoisting here yet.
                // A real compiler would register function signatures in a global pass first.
                // Let's just enter scope and declare params.
                self.enter_scope();
                for (param_name, param_ty) in params {
                    self.declare(param_name, param_ty.clone())?;
                }
                for b_stmt in body {
                    self.check_statement(b_stmt)?;
                }
                self.exit_scope();
            }
            Statement::IfStmt { condition, then_branch, else_branch } => {
                let cond_ty = self.check_expr(condition)?;
                if cond_ty != Type::Bool {
                    return Err("If condition must be a boolean".into());
                }
                self.enter_scope();
                for b_stmt in then_branch {
                    self.check_statement(b_stmt)?;
                }
                self.exit_scope();

                if let Some(el) = else_branch {
                    self.enter_scope();
                    for b_stmt in el {
                        self.check_statement(b_stmt)?;
                    }
                    self.exit_scope();
                }
            }
            Statement::WhileStmt { condition, body } => {
                let cond_ty = self.check_expr(condition)?;
                if cond_ty != Type::Bool {
                    return Err("While condition must be a boolean".into());
                }
                self.enter_scope();
                for b_stmt in body {
                    self.check_statement(b_stmt)?;
                }
                self.exit_scope();
            }
            Statement::ReturnStmt(expr_opt) => {
                if let Some(expr) = expr_opt {
                    self.check_expr(expr)?;
                }
            }
            Statement::ExpressionStmt(expr) => {
                self.check_expr(expr)?;
            }
            Statement::Assignment { name, value } => {
                let val_ty = self.check_expr(value)?;
                if let Some(expected_ty) = self.lookup(name) {
                    if *expected_ty != val_ty {
                        return Err(format!("Type mismatch in assignment to '{}': Expected {:?}, got {:?}", name, expected_ty, val_ty));
                    }
                } else {
                    return Err(format!("Undeclared variable '{}'", name));
                }
            }
            Statement::UserLoginBlock { statements } => {
                self.enter_scope();
                for b_stmt in statements {
                    self.check_statement(b_stmt)?;
                }
                self.exit_scope();
            }
            Statement::InputDecl(name) => {
                self.declare(name, Type::String)?;
            }
            Statement::SecureDecl(name) => {
                self.declare(name, Type::String)?; // Passwords stored as Secure Strings
            }
            Statement::AuthenticateStmt => {}
        }
        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::IntLiteral(_) => Ok(Type::Int),
            Expr::StringLiteral(_) => Ok(Type::String),
            Expr::BoolLiteral(_) => Ok(Type::Bool),
            Expr::Identifier(name) => {
                if let Some(ty) = self.lookup(name) {
                    Ok(ty.clone())
                } else {
                    Err(format!("Undeclared variable '{}'", name))
                }
            }
            Expr::BinaryOp(left, op, right) => {
                let l_ty = self.check_expr(left)?;
                let r_ty = self.check_expr(right)?;

                match op {
                    BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul | BinaryOperator::Div => {
                        if l_ty == Type::Int && r_ty == Type::Int {
                            Ok(Type::Int)
                        } else {
                            Err("Arithmetic operators require Int operands".into())
                        }
                    }
                    BinaryOperator::Lt | BinaryOperator::Gt | BinaryOperator::Lte | BinaryOperator::Gte => {
                        if l_ty == Type::Int && r_ty == Type::Int {
                            Ok(Type::Bool)
                        } else {
                            Err("Comparison operators require Int operands".into())
                        }
                    }
                    BinaryOperator::Eq | BinaryOperator::Neq => {
                        if l_ty == r_ty {
                            Ok(Type::Bool)
                        } else {
                            Err("Equality operators require operands of the same type".into())
                        }
                    }
                }
            }
            Expr::Call(_name, args) => {
                // Native function stubs checking would go here
                for arg in args {
                    self.check_expr(arg)?;
                }
                // Mock return type for all calls
                Ok(Type::Int)
            }
        }
    }
}
