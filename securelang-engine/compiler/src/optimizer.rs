use crate::ast::{Program, Statement, Expr, BinaryOperator};

pub struct Optimizer;

impl Optimizer {
    pub fn new() -> Self {
        Optimizer
    }

    pub fn optimize(&self, mut program: Program) -> Program {
        // Pass 1: Common Subexpression Elimination (CSE)
        program.statements = self.common_subexpression_elimination(program.statements);

        // Optimize all statements
        for stmt in &mut program.statements {
            self.optimize_stmt(stmt);
        }
        
        // Dead Code Elimination (DCE) Pass
        program.statements = self.dead_code_elimination(program.statements);
        
        program
    }

    fn common_subexpression_elimination(&self, statements: Vec<Statement>) -> Vec<Statement> {
        // Advanced mock: A real CSE pass would build a hashmap of pure expressions
        // and replace redundant computations with a cached variable.
        statements
    }

    fn dead_code_elimination(&self, statements: Vec<Statement>) -> Vec<Statement> {
        let mut optimized = Vec::new();
        let mut terminated = false;

        for stmt in statements {
            if terminated {
                // If we've already seen a return or break, everything after is dead code
                break;
            }

            match &stmt {
                Statement::ReturnStmt(_) => {
                    optimized.push(stmt);
                    terminated = true;
                }
                Statement::IfStmt { condition, then_branch, else_branch } => {
                    // Constant branch elimination
                    if let Expr::BoolLiteral(b) = condition {
                        if *b {
                            optimized.extend(self.dead_code_elimination(then_branch.clone()));
                        } else if let Some(el) = else_branch {
                            optimized.extend(self.dead_code_elimination(el.clone()));
                        }
                        continue;
                    }

                    // Otherwise keep the if statement
                    let mut new_then = self.dead_code_elimination(then_branch.clone());
                    let mut new_else = else_branch.clone().map(|el| self.dead_code_elimination(el));
                    optimized.push(Statement::IfStmt {
                        condition: condition.clone(),
                        then_branch: new_then,
                        else_branch: new_else,
                    });
                }
                _ => optimized.push(stmt),
            }
        }
        optimized
    }

    fn optimize_stmt(&self, stmt: &mut Statement) {
        match stmt {
            Statement::LetDecl { value, .. } | Statement::Assignment { value, .. } => {
                self.optimize_expr(value);
            }
            Statement::IfStmt { condition, then_branch, else_branch } => {
                self.optimize_expr(condition);
                for b_stmt in then_branch {
                    self.optimize_stmt(b_stmt);
                }
                if let Some(el) = else_branch {
                    for b_stmt in el {
                        self.optimize_stmt(b_stmt);
                    }
                }
            }
            Statement::WhileStmt { condition, body } => {
                self.optimize_expr(condition);
                for b_stmt in body {
                    self.optimize_stmt(b_stmt);
                }
            }
            Statement::ReturnStmt(Some(expr)) | Statement::ExpressionStmt(expr) => {
                self.optimize_expr(expr);
            }
            Statement::FunctionDecl { body, .. } => {
                for b_stmt in body {
                    self.optimize_stmt(b_stmt);
                }
            }
            _ => {}
        }
    }

    fn optimize_expr(&self, expr: &mut Expr) {
        match expr {
            Expr::BinaryOp(left, op, right) => {
                self.optimize_expr(left);
                self.optimize_expr(right);

                // Constant Folding
                if let (Expr::IntLiteral(l), Expr::IntLiteral(r)) = (&**left, &**right) {
                    match op {
                        BinaryOperator::Add => *expr = Expr::IntLiteral(*l + *r),
                        BinaryOperator::Sub => *expr = Expr::IntLiteral(*l - *r),
                        BinaryOperator::Mul => *expr = Expr::IntLiteral(*l * *r),
                        BinaryOperator::Div => if *r != 0 { *expr = Expr::IntLiteral(*l / *r) },
                        BinaryOperator::Eq => *expr = Expr::BoolLiteral(*l == *r),
                        BinaryOperator::Neq => *expr = Expr::BoolLiteral(*l != *r),
                        BinaryOperator::Lt => *expr = Expr::BoolLiteral(*l < *r),
                        BinaryOperator::Gt => *expr = Expr::BoolLiteral(*l > *r),
                        BinaryOperator::Lte => *expr = Expr::BoolLiteral(*l <= *r),
                        BinaryOperator::Gte => *expr = Expr::BoolLiteral(*l >= *r),
                    }
                    return;
                }

                // Strength Reduction (e.g. x * 2 -> x << 1, x / 2 -> x >> 1)
                // For simplicity, we just fold x * 0 -> 0, x * 1 -> x, x + 0 -> x
                if let Expr::IntLiteral(l) = &**left {
                    match (l, op) {
                        (0, BinaryOperator::Add) => *expr = *right.clone(),
                        (0, BinaryOperator::Mul) => *expr = Expr::IntLiteral(0),
                        (1, BinaryOperator::Mul) => *expr = *right.clone(),
                        _ => {}
                    }
                } else if let Expr::IntLiteral(r) = &**right {
                    match (r, op) {
                        (0, BinaryOperator::Add) | (0, BinaryOperator::Sub) => *expr = *left.clone(),
                        (0, BinaryOperator::Mul) => *expr = Expr::IntLiteral(0),
                        (1, BinaryOperator::Mul) | (1, BinaryOperator::Div) => *expr = *left.clone(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
