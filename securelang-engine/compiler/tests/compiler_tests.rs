use compiler;

#[test]
fn test_valid_program_compiles() {
    let source = "
        let x = 10;
        let y = 20;
        let mut z = x + y;
    ";
    
    let result = compiler::compile(source);
    assert!(result.is_ok(), "Valid program failed to compile: {:?}", result.err());
}

#[test]
fn test_semantic_re_declaration_error() {
    let source = "
        let x = 10;
        let x = 20;
    ";
    
    let result = compiler::compile(source);
    // Since compilation will fail at diagnostics, we expect an Err.
    assert!(result.is_err(), "Expected re-declaration error, but compilation succeeded");
}

#[test]
fn test_semantic_undeclared_variable_error() {
    let source = "
        let y = x + 10;
    ";
    
    let result = compiler::compile(source);
    assert!(result.is_err(), "Expected undeclared variable error, but compilation succeeded");
}

#[test]
fn test_securelang_dsl_parsing() {
    let source = "
        user login {
            input username;
            secure password;
            authenticate;
        }
    ";
    
    let result = compiler::compile(source);
    assert!(result.is_ok(), "Valid DSL block failed to compile: {:?}", result.err());
}
