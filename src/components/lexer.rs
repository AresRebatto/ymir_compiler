pub enum TokenKind{
	Int(i32),
	Equal,
	Minus,
	Plus,
	Semicolon,
	Define,
	FuncReturnType(String),
	FuncName(String),
	LeftBracket,
	RightBracket,
	LeftParentheses,
	RightParentheses
}


pub struct Token{
	token: TokenKind,
	next_token: TokenKind 	
}

impl Token{
	pub fn new(t: TokenKind, nt: TokenKind)-> Self{
		Self{
			token: t,
			next_token: nt 
		}
	}
}

pub fn get_tokens(source_code: String)-> Vec<Token>{
	let mut res: Vec<Token> = vec![];

	return res;
}
