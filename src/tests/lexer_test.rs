use crate::components::lexer::*;

#[test]
fn get_tokens_i(){
	let res = get_tokens(String::from("int + = +="));
	assert_eq!(res, [
		Token{
			token: TokenKind::Type(String::from("int")),
			next_token:Some(TokenKind::Plus)
		},
		Token{
			token: TokenKind::Plus,
			next_token: Some(TokenKind::Equal)
		},
		Token{
			token: TokenKind::Equal,
			next_token: Some(TokenKind::Identifier(String::from("+=")))
		},
		Token{
			token: TokenKind::Identifier(String::from("+=")), 
			next_token: None
		}
	]);
}
