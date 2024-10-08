pub enum TokenKind<T>{
	Int(i32),
	Equal,
	Minus,
	Plus,
	Semicolon,
	Preprocessor,
	Include,
	Define,
	FuncReturnType(T)
}

pub struct Token{
	token: TokenKind,
	next_token: TokenKind 	
}

impl Token{
	pub fn new(t: TokenKind, nt: TokenKind)-> Self{
		Self{
			token: t,
			next_token: tn 
		}
	}
}

pub fn get_tokens(source_code: String)-> Vec<Token>{
	let mut res: Vec<Token> = vec![];
}
