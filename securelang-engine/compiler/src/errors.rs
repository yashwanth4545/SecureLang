#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub phase: ErrorPhase,
}

#[derive(Debug, Clone)]
pub enum ErrorPhase {
    Lexical,
    Parse,
    Semantic,
    TypeCheck,
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:?} Error] at line {}:{}: {}", self.phase, self.line, self.column, self.message)
    }
}
