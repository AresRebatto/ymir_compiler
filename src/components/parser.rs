use super::lexer::*;

pub struct ASTNode{
    pub token: Token,
    pub linked_token: Vec<Token>
}

impl ASTNode{
    pub fn new(tk: Token, ltk: Vec<Token>)-> Self{
        Self{
            token: tk,
            linked_token: ltk
        }
    }
}

pub fn get_ast() -> Vec<ASTNode>{
    let mut abs_tree: Vec<ASTNode> = vec![];



    return  abs_tree;
}