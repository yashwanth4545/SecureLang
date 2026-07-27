use crate::ast::{Program, Statement, Expr, Type};
use crate::diagnostics::{DiagnosticsEngine, Diagnostic, DiagnosticSeverity};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: Type,
    pub is_mut: bool,
    pub is_initialized: bool,
}

pub struct SemanticAnalyzer<'a> {
    scopes: Vec<HashMap<String, Symbol>>,
    traits: HashMap<String, HashSet<String>>,
    imports: HashSet<String>,
    diagnostics: &'a mut DiagnosticsEngine,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticsEngine) -> Self {
        SemanticAnalyzer {
            scopes: vec![HashMap::new()],
            traits: HashMap::new(),
            imports: HashSet::new(),
            diagnostics,
        }
    }

    pub fn analyze(&mut self, program: &Program) {
        // Pass 1: Resolve imports and trait scaffolding
        for stmt in &program.statements {
            self.resolve_imports(stmt);
        }

        // Pass 2: Type hoisting and full scope resolution
        for stmt in &program.statements {
            self.visit_stmt(stmt);
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &str, ty: Type, is_mut: bool) {
        let current_scope = self.scopes.last_mut().expect("No active scope");
        if current_scope.contains_key(name) {
            self.diagnostics.emit(Diagnostic::new(
                DiagnosticSeverity::Error,
                format!("Variable '{}' is already declared in this scope", name),
                "unknown".to_string(),
                0, 0, // Should be passed from AST nodes when available
            ));
        } else {
            current_scope.insert(name.to_string(), Symbol { ty, is_mut, is_initialized: true });
        }
    }

    fn resolve(&mut self, name: &str) -> Option<Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol.clone());
            }
        }
        self.diagnostics.emit(Diagnostic::new(
            DiagnosticSeverity::Error,
            format!("Use of undeclared variable '{}'", name),
            "unknown".to_string(),
            0, 0,
        ));
        None
    }

    fn resolve_imports(&mut self, _stmt: &Statement) {
        // Stub for `import` handling
    }

    fn visit_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::FunctionDecl { name, params, body, return_type, .. } => {
                self.declare(name, return_type.clone(), false);
                self.enter_scope();
                for (p_name, p_ty) in params {
                    self.declare(p_name, p_ty.clone(), false);
                }
                for b_stmt in body {
                    self.visit_stmt(b_stmt);
                }
                self.exit_scope();
            }
            Statement::LetDecl { name, ty, is_mut, value } => {
                let inferred_type = self.infer_expr_type(value);
                let resolved_ty = ty.clone().unwrap_or(inferred_type);
                self.declare(name, resolved_ty, *is_mut);
            }
            Statement::Assignment { name, value } => {
                self.visit_expr(value);
                if let Some(symbol) = self.resolve(name) {
                    if !symbol.is_mut {
                        self.diagnostics.emit(Diagnostic::new(
                            DiagnosticSeverity::Error,
                            format!("Cannot reassign immutable variable '{}'", name),
                            "unknown".to_string(),
                            0, 0,
                        ));
                    }
                }
            }
            Statement::IfStmt { condition, then_branch, else_branch } => {
                self.visit_expr(condition);
                self.enter_scope();
                for b_stmt in then_branch {
                    self.visit_stmt(b_stmt);
                }
                self.exit_scope();
                if let Some(el) = else_branch {
                    self.enter_scope();
                    for b_stmt in el {
                        self.visit_stmt(b_stmt);
                    }
                    self.exit_scope();
                }
            }
            Statement::WhileStmt { condition, body } => {
                self.visit_expr(condition);
                self.enter_scope();
                for b_stmt in body {
                    self.visit_stmt(b_stmt);
                }
                self.exit_scope();
            }
            Statement::ExpressionStmt(expr) | Statement::ReturnStmt(Some(expr)) => {
                self.visit_expr(expr);
            }
            Statement::TryCatchStmt { try_body, catch_var, catch_body } => {
                self.enter_scope();
                for b_stmt in try_body {
                    self.visit_stmt(b_stmt);
                }
                self.exit_scope();

                self.enter_scope();
                self.declare(catch_var, Type::String, false); // Exception type mock
                for b_stmt in catch_body {
                    self.visit_stmt(b_stmt);
                }
                self.exit_scope();
            }
            _ => {}
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        let _ = self.infer_expr_type(expr);
    }

    fn infer_expr_type(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::IntLiteral(_) => Type::Int,
            Expr::StringLiteral(_) => Type::String,
            Expr::BoolLiteral(_) => Type::Bool,
            Expr::Identifier(name) => {
                if let Some(symbol) = self.resolve(name) {
                    symbol.ty
                } else {
                    Type::Void
                }
            }
            Expr::BinaryOp(left, _, right) => {
                self.infer_expr_type(left);
                self.infer_expr_type(right);
                Type::Int // Simplified binary op inference
            }
            Expr::Call(name, args) => {
                self.resolve(name);
                for arg in args {
                    self.infer_expr_type(arg);
                }
                Type::Void // Function return types would be retrieved from symbol table
            }
            _ => Type::Void,
        }
    }
}
