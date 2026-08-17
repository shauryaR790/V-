use vpp::lexer::Lexer;
use vpp::lexer::TokenKind;

#[test]
fn lexes_int_literal() {
    let tokens = Lexer::new("42").tokenize().unwrap();
    assert!(matches!(tokens[0].kind, TokenKind::IntLit(42)));
}

#[test]
fn lexes_string_literal() {
    let tokens = Lexer::new("\"hello\"").tokenize().unwrap();
    assert!(matches!(tokens[0].kind, TokenKind::StringLit(_)));
}

#[test]
fn lexes_operators() {
    let tokens = Lexer::new("+ - * / == != && || ..").tokenize().unwrap();
    assert!(matches!(tokens[0].kind, TokenKind::Plus));
    assert!(matches!(tokens[8].kind, TokenKind::DotDot));
}
