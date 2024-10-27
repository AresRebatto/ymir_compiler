use super::{parser::*, lexer::*};
/// Return if source code is valid \
/// param:
/// ```
/// tokens: Vector of tokens in source code
/// ast: The Abstract tree of the code source
/// ```
pub fn is_code_valid(tokens: &Vec<Token>) -> bool{

	//Cose da fare:
	//1. Deve verificiare se quando c'e' la semicolonna, il codice e'
	//	valido fino a quel punto
	//
	//2. L'AST ritorna un vettore con valori come TokenKind::Int(0),
	//	mentre noi abbiamo un vero e proprio intero, va quindi omessa
	//	la parte del valore interno e poi va fatto un tentativo di
	//	caasting del valore intero
    for token in &tokens[..tokens.len()-1]
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
        	return false;
        }
    }
    return true;

}


struct SAResult{
	pub symbol_table: Option<SymbolTableRecord>,
	pub is_code_valid: bool
}

struct SymbolTableRecord{
	pub identifier: String,

	//Per un linguaggio completo dovrebbe essere una 
	//generics, ma noi gestiamo solo interi 
	pub value: i32
}

