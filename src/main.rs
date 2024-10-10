mod components;

use components::{lexer::*, parser};
use std::env;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() == 2 {
        for file_path in args.iter().skip(1) {
			
        	parser::get_ast();
            let tokens = get_tokens(String::from("Ciao a tutti quanti int ;"));
            for t in tokens{
                println!("{:?}", t.token);
            }
        }
    }else{
        println!("Numero di parametri non valido");
    }

    
}
