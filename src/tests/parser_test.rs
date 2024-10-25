use crate::components::{parser::*, lexer::*};

#[test]
fn ast_test_i(){
    let ast: Vec<TokenKind> = get_tokens("int".to_string())
              .iter()
              .map(|x|get_ast(&x.token).unwrap())
              .flatten()
              .collect();

    assert_eq!(ast, [
            TokenKind::Identifier("".to_string()), 
            TokenKind::Minus, TokenKind::Plus
        ]
    )
}