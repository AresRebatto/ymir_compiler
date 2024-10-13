use crate::components::lexer::*;
						//Type(String), Identifier(String), Equal
pub struct Declaration(TokenKind, TokenKind, TokenKind, Value);

					//Identifier(String), Equal
pub struct Assignment(TokenKind, TokenKind, Value);

pub enum Value{
	Integer(TokenKind), //Int(i32)
	Id(TokenKind), //Identifier(String)
	Exp(Box<Expression>)
}

pub struct Expression{
	op1: Box<Value>,
	operator: Operator,
	op2: Box<Value>
}

pub enum Operator{
	Add(TokenKind), //Plus
	Sub(TokenKind) //Minus
}

pub fn declaration(_type: TokenKind, id: TokenKind, value: Value)-> Declaration{
	Declaration(_type, id, TokenKind::Equal, value)
}

