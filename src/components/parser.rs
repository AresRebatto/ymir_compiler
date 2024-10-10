use super::lexer::*;
use super::grammar::Grammar;
use serde::Deserialize;
use std::{fs::File, io::Read};


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
	
	get_language_grammar_rule(&TokenKind::Plus);

    return  abs_tree;
}

pub fn get_language_grammar_rule(token: &TokenKind) -> Result<(), Box<dyn std::error::Error>>{

	let mut file = File::open("src/components/grammar.yaml")?;
	let mut file_content = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file.");

	let grammar: Grammar = serde_yaml::from_str(&file_content).expect("Failed to parse YAML.");

	println!("{:?}", grammar);	    
	return Ok(()); //Dovra' ritornare l'insieme dei Token ammessi
}
