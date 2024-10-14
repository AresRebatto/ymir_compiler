use super::lexer::*;
use serde::Deserialize;
use std::{fs::File, io::Read};


pub struct ASTNode{
    pub token: Token,
    pub linked_token: Vec<TokenKind>
}

impl ASTNode{
    pub fn new(tk: Token, ltk: Vec<TokenKind>)-> Self{
        Self{
            token: tk,
            linked_token: ltk
        }
    }
}

pub fn get_ast(read_tokens: Vec<Token>) -> Vec<ASTNode>{
    let mut abs_tree: Vec<ASTNode> = vec![];
	
	for read_token in read_tokens{
		abs_tree.push(ASTNode::new(
					  read_token.clone(),
					  get_language_rule(&read_token.token).unwrap())
					  );
	}

    return  abs_tree;
}

pub fn get_language_rule(token: &TokenKind) ->Result<Vec<TokenKind>, Box<dyn std::error::Error>>{
	match token{
		TokenKind::Int(i) => Ok(vec![TokenKind::Identifier("".to_string()), TokenKind::Minus, TokenKind::Plus]),
		TokenKind::Equal => Ok(vec![TokenKind::Identifier("".to_string()), TokenKind::Int(0)]),
		TokenKind::Minus => Ok(vec![TokenKind::Int(0), TokenKind::Identifier("".to_string())]),
		TokenKind::Plus => Ok(vec![TokenKind::Int(0), TokenKind::Identifier("".to_string())]),
		TokenKind::Semicolon => Ok(vec![]),
		TokenKind::Type(i) => Ok(vec![TokenKind::Identifier("".to_string())]),
		TokenKind::Identifier(i) => Ok(vec![TokenKind::Minus, TokenKind::Plus, TokenKind::Equal]) ,
	}
}
