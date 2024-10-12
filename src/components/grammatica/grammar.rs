use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Grammar{
	pub program: String,
	pub instruction: String,
	pub declaration: String,
	pub assignment: String,
	pub expression: String 
}
