use crate::components::{lexer::*, parser::*};

//Serve per contenere le singole regole della
//grammatica del linguaggio
#[derive(Clone, Debug)]
pub enum GrammarRule{
    Raw(String),
    //Non posso lavorare con vec, ma serve una
    //struttura che dia la possibilita' di legare
    //tra loro piu' elementi
    Worked(GrammarRuleSection),

}

impl GrammarRule{
    pub fn new(raw_rule: String) -> Self{
        Self::Raw(raw_rule)
    }

    //Quando le regole vengono scaricate, sono in
    //un formato stringa. Questa funzione le deserializza
    pub fn deserialize(&self){
        let binding = self.unwrap_raw_rules();
        let rules = binding.as_str()
                           .split('/');
                        
        for rule in rules{
            if rule.to_string().contains('|'){

            }else{

            }
        }
    }

    pub fn unwrap(&self) -> GrammarRuleSection{
        match self{
            GrammarRule::Raw(_)=> panic!("You can't unwrap the rules before you've deseriaalized them"),
            GrammarRule::Worked(value)=> value.clone()

        }
    }

    pub fn unwrap_raw_rules(&self) -> String{
        match self{
            GrammarRule::Raw(raw_rule)=> raw_rule.clone(),
            _ => panic!("You can't unwrap non-raw rules with this method. Try the unwrap() method")
        }
    }
}

#[derive(Clone, Debug)]
enum GrammarRuleSection{
	SingleToken(TokenKind),
	MultipleTokens(Vec<TokenKind>)
}

impl GrammarRuleSection{

    pub fn unwrap_single_token(&self) -> TokenKind{
        match self{
            GrammarRuleSection::SingleToken(token) => token.clone(),
            _ => panic!("You can't unwrap non-single token rules section with this method. Try the unwrap_multiple_tokens() method")
        }
    }

    pub fn unwrap_multiple_tokens(&self) -> Vec<TokenKind>{
        match self{
            GrammarRuleSection::MultipleTokens(tokens) => tokens.clone(),
            _ => panic!("You can't unwrap non-multiple tokens rules section with this method. Try the unwrap_single_token() method")
        }
    }
}
