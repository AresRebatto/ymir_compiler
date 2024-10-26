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

#[test]
fn get_tokens_int_plus_equal() {
	let res = get_tokens(String::from("int + = +="));
	assert_eq!(res, [
		Token{
			token: TokenKind::Type(String::from("int")),
			next_token: Some(TokenKind::Plus)
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

#[test]
fn get_tokens_int_equal_42() {
	let res = get_tokens(String::from("int = 42"));
	assert_eq!(res, [
		Token{
			token: TokenKind::Type(String::from("int")),
			next_token: Some(TokenKind::Equal)
		},
		Token{
			token: TokenKind::Equal,
			next_token: Some(TokenKind::Int(42))
		},
		Token{
			token: TokenKind::Int(42),
			next_token: None
		}
	]);
}

#[test]
fn get_tokens_identifier_plus_minus() {
	let res = get_tokens(String::from("x + -"));
	assert_eq!(res, [
		Token{
			token: TokenKind::Identifier(String::from("x")),
			next_token: Some(TokenKind::Plus)
		},
		Token{
			token: TokenKind::Plus,
			next_token: Some(TokenKind::Minus)
		},
		Token{
			token: TokenKind::Minus,
			next_token: None
		}
	]);
}

#[test]
fn get_tokens_42_plus_x() {
	let res = get_tokens(String::from("42 + x"));
	assert_eq!(res, [
		Token{
			token: TokenKind::Int(42),
			next_token: Some(TokenKind::Plus)
		},
		Token{
			token: TokenKind::Plus,
			next_token: Some(TokenKind::Identifier(String::from("x")))
		},
		Token{
			token: TokenKind::Identifier(String::from("x")),
			next_token: None
		}
	]);
}

#[test]
fn get_tokens_multiple_statements() {
	let res = get_tokens(String::from("int x = 42 ; x + 1"));
	assert_eq!(res, [
		Token{
			token: TokenKind::Type(String::from("int")),
			next_token: Some(TokenKind::Identifier(String::from("x")))
		},
		Token{
			token: TokenKind::Identifier(String::from("x")),
			next_token: Some(TokenKind::Equal)
		},
		Token{
			token: TokenKind::Equal,
			next_token: Some(TokenKind::Int(42))
		},
		Token{
			token: TokenKind::Int(42),
			next_token: Some(TokenKind::Semicolon)
		},
		Token{
			token: TokenKind::Semicolon,
			next_token: Some(TokenKind::Identifier(String::from("x")))
		},
		Token{
			token: TokenKind::Identifier(String::from("x")),
			next_token: Some(TokenKind::Plus)
		},
		Token{
			token: TokenKind::Plus,
			next_token: Some(TokenKind::Int(1))
		},
		Token{
			token: TokenKind::Int(1),
			next_token: None
		}
	]);
}

#[test]
fn get_tokens_empty_string() {
	let res = get_tokens(String::from(""));
	assert_eq!(res, []);
}
