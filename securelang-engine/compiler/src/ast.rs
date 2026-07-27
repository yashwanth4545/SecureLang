#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    String,
    Bool,
    Void,
    Custom(String, Vec<Type>), // For generics e.g., List<Int>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    IntLiteral(i64),
    StringLiteral(String),
    BoolLiteral(bool),
    Identifier(String),
    BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
    Call(String, Vec<Expr>),
    Await(Box<Expr>),
    PropertyAccess(Box<Expr>, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div,
    Eq, Neq, Lt, Gt, Lte, Gte,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetDecl { name: String, is_mut: bool, ty: Option<Type>, value: Expr },
    FunctionDecl { name: String, is_async: bool, params: Vec<(String, Type)>, return_type: Type, body: Vec<Statement> },
    ClassDecl { name: String, generics: Vec<String>, fields: Vec<(String, Type)>, methods: Vec<Statement> },
    InterfaceDecl { name: String, generics: Vec<String>, methods: Vec<(String, Vec<Type>, Type)> },
    TraitDecl { name: String, methods: Vec<(String, Vec<Type>, Type)> },
    IfStmt { condition: Expr, then_branch: Vec<Statement>, else_branch: Option<Vec<Statement>> },
    WhileStmt { condition: Expr, body: Vec<Statement> },
    MatchStmt { value: Expr, arms: Vec<(Expr, Vec<Statement>)> },
    ReturnStmt(Option<Expr>),
    ExpressionStmt(Expr),
    Assignment { name: String, value: Expr },
    
    // Exception Handling
    TryCatchStmt {
        try_body: Vec<Statement>,
        catch_var: String,
        catch_body: Vec<Statement>,
    },

    // SecureLang DSL
    UserLoginBlock { statements: Vec<Statement> },
    InputDecl(String),
    SecureDecl(String),
    AuthenticateStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
