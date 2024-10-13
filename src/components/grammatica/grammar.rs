use serde::Deserialize;

use crate::TokenKind;

#[derive(Deserialize, Debug)]
pub struct Grammar{
	pub program: Vec<RuleFormat>,
	pub instruction: Vec<RuleFormat>,
	pub declaration: Vec<RuleFormat>,
	pub assignment: Vec<RuleFormat>,
	pub expression: Vec<RuleFormat> 
}



impl Grammar{
	// pub fn new() -> Self{
	// }
}

pub enum RuleFormat{
	SingleToken(TokenKind),
	PultipleTokens(Vec<TokenKind>),
	GrammarRule(Grammar)
}

