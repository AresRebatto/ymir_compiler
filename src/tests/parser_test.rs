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





