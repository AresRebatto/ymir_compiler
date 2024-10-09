pub enum TokenKind{
	Int(i32),
	Equal,
	Minus,
	Plus,
	Semicolon,
	Define,
	Type(String), //Solo int
	FuncName(String),
	LeftBracket,
	RightBracket,
	LeftParentheses,
	RightParentheses,
	Identifier
}


pub struct Token{
	pub token: TokenKind,
	pub next_token: TokenKind 	
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
	let res: Vec<Token> = vec![];
	for _c in source_code.split(' ').into_iter(){
		// match _c{
		// 	"int"=> res.push(TokenKind::Type(String::from("int"))), 


		// }
	}
	return res;
}
