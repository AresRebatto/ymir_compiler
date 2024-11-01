use std::iter::Iterator;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind{
	Int(i32),
	Equal,
	Minus,
	Plus,
	Semicolon,
	Type(String), //Solo int
	Identifier(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token{
	pub token: TokenKind,
	pub next_token: Option<TokenKind> 	
}

impl Token{
	pub fn new(t: TokenKind, nt: Option<TokenKind>)-> Self{
		Self{
			token: t,
			next_token: nt 
		}
	}
}

///Return the tokens based on the code source passed as a String \
/// param:
/// ```
/// source_code: String
/// ```
pub fn get_tokens(source_code: String)-> Vec<Token>{
	if source_code.is_empty(){
		return vec![];
	}
	
	let mut res: Vec<Token> = vec![];
	let mut counter: usize = 0;
	for _c in source_code.split(' ').into_iter(){
		if counter == 0
		{
			res.push(Token::new(get_token(_c).unwrap(), None));
		}else{
			res.push(Token::new(get_token(_c).unwrap(), None));
			res[counter-1].next_token = Some(res[counter].clone().token);
		}
		counter+=1;
	}
	return res;
}

fn get_token(word: &str)-> Option<TokenKind>{
	match word{
		"=" => Some(TokenKind::Equal),
		"-" => Some(TokenKind::Minus),
		"+" => Some(TokenKind::Plus),
		";" => Some(TokenKind::Semicolon),
		"int" => Some(TokenKind::Type(String::from("int"))),
		_ => {
			if let Ok(i) = word.parse::<i32>() {
                Some(TokenKind::Int(i))
            } else {
                Some(TokenKind::Identifier(String::from(word)))
            }
		}
	}
}

impl TokenKind{
	pub fn unwrap_id(&self)-> String{
		match self{
			TokenKind::Identifier(val)=> val.clone(),
			_ => panic!("impossible to implicit convert")
		}
	}
}
