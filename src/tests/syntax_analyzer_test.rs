use crate::components::{lexer::*, syntax_analyzer::*};

#[test]
fn assignment_test(){
	let res = is_code_valid(&get_tokens("= ".to_string()));
	assert_eq!(res, true);
}
