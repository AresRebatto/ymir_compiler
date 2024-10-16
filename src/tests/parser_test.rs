use crate::components::{parser::*, lexer::*};

#[test]
fn ast_test_i(){
    let ast = get_ast(get_tokens("int".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Type("int".to_string()), next_token: None },
            linked_token: vec![TokenKind::Identifier("".to_string())]
        }
    ])
}

#[test]
fn ast_test_int() {
    let ast = get_ast(get_tokens("int".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Type("int".to_string()), next_token: None
            },
            linked_token: vec![TokenKind::Identifier("".to_string())]
        }
    ]);
}

#[test]
fn ast_test_int_value() {
    let ast = get_ast(get_tokens("42".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Int(42), next_token: None
            },
            linked_token: vec![TokenKind::Identifier("".to_string()), TokenKind::Minus, TokenKind::Plus]
        }
    ]);
}

#[test]
fn ast_test_equal() {
    let ast = get_ast(get_tokens("=".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Equal, next_token: None
            },
            linked_token: vec![TokenKind::Identifier("".to_string()), TokenKind::Int(0)]
        }
    ]);
}

#[test]
fn ast_test_minus() {
    let ast = get_ast(get_tokens("-".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Minus, next_token: None
            },
            linked_token: vec![TokenKind::Int(0), TokenKind::Identifier("".to_string())]
        }
    ]);
}

#[test]
fn ast_test_plus() {
    let ast = get_ast(get_tokens("+".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Plus, next_token: None
            },
            linked_token: vec![TokenKind::Int(0), TokenKind::Identifier("".to_string())]
        }
    ]);
}

#[test]
fn ast_test_identifier() {
    let ast = get_ast(get_tokens("x".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Identifier("x".to_string()), next_token: None
            },
            linked_token: vec![TokenKind::Minus, TokenKind::Plus, TokenKind::Equal]
        }
    ]);
}

#[test]
fn ast_test_semicolon() {
    let ast = get_ast(get_tokens(";".to_string()));

    assert_eq!(ast, [
        ASTNode{
            token: Token{
                token: TokenKind::Semicolon, next_token: None
            },
            linked_token: vec![]
        }
    ]);
}







