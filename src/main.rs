mod components;

use components::{lexer::*, parser};

fn main() {
    get_tokens(String::from("Ciao a tutti quanti"));
}
