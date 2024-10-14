use super::lexer::*;
use super::grammatica::grammar::*;
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

pub fn get_language_rule(token: &TokenKind) -> Result<Vec<TokenKind>, Box<dyn std::error::Error>>{

	    
	return Ok(vec![]); //Dovra' ritornare l'insieme dei Token ammessi
}
