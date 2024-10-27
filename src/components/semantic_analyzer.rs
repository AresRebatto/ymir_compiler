use super::{parser::*, lexer::*};


struct SemanticAnalyzer{
	pub symbol_table: Vec<SymbolTableRecord>,
	pub is_code_valid: bool
}

struct SymbolTableRecord{
	pub identifier: String,

	//We manage only integer 
	pub value: i32
}

impl SemanticAnalyzer{

	/// Return if source code is valid \
	/// param:
	/// ```
	/// tokens: Vector of tokens in source code
	///
	///SemaanticAnalyzer::is_code_valid();
	/// ```
	pub fn is_code_valid(tokens: &Vec<Token>)-> Self{

		let mut res: SemanticAnalyzer = SemanticAnalyzer{
			symbol_table: vec![], 
			is_code_valid: true
		};
	for (i, token) in tokens[..tokens.len()-1].iter().enumerate()
	    {
	    	let mut compare_value: TokenKind = match &token.next_token{
	    		Some(TokenKind::Int(i))=> TokenKind::Int(0), 
	    		Some(TokenKind::Identifier(i)) => TokenKind::Identifier("".to_string()),
	    		Some(value)=> value.clone(),
	    		_ => TokenKind::Identifier("null".to_string())
	    	};
	    	
	        if !get_ast(&
	        token.token)
	        	.unwrap()
	        	.contains(&compare_value)
	        	&& token.token != TokenKind::Semicolon{
	        	return Self{
					symbol_table: vec![],
					is_code_valid: false
	        	};
	        }

	        match &token.token{
	        	TokenKind::Identifier(value)=>{
	        		if token.next_token == Some(TokenKind::Equal){
	        			match &tokens[i+2].token{
	        				TokenKind::Identifier(id)=>{},
	        				TokenKind::Int(val) => {
	        					res.symbol_table
	        						.push(SymbolTableRecord::new(token.token.clone().unwrap_id(), val.clone()));
	        				},
	        				_ => panic!("Exit code 1: incorretc aassignment format")
	        			}
	        		}
	        	},
	        	_ => {}
	        }
	    }

    	return res;
	}
}


impl SymbolTableRecord{
	pub fn new(id: String, val: i32) -> Self{
		Self{
			identifier: id,
			value: val
		}
	}
}


