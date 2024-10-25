use super::lexer::*;


///Return the possible token kind tha could be after the token borrowed as
/// a parameter \
/// param:
/// ```
/// token: &TokenKind
/// ```
pub fn get_ast(token: &TokenKind) ->Result<Vec<TokenKind>, Box<dyn std::error::Error>>{
	match token{
		TokenKind::Int(_i) => Ok(vec![TokenKind::Identifier("".to_string()), TokenKind::Minus, TokenKind::Plus]),
		TokenKind::Equal => Ok(vec![TokenKind::Identifier("".to_string()), TokenKind::Int(0)]),
		TokenKind::Minus => Ok(vec![TokenKind::Int(0), TokenKind::Identifier("".to_string())]),
		TokenKind::Plus => Ok(vec![TokenKind::Int(0), TokenKind::Identifier("".to_string())]),
		TokenKind::Semicolon => Ok(vec![]),
		TokenKind::Type(_i) => Ok(vec![TokenKind::Identifier("".to_string())]),
		TokenKind::Identifier(_i) => Ok(vec![TokenKind::Minus, TokenKind::Plus, TokenKind::Equal]) ,
	}
}


