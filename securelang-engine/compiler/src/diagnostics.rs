use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Diagnostic {
    pub fn new(severity: DiagnosticSeverity, message: String, file: String, line: usize, column: usize) -> Self {
        Diagnostic { severity, message, file, line, column }
    }
}

pub struct DiagnosticsEngine {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsEngine {
    pub fn new() -> Self {
        DiagnosticsEngine { diagnostics: Vec::new() }
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == DiagnosticSeverity::Error)
    }

    pub fn print_all(&self) {
        for diag in &self.diagnostics {
            let color_code = match diag.severity {
                DiagnosticSeverity::Error => "\x1b[31m", // Red
                DiagnosticSeverity::Warning => "\x1b[33m", // Yellow
                DiagnosticSeverity::Note => "\x1b[36m", // Cyan
            };
            let reset_code = "\x1b[0m";
            println!("{}{:?}: {}{}\n  --> {}:{}:{}", color_code, diag.severity, diag.message, reset_code, diag.file, diag.line, diag.column);
        }
    }
}
