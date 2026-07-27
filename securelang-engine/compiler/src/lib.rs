pub mod errors;
pub mod diagnostics;
pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod resolver;
pub mod semantic;
pub mod type_checker;
pub mod optimizer;
pub mod ir;
pub mod cfg;
pub mod bytecode;
pub mod codegen;

pub fn compile(source: &str) -> Result<bytecode::Chunk, String> {
    let mut diagnostics_engine = diagnostics::DiagnosticsEngine::new();
    
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse()?;
    
    let mut resolver = resolver::ModuleResolver::new();
    resolver.resolve_imports(&program)?;
    
    let mut semantic_analyzer = semantic::SemanticAnalyzer::new(&mut diagnostics_engine);
    semantic_analyzer.analyze(&program);
    
    let mut checker = type_checker::TypeChecker::new();
    checker.check(&program)?;
    
    let optimizer = optimizer::Optimizer::new();
    let optimized_program = optimizer.optimize(program);
    
    let mut ir_gen = ir::IRGenerator::new();
    ir_gen.generate_from_ast(&optimized_program)?;
    
    let mut chunk = bytecode::Chunk::new();
    if let Some(main_func) = ir_gen.functions.first() {
        let _cfg = cfg::ControlFlowGraph::build(&main_func.instructions);
        
        let mut codegen = codegen::CodeGenerator::new();
        chunk = codegen.generate_from_ir(main_func);
    }
    
    if diagnostics_engine.has_errors() {
        diagnostics_engine.print_all();
        return Err("Compilation failed with diagnostics errors".to_string());
    }
    
    Ok(chunk)
}
