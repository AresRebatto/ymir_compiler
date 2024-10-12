use crate::components::{lexer::*, parser::*};

//Serve per contenere le singole regole della
//grammatica del linguaggio
#[derive(Clone, Debug)]
pub enum GrammarRule{
    Raw(String),
    Worked(Vec<GrammarRuleSec>)
}

impl GrammarRule{
    pub fn new(raw_rule: String) -> Self{
        Self::Raw(raw_rule)
    }

    //Quando le regole vengono scaricate, sono in
    //un formato stringa. Questa funzione le deserializza
    pub fn deserialize(&self){

    }

    pub fn unwrap(&self) -> Vec<GrammarRuleSec>{
        match self{
            GrammarRule::Raw(_)=> panic!("Error: before extracting the rules, 
                                        it is necessary to deserialise them"),
            GrammarRule::Worked(value)=> value.clone()

        }
    }
}

//Probabilmente da inserire un enum grammar section che permetta
//Di inserire o una componente specifico o piu' possibili componenti
#[derive(Clone, Debug)]
pub enum GrammarRuleSec{
    SingleRule(TokenKind),
    MultipleChoise(Vec<TokenKind>)
}

impl GrammarRuleSec{
    pub fn unwrap(&self){

    }
}



