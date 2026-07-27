use compiler::lexer::{Lexer, Token};

#[test]
fn test_lexer_basic() {
    let source = "user login { username input }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0], Token::KeywordUser);
    assert_eq!(tokens[1], Token::KeywordLogin);
    assert_eq!(tokens[2], Token::LBrace);
    assert_eq!(tokens[3], Token::Identifier("username".to_string()));
    assert_eq!(tokens[4], Token::KeywordInput);
    assert_eq!(tokens[5], Token::RBrace);
    assert_eq!(tokens[6], Token::EOF);
}
