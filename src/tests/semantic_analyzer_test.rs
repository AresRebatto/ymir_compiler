use crate::components::{lexer::*, semantic_analyzer::*};

#[test]
fn assignment_test(){
	let res = is_code_valid(&get_tokens("int a = 4 ;".to_string()));
	assert_eq!(res, true);
}

#[test]
fn sum_assignment_test(){
	let res = is_code_valid(&get_tokens("int a = 4 + a ;".to_string()));
	assert_eq!(res, true);
}
