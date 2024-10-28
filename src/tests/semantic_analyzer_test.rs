use crate::components::{lexer::*, semantic_analyzer::*};

#[test]
fn assignment_test(){
	let res = SemanticAnalyzer::is_code_valid(&get_tokens("int a = 4 ;".to_string()));
	assert_eq!(res, SemanticAnalyzer{
		symbol_table: vec![
			SymbolTableRecord{
				identifier: "a".to_string(),
				value: 4
			}
		],
		is_code_valid: true
	})
}

#[test]
#[should_panic]
fn not_managed_type_assignment(){
	let res = SemanticAnalyzer::is_code_valid(&get_tokens("int a = 4.4 ;".to_string()));
}

#[test]
#[should_panic]
fn not_existed_var_assignment(){
	let res = SemanticAnalyzer::is_code_valid(&get_tokens("int a = b ;".to_string()));
}

#[test]
fn sum_assignment_test(){
	let res = SemanticAnalyzer::is_code_valid(&get_tokens("int a = 4 + a ;".to_string()));
}
