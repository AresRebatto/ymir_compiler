use super::lexer::*;
use std::fs;
pub struct ASTNode{
    pub token: Token,
    pub linked_token: Vec<Token>
}

impl ASTNode{
    pub fn new(tk: Token, ltk: Vec<Token>)-> Self{
        Self{
            token: tk,
            linked_token: ltk
        }
    }
}

pub fn get_ast() -> Vec<ASTNode>{
    let mut abs_tree: Vec<ASTNode> = vec![];
	
	get_language_grammar_rule(TokenKind::Plus);

    return  abs_tree;
}

fn get_language_grammar_rule(token: TokenKind) -> Result<(), Box<dyn std::error::Error>>{

	let file_contents = fs::read_to_string("grammar.yaml")?;
	    println!("info.txt content =\n{file_contents}");
	    
	return Ok(()); //Dovra' ritornare l'insieme dei Token ammessi
}
