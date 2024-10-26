use crate::components::{parser::*, lexer::*};

#[test]
fn ast_test_int(){
    let ast: Vec<Vec<TokenKind>> = get_tokens("int".to_string())
              .iter()
              .map(|x|get_ast(&x.token).unwrap())
              .collect();

    assert_eq!(ast, [[
            TokenKind::Identifier("".to_string())
        ]]
    )
}

#[test]
fn ast_test_equal() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("=".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Identifier("".to_string()), 
            TokenKind::Int(0)
        ]
    ]);
}

#[test]
fn ast_test_minus() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("-".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Int(0), 
            TokenKind::Identifier("".to_string())
        ]
    ]);
}

#[test]
fn ast_test_plus() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("+".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Int(0), 
            TokenKind::Identifier("".to_string())
        ]
    ]);
}

#[test]
fn ast_test_semicolon() {
    let ast: Vec<Vec<TokenKind>> = get_tokens(";".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![] // Nessun token in uscita
    ]);
}

#[test]
fn ast_test_type() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("int".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Identifier("".to_string())
        ]
    ]);
}

#[test]
fn ast_test_identifier() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("variable".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Minus, 
            TokenKind::Plus, 
            TokenKind::Equal
        ]
    ]);
}

#[test]
fn ast_test_complex_string() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("int variable =".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Identifier("".to_string())
        ],
        vec![
            TokenKind::Minus, 
            TokenKind::Plus, 
            TokenKind::Equal
        ],
        vec![
            TokenKind::Identifier("".to_string()), 
            TokenKind::Int(0)
        ]

    ]);
}

#[test]
fn ast_test_int_variable() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("42 variable".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Identifier("".to_string()),
            TokenKind::Minus, 
            TokenKind::Plus
        ],
        vec![
            TokenKind::Minus, 
            TokenKind::Plus, 
            TokenKind::Equal
        ]
    ]);
}

#[test]
fn ast_test_variable_assignment() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("variable = 10".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Minus, 
            TokenKind::Plus, 
            TokenKind::Equal
        ],
        vec![
            TokenKind::Identifier("".to_string()), 
            TokenKind::Int(0)
        ],
        vec![
            TokenKind::Identifier("".to_string()), 
            TokenKind::Minus, 
            TokenKind::Plus
        ]
    ]);
}

#[test]
fn ast_test_int_plus_variable() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("int + variable".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Identifier("".to_string())
        ],
        vec![
            TokenKind::Int(0), 
            TokenKind::Identifier("".to_string())
        ],
        vec![
            TokenKind::Minus, 
            TokenKind::Plus, 
            TokenKind::Equal
        ]
    ]);
}

#[test]
fn ast_test_nested_expression() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("42 + 18 - variable".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Identifier("".to_string()), 
            TokenKind::Minus, 
            TokenKind::Plus
        ],
        vec![
            TokenKind::Int(0), 
            TokenKind::Identifier("".to_string())
        ],
        vec![
            TokenKind::Int(0), 
            TokenKind::Identifier("".to_string())
        ],
        vec![
            TokenKind::Minus, 
            TokenKind::Plus, 
            TokenKind::Equal
        ]
    ]);
}

#[test]
fn ast_test_semicolon_ending() {
    let ast: Vec<Vec<TokenKind>> = get_tokens("variable = 42 ;".to_string())
        .iter()
        .map(|x| get_ast(&x.token).unwrap())
        .collect();

    assert_eq!(ast, [
        vec![
            TokenKind::Minus, 
            TokenKind::Plus, 
            TokenKind::Equal
        ],
        vec![
            TokenKind::Identifier("".to_string()), 
            TokenKind::Int(0)
        ],
        vec![
            TokenKind::Identifier("".to_string()), 
            TokenKind::Minus, 
            TokenKind::Plus
        ],
        vec![]
    ]);
}
