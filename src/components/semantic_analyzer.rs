use super::{parser::*, lexer::*};

#[derive(Debug, PartialEq)]
pub struct SemanticAnalyzer{
	pub symbol_table: Vec<SymbolTableRecord>,
	pub is_code_valid: bool
}

#[derive(Debug, PartialEq)]
pub struct SymbolTableRecord{
	pub identifier: String,

	//We manage only integer 
	pub value: i32
}

impl SemanticAnalyzer{

	/// Return if source code is valid an
	/// the Symbol Table\
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
	    	let compare_value: TokenKind = match &token.next_token{
	    		Some(TokenKind::Int(_i))=> TokenKind::Int(0), 
	    		Some(TokenKind::Identifier(_i)) => TokenKind::Identifier("".to_string()),
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
	        	TokenKind::Identifier(_value)=>{

	        		if token.next_token == Some(TokenKind::Equal){

	        			match &tokens[i+2].token{

	        				TokenKind::Identifier(_id)=>{

								let identifier = _id;

								match &tokens[i+3].token {

									TokenKind::Semicolon =>{

										let index = res.symbol_table
															  .iter()
															  .position(|value| &value.identifier == identifier)
															  .unwrap_or(panic!("{} doesn't exist", identifier));
										
										res.symbol_table.push(
											SymbolTableRecord::new(
												token.token.clone().unwrap_id(), 
												res.symbol_table[index].value
											)
										);

									},

									TokenKind::Plus =>{},

									TokenKind::Minus => {},

									_ => panic!("incorrect assignment format")
								}
							},
	        				TokenKind::Int(val) => {

	        					res.symbol_table
	        						.push(SymbolTableRecord::new(token.token.clone().unwrap_id(), val.clone()));
	        				
							},
	        				_ => panic!("incorretc assignment format")
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


